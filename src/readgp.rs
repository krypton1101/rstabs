use core::panic;
use std::{fs, io::Read};

use guitarpro::gp;

fn main() {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .open("resources/Span - Peaceful.mscz")
        .unwrap_or_else(|_error| {
            panic!("Unknown error while reading file");
        });
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap_or_else(|_error| {
        panic!("Unable to read file contents");
    });
    let mut song: gp::Song = gp::Song::default();
    song.read_gp5(&data);
    // for i in data {
    //     print!("{} ", i);
    // }
    for measure in &song.tracks[0].measures {
        for voice in &measure.voices {
            for beat in &voice.beats {
                for note in &beat.notes {
                    if note.value != 0 {
                        println!("{}", note.value);
                    }
                }
            }
        }
    }
}
