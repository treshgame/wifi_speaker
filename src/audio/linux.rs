use std::{cell::RefCell, rc::Rc, thread, time::Duration};

use libpulse_binding::{context::{Context, FlagSet, State}, error::PAErr, mainloop::standard::{IterateResult, Mainloop}, proplist::Proplist, sample::{Format, Spec}, stream::{Direction}};
use libpulse_simple_binding::Simple;
use rtrb::{Consumer, Producer};

use crate::{audio::{AudioParams, AudioStream, SpeakerDirection}, core::AUDIO_BUFFER_LENGTH};

pub struct LinuxStream {
    stream: Simple,
    params: AudioParams
}

impl AudioStream for LinuxStream {
    fn write_loop(&self, mut consumer: Consumer<[u8; AUDIO_BUFFER_LENGTH]>) {
        let sleep_dur = Duration::from_millis(1);
        loop {
            while let Ok(bytes) = consumer.pop() {
                if let Err(err) = self.stream.write(&bytes) {
                    println!("Error while writing to audio stream: {:?}", err);
                }
            }
            // sleep while queue is empty to not 100% a CPU
            thread::sleep(sleep_dur);
        }
    }

    fn read_loop(&self, mut producer: Producer<[u8; AUDIO_BUFFER_LENGTH]>) {
        let mut buffer = [0u8; AUDIO_BUFFER_LENGTH];
        loop {
            // TODO: normal error handling after testing
            if let Ok(()) = self.stream.read(&mut buffer) {
                let _ = producer.push(buffer);
            }
        }
    }

    fn get_audio_params(&self) -> AudioParams {
        self.params.clone()
    }
}

pub fn create_audio_stream(dir: SpeakerDirection, audio_params: AudioParams) -> Option<Box<dyn AudioStream>> {
    let spec = Spec {
        format: convert_to_format(audio_params.format),
        channels: audio_params.channels,
        rate: audio_params.rate
    };
    
    assert!(spec.is_valid());

    // TODO: Refactor this later, don't know why Rust doesn't allow me to normally init Option
    let stream = if let SpeakerDirection::Capture = dir {
            let default_mon = get_default_monitor_name().expect("No default monitor name found");
            create_simple(Direction::Record, &spec, Some(default_mon.as_str()))
        } else {
            create_simple(Direction::Playback, &spec, None)
        };

    if let Ok(stream) = stream {
        let params = AudioParams {
            rate: spec.rate,
            channels: spec.channels,
            format: convert_from_format(spec.format)
        };
        let audio_stream = LinuxStream{stream, params};
        return Some(Box::new(audio_stream));
    }

    None
}

fn create_simple(dir: Direction, spec: &Spec, dev: Option<&str>) -> Result<Simple, PAErr> {
    Simple::new(
        None,
        "WifiSpeaker",
        dir,
        dev,
        "Desktop Capture",
        &spec,
        None,
        None,
    )
}

// Helper function to query the active system playback device
fn get_default_monitor_name() -> Option<String> {
    let mut mainloop = Mainloop::new().expect("No mainloop");
    let proplist = Proplist::new().expect("No proplist");

    let mut context = Context::new_with_proplist(&mainloop, "DeviceScanner", &proplist).expect("No context");
    context.connect(None, FlagSet::NOFLAGS, None).expect("Error while connect");

    loop {
        match mainloop.iterate(false) {
            IterateResult::Quit(_) | IterateResult::Err(_) => {
                println!("Erorr while mainloop iterate");
                return None;
            }
            IterateResult::Success(_) => {}
        }

        match context.get_state() {
            State::Ready => {
                break;
            }
            State::Failed | State::Terminated => {
                println!("Connect context failed or terminated");
                return None;
            }
            _ => {

            }
        }
    }

    let default_sink_store = Rc::new(RefCell::new(None));
    let store_clone = Rc::clone(&default_sink_store);

    let op = context.introspect().get_server_info(move |info| {
        if let Some(sink_name) = &info.default_sink_name {
            *store_clone.borrow_mut() = Some(sink_name.to_string());
        }
    });

    loop {
        match mainloop.iterate(true) {
            IterateResult::Quit(_) | IterateResult::Err(_) => return None,
            IterateResult::Success(_) => {}
        }

        if op.get_state() != libpulse_binding::operation::State::Running {
            break;
        }
    }

    let mut sink_name = default_sink_store.borrow_mut().take()?;
    sink_name.push_str(".monitor");
    
    println!("Discovered system default monitor: {}", sink_name);
    Some(sink_name)
}

pub fn convert_to_format(num: u8) -> Format {
    match num {
        8 => Format::U8,
        16 => Format::S16NE,
        24 => Format::S24NE,
        32 => Format::FLOAT32NE,
        33 => Format::S32NE,
        _ => Format::Invalid
    }
}

pub fn convert_from_format(format: Format) -> u8 {
    match format {
        Format::U8 => 8,
        Format::S16NE => 16,
        Format::S24NE => 24,
        Format::FLOAT32NE => 32,
        Format::S32NE => 33,
        _ => 0
    }
}
