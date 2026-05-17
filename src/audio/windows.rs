use crate::{audio::{AudioStream, SpeakerDirection}, core::AUDIO_BUFFER_LENGTH};

struct WindowsStream {

}

// TODO
impl AudioStream for WindowsStream {
    fn write(&self, bytes: &[u8]) {
        println!("Windows write");
    }
    fn read(&self, buf: &mut [u8; AUDIO_BUFFER_LENGTH]) {
        
    }
}

// TODO
pub fn create_auido_stream(dir: SpeakerDirection) {
    println!("Windows direction: {:?}", dir);
}
