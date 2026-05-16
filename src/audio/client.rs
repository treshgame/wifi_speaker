use rtrb::Consumer;

use crate::{audio::pcm_utils, core::AUDIO_BUFFER_LENGTH};

pub fn audio_client(mut queue: Consumer<[i16; AUDIO_BUFFER_LENGTH]>) {
    let pcm = pcm_utils::create_pcm();
    if let Err(err) = pcm {
        println!("Error while creating pcm for client {:?}", err);
    }
    let pcm = pcm.unwrap();
    let io = pcm.io_i16().expect("Error to create IO for pcm");
    loop {
        if let Ok(bytes) = queue.pop() {
            io.writei(&bytes).unwrap();
            println!("Bytes wrote: {:?}", bytes);
        }
    }
}
