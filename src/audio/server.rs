use alsa::{Card, Ctl, Direction, Error, card::Iter, pcm::PCM};
use rtrb::Producer;

use crate::{audio::pcm_utils, core::AUDIO_BUFFER_LENGTH};

pub fn audio_server(bytes_to_send: Producer<[i16; AUDIO_BUFFER_LENGTH]>) {
    println!("-------");
    let pcm = pcm_utils::create_pcm();
    if let Err(err) = pcm {
        println!("Error while creating io: {:?}", err);
        return
    }
    let pcm = pcm.unwrap();
    
    read_audio_loop(&pcm, bytes_to_send);
}

fn read_audio_loop(pcm: &PCM, mut bytes_to_send: Producer<[i16; 4096]>) {
    let io = pcm.io_i16().expect("Error getting io_i16");
    let mut buf = [0i16; 4096];
    pcm.prepare().expect("Error while preparing");
    pcm.start().expect("Error while starting");
    println!("Loop is starting");
    loop {
        match io.readi(&mut buf) {
            Ok(_) => {
                if !bytes_to_send.is_full() {
                    if let Err(_) = bytes_to_send.push(buf) {
                        println!("producer is full");
                    }
                }
            }
            Err(err) => {
                println!("Erorr while reading from an audio server: {:?}",err);
                break
            }
        }
    }
}
