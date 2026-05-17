use libpulse_binding::{context::{Context, FlagSet, State}, error::PAErr, mainloop::standard::{IterateResult, Mainloop}, proplist::Proplist, sample::{Format, Spec}, stream::{self, Direction, Stream}};
use libpulse_simple_binding::Simple;

pub fn create_pusle_stream(dir: Direction) -> Option<Simple> {
    let spec = Spec{
        format: Format::S16le,
        channels: 2,
        rate: 44100
    };
    assert!(spec.is_valid());

    // TODO: Refactor this later, don't know why Rust doesn't allow me to normally init Option
    let stream = if let Direction::Record = dir {
            let default_mon = get_default_monitor_name().expect("No default monitor name found");
            create_simple(dir, &spec, Some(default_mon.as_str()))
        } else {
            create_simple(dir, &spec, None)
        };

    if let Ok(stream) = stream {
        return Some(stream);
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
    
    Some("@DEFAULT_SINK@.monitor".to_string()) 
}
