mod tabsprint;

use guitarpro::*;
use std::fs;
use std::io::Read;
use tabsprint::ReadInput;
use tabsprint::TabsPrint;

fn main() {
    let mut f = fs::OpenOptions::new()
        .read(true)
        .open("resources/I Built The Sky-Up Into the Ether.gp5")
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

    loop {
        let key = terminal.read_key();
        match key {
            Ok(console::Key::Char('h')) | Ok(console::Key::ArrowLeft) => {
                terminal.cursor_move(tabsprint::Direction::Left)
            }
            Ok(console::Key::Char('l')) | Ok(console::Key::ArrowRight) => {
                terminal.cursor_move(tabsprint::Direction::Right)
            }
            Ok(console::Key::Char('j')) | Ok(console::Key::ArrowUp) => {
                terminal.cursor_move(tabsprint::Direction::Up)
            }
            Ok(console::Key::Char('k')) | Ok(console::Key::ArrowDown) => {
                terminal.cursor_move(tabsprint::Direction::Down)
            }
            Ok(console::Key::Char('q')) => break,
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        }
    }
    // println!("{:?}", terminal.tab[0].measures[2].voices[0].beats[1]);
}
