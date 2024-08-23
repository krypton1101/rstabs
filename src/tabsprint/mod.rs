use console::{Key, Term};
use guitarpro::track::Track;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait TabsPrint {
    fn set_tab(&mut self, tracks: Vec<Track>);
    fn cursor_move(&self, direction: Direction);
}

pub struct Terminal {
    term: Term,
    tab: Vec<Track>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            term: Term::buffered_stdout(),
            tab: Vec::new(),
        }
    }

    fn write_tab(&self) {
        self.term.clear_screen().unwrap();
        self.term.flush().expect("error writing tab");
        for measure_num in 0..self.term.size().0 / 8 {
            for string in 0..6 {
                let mut string_notes = String::new();
                for beat_num in 0..self.tab[0].measures[measure_num as usize].voices[0]
                    .beats
                    .len()
                {
                    if beat_num == 0 {
                        match string {
                            0 => {
                                string_notes.push_str("╭");
                            }
                            5 => {
                                string_notes.push_str("╰");
                            }
                            _ => {
                                string_notes.push_str("├");
                            }
                        }
                    } else {
                        string_notes.push_str("─");
                    }

                    let mut used = false;
                    for note in
                        &self.tab[0].measures[measure_num as usize].voices[0].beats[beat_num].notes
                    {
                        if note.string == string {
                            string_notes.push_str(note.value.to_string().as_str());
                            if note.value.to_string().len() != 2 {
                                string_notes.push_str("─");
                            }
                            string_notes.push_str("─");
                            used = true;
                            break;
                        }
                    }
                    if !used {
                        string_notes.push_str("───");
                    }
                }
                self.term.write_line(string_notes.as_str()).unwrap();
            }
            let mut string_durations = String::new();
            for beat in &self.tab[0].measures[measure_num as usize].voices[0].beats {
                string_durations.push(' ');
                string_durations.push_str(beat.duration.value.to_string().as_str());
                for _ in 0..3 - beat.duration.value.to_string().len() {
                    string_durations.push(' ');
                }
            }
            self.term.write_line(string_durations.as_str()).unwrap();
        }
        self.term.flush().expect("error writing tab");
    }

    pub fn read_key(&self) -> char {
        self.term.read_char().unwrap()
    }
}

impl TabsPrint for Terminal {
    fn set_tab(&mut self, tracks: Vec<Track>) {
        self.tab.append(&mut tracks.clone());
        self.write_tab();
    }

    fn cursor_move(&self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.term.move_cursor_up(1).unwrap();
            }
            Direction::Down => {
                self.term.move_cursor_down(1).unwrap();
            }
            Direction::Left => {
                self.term.move_cursor_left(1).unwrap();
            }
            Direction::Right => {
                self.term.move_cursor_right(1).unwrap();
            }
        }
    }
}
