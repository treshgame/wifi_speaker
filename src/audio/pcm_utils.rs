use std::{ffi::CString};

use alsa::{Card, Ctl, Direction, Error, PCM, ValueOr, ctl::DeviceIter, device_name::HintIter, pcm::{Access, Format, HwParams}};

const DEFAULT_SOUND_RATE: u32 = 44100;


pub fn create_pcm() -> Result<PCM, Error> {
    println!("----create_pcm----");
    let pcm = find_working_pcm();
    if let None = pcm {
        panic!("No proper pcm was found");
    }

    println!("Creating a pcm was successful");

    return Ok(pcm.unwrap())
    
}

fn find_working_pcm() -> Option<PCM> {
    let cards = HintIter::new(None, CString::from(c"pcm").as_c_str())
        .expect("Error while getting HintIter");
    for card in cards {
        if let None = card.name {
            continue;
        }
        let cname = card.name.unwrap();
        println!("Card name: {}", cname);
        let pcm = PCM::new(&cname, Direction::Playback, false);
        if let Err(_) = pcm {
            continue;
        }

        let pcm = pcm.unwrap();
        {
            let hwp = HwParams::any(&pcm).unwrap();
            hwp.set_channels(1).unwrap();
            hwp.set_rate(DEFAULT_SOUND_RATE, ValueOr::Nearest).unwrap();
            hwp.set_format(Format::s16()).unwrap();
            hwp.set_access(Access::RWInterleaved).unwrap();
            pcm.hw_params(&hwp).unwrap();
        }

        if let Err(err) = pcm.start() {
            println!("Error starting pcm: {:?}", err);
            continue;
        }

        {
            let io = pcm.io_i16();
            if let Err(err) = io {
                println!("Error getting io: {:?}", err);
                continue;
            }
            let io = io.unwrap();
            let mut test_buf = [0; 100];
            if let Err(err) = io.readi(&mut test_buf) {
                println!("Error while reading: {:?}", err);
                continue;
            }
        }

        return Some(pcm);
    }


    return None
}

fn card_controls(name: String) {
    let ctl = Ctl::new(&name, false);
    if let Err(err) = ctl {
        println!("Err while getting ctl for {}: {:?}", name, err);
        return;
    }
    let ctl = ctl.unwrap();
    let card_info = ctl.card_info().expect("Error while reading card_info");
    list_devices_for_card(&card_info.get_card());
}

fn list_devices_for_card(card: &Card) {
    let ctl = Ctl::from_card(card, false).expect("Erorr getting ctl from card");
    for device in DeviceIter::new(&ctl) {

    }
}
