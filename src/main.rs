mod tabsprint;

use guitarpro::*;
use std::fs;
use std::io::Read;
use tabsprint::TabsPrint;

fn main() {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .open("resources/Veil Of Maya-Mikasa.gp5")
        .unwrap_or_else(|_error| {
            panic!("Unknown error while reading file");
        });
    let mut data = Vec::new();
    f.read_to_end(&mut data).unwrap_or_else(|_error| {
        panic!("Unable to read file contents");
    });
    let mut song: gp::Song = gp::Song::default();
    song.read_gp5(&data);

    let mut terminal = tabsprint::Terminal::new();
    terminal.set_tab(song.tracks);

    terminal.read_key();
}



