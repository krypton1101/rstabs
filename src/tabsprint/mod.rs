use std::{u16, usize};

use console::{style, Key, Term};
use guitarpro::enums::{NoteType, SlideType};
use guitarpro::track::Track;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait TabsPrint {
    fn set_tab(&mut self, tracks: Vec<Track>);
    fn cursor_move(&mut self, direction: Direction);
}

pub trait ReadInput {
    fn read_key(&self) -> Result<Key, std::io::Error>;
}

pub struct Terminal {
    term: Term,
    // TODO: remove pub
    pub tab: Vec<Track>,
    edit_mode: bool,
    shift: u16,
    cursor_pos: (u16, u16, u8),
}

#[derive(Clone)]
struct NoteText {
    pub note_type: NoteType,
    pub value: i16,
    pub is_selected: bool,
    pub tie_left: bool,
    pub tie_right: bool,
    pub hammer_left: bool,
    pub hammer_right: bool,
    pub harmonic: bool,
    pub ghost_note: bool,
    pub slides: Vec<SlideType>,
}

struct MeasureText {
    beats: Vec<guitarpro::beat::Beat>,
    strings: Vec<Vec<NoteText>>,
}

impl MeasureText {
    pub fn gen_text(&self) -> String {
        let mut result = vec![String::new(); self.strings.len()];
        for string_num in 0..self.strings.len() {
            match string_num {
                0 => {
                    result[string_num].push_str("╭");
                }
                5 => {
                    result[string_num].push_str("╰");
                }
                _ => {
                    result[string_num].push_str("├");
                }
            }
            for note_num in 0..self.strings[string_num].len() {
                result[string_num].push_str(self.strings[string_num][note_num].gen_text().as_str());
            }
        }
        result.join("\n")
    }
}

impl NoteText {
    pub fn new() -> Self {
        NoteText {
            note_type: NoteType::Normal,
            value: -1,
            is_selected: false,
            tie_left: false,
            tie_right: false,
            hammer_left: false,
            hammer_right: false,
            harmonic: false,
            ghost_note: false,
            slides: Vec::new(),
        }
    }
    pub fn gen_text(&self) -> String {
        let mut parts: Vec<String> = vec![String::from("─"); 6];
        let mut is_value_of_len_two = false;
        if self.value != -1 {
            match self.note_type {
                NoteType::Tie => (),
                NoteType::Rest => parts[2] = "P".to_string(),
                NoteType::Dead => parts[2] = "X".to_string(),
                NoteType::Normal => {
                    is_value_of_len_two = self.value >= 10;
                    if is_value_of_len_two {
                        parts[2] = (self.value / 10).to_string();
                        parts[3] = (self.value % 10).to_string();
                    } else {
                        parts[2] = self.value.to_string();
                    }
                }
                NoteType::Unknown(_) => parts[2] = "u".to_string(),
            }
            if self.ghost_note {
                parts[1] = "(".to_string();
                if is_value_of_len_two {
                    parts[4] = ")".to_string();
                } else {
                    parts[3] = ")".to_string();
                }
            }
            for slide in &self.slides {
                match slide {
                    SlideType::None => (),
                    SlideType::IntoFromAbove => parts[0] = "╲".to_string(),
                    SlideType::IntoFromBelow => parts[0] = "╱".to_string(),
                    SlideType::ShiftSlideTo => parts[5] = "╱".to_string(),
                    SlideType::LegatoSlideTo => parts[5] = "╱".to_string(),
                    SlideType::OutUpWards => parts[5] = "╱".to_string(),
                    SlideType::OutDownwards => parts[5] = "╲".to_string(),
                }
            }
            if self.harmonic && !self.ghost_note {
                parts[1] = "◇".to_string();
            }
            if self.tie_left {
                parts[0].push_str("︭");
                parts[1].push_str("︨︬");
            }
            if self.tie_right {
                parts[2].push_str("︧︫");
                for i in 3..6 {
                    parts[i].push_str("︭");
                }
            }
            if self.hammer_left && self.hammer_right {
                for i in 0..6 {
                    parts[i].push_str("");
                }
            } else {
                if self.hammer_left {
                    parts[0].push_str("︦");
                    parts[1].push_str("︡︥");
                }
                if self.hammer_right {
                    parts[2].push_str("︠︤");
                    for i in 3..6 {
                        parts[i].push_str("︦");
                    }
                }
            }
        } else if self.hammer_left {
            for i in 0..6 {
                parts[i].push_str("");
            }
        }
        if self.is_selected {
            parts[2] = style(parts[2].clone())
                .bg(console::Color::White)
                .fg(console::Color::Black)
                .to_string();
            if is_value_of_len_two {
                parts[3] = style(parts[3].clone())
                    .bg(console::Color::White)
                    .fg(console::Color::Black)
                    .to_string();
            }
        }

        parts.join("")
    }
}

impl Terminal {
    pub fn new() -> Self {
        let term = Term::buffered_stdout();
        term.show_cursor().unwrap();
        Terminal {
            term,
            tab: Vec::new(),
            edit_mode: false,
            shift: 0,
            cursor_pos: (0, 0, 0),
        }
    }

    fn write_tab(&self) {
        self.term
            .clear_last_lines(self.term.size().0 as usize)
            .unwrap();
        self.term.flush().expect("error writing tab");
        let mut measures: Vec<MeasureText> = Vec::new();
        for measure_num in (self.shift) as usize..((self.term.size().0 / 8) + self.shift) as usize {
            let measure = MeasureText {
                beats: self.tab[0].measures[measure_num].voices[0].beats.clone(),
                strings: vec![
                    vec![
                        NoteText::new();
                        self.tab[0].measures[measure_num].voices[0].beats.len()
                    ];
                    self.tab[0].strings.len()
                ],
            };
            measures.push(measure);
        }
        for measure_num in 0..(self.term.size().0 / 8) as usize {
            for beat_num in 0..measures[measure_num].beats.len() {
                for note_num in 0..measures[measure_num].beats[beat_num].notes.len() {
                    let note_str =
                        measures[measure_num].beats[beat_num].notes[note_num].string as usize - 1;
                    if measures[measure_num].beats[beat_num].notes[note_num].kind == NoteType::Tie {
                        measures[measure_num].strings[note_str][beat_num].tie_left = true;
                        match measures[measure_num].strings[note_str].get_mut(if beat_num == 0 {
                            usize::MAX
                        } else {
                            beat_num - 1
                        }) {
                            Some(note_text) => note_text.tie_right = true,
                            None => {
                                if measure_num != 0 {
                                    match measures.get_mut(measure_num - 1) {
                                        Some(m) => {
                                            match m.strings[note_str].get_mut(m.beats.len() - 1) {
                                                Some(note_text) => note_text.tie_right = true,
                                                None => (),
                                            }
                                        }
                                        None => (),
                                    }
                                }
                            }
                        };
                    }
                    if measures[measure_num].beats[beat_num].notes[note_num]
                        .effect
                        .hammer
                    {
                        measures[measure_num].strings[note_str][beat_num].hammer_right = true;
                        match measures[measure_num].strings[note_str].get_mut(beat_num + 1) {
                            Some(note) => note.hammer_left = true,
                            None => match measures.get_mut(measure_num + 1) {
                                None => (),
                                Some(m) => m.strings[note_num][0].hammer_left = true,
                            },
                        }
                    }
                    // Transfering data to NoteText
                    {
                        let note_value =
                            measures[measure_num].beats[beat_num].notes[note_num].value;
                        measures[measure_num].strings[note_str][beat_num].value = note_value;
                    }
                    {
                        let note_slides = measures[measure_num].beats[beat_num].notes[note_num]
                            .effect
                            .slides
                            .clone();
                        measures[measure_num].strings[note_str][beat_num].slides = note_slides;
                    }
                    {
                        // NoteType does not implement Clone trait, so...
                        match measures[measure_num].beats[beat_num].notes[note_num].kind {
                            NoteType::Tie => {
                                measures[measure_num].strings[note_str][beat_num].note_type =
                                    NoteType::Tie
                            }
                            NoteType::Rest => {
                                measures[measure_num].strings[note_str][beat_num].note_type =
                                    NoteType::Rest
                            }
                            NoteType::Dead => {
                                measures[measure_num].strings[note_str][beat_num].note_type =
                                    NoteType::Dead
                            }
                            NoteType::Normal => {
                                measures[measure_num].strings[note_str][beat_num].note_type =
                                    NoteType::Normal
                            }
                            NoteType::Unknown(x) => {
                                measures[measure_num].strings[note_str][beat_num].note_type =
                                    NoteType::Unknown(x)
                            }
                        }
                    }
                    if measures[measure_num].beats[beat_num].notes[note_num]
                        .effect
                        .harmonic
                        != None
                    {
                        measures[measure_num].strings[note_num][beat_num].harmonic = true;
                    }
                    measures[measure_num].strings[note_str][beat_num].ghost_note =
                        measures[measure_num].beats[beat_num].notes[note_num]
                            .effect
                            .ghost_note;
                    measures[measure_num].strings[note_str][beat_num].is_selected =
                        self.cursor_pos == (measure_num as u16, beat_num as u16, note_str as u8);
                }
                if !self.edit_mode
                    && self.cursor_pos.0 == measure_num as u16 + self.shift
                    && self.cursor_pos.1 == beat_num as u16
                {
                    for string_num in 0..self.tab[0].strings.len() {
                        measures[measure_num].strings[string_num][beat_num].is_selected = true;
                    }
                }
            }
            // let mut string_durations = String::from(" ");
            // for beat in &self.tab[0].measures[measure_num as usize].voices[0].beats {
            //     if beat.duration.dotted {
            //         string_durations.push_str(
            //             format!("{: ^5}", beat.duration.value.to_string() + ".").as_str(),
            //         );
            //     } else {
            //         string_durations.push_str(format!("{: ^6}", beat.duration.value).as_str());
            //     }
            //     // string_durations.push('-');
            //     // string_durations.push_str(beat.duration.tuplet_times.to_string().as_str());
            //     // string_durations.push('-');
            //     // string_durations.push_str(beat.duration.tuplet_enters.to_string().as_str());
            //     // string_durations.push('-');
            //     // string_durations.push_str(beat.duration.min_time.to_string().as_str());
            // }
            // measure.push(string_durations);
        }

        self.term
            .write_line(
                measures
                    .into_iter()
                    .map(|measure_iter| measure_iter.gen_text())
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_str(),
            )
            .unwrap();

        self.term.flush().expect("error writing tab");
    }
}

impl ReadInput for Terminal {
    fn read_key(&self) -> Result<Key, std::io::Error> {
        self.term.read_key()
    }
}

impl TabsPrint for Terminal {
    fn set_tab(&mut self, tracks: Vec<Track>) {
        self.tab.append(&mut tracks.clone());
        self.write_tab();
    }

    fn cursor_move(&mut self, direction: Direction) {
        if !self.edit_mode {
            match direction {
                Direction::Up => {
                    if self.cursor_pos.0 > 0 {
                        self.cursor_pos.0 -= 1;
                    }
                    if self.shift > self.cursor_pos.0 && self.shift > 0 {
                        self.shift -= 1;
                    }
                    self.cursor_pos.1 = 0;
                    self.write_tab();
                }
                Direction::Down => {
                    if self.cursor_pos.0 < self.tab[0].measures.len() as u16 - 1 {
                        self.cursor_pos.0 += 1;
                    }
                    if self.shift + (self.term.size().0 / 8) < self.cursor_pos.0 + 1
                        && self.shift < self.tab[0].measures.len() as u16
                    {
                        self.shift += 1;
                    }
                    self.cursor_pos.1 = 0;
                    self.write_tab();
                }
                Direction::Left => {
                    if self.cursor_pos.1 == 0 {
                        if self.cursor_pos.0 > 0 {
                            self.cursor_pos.0 -= 1;
                        }
                        if self.shift > self.cursor_pos.0 && self.shift > 0 {
                            self.shift -= 1;
                        }
                        self.cursor_pos.1 = self.tab[0].measures[self.cursor_pos.0 as usize].voices
                            [0]
                        .beats
                        .len() as u16
                            - 1;
                    } else {
                        self.cursor_pos.1 -= 1;
                    }
                    self.write_tab();
                }
                Direction::Right => {
                    if self.cursor_pos.1
                        == self.tab[0].measures[self.cursor_pos.0 as usize].voices[0]
                            .beats
                            .len() as u16
                            - 1
                    {
                        if self.cursor_pos.0 < self.tab[0].measures.len() as u16 - 1 {
                            self.cursor_pos.0 += 1;
                        }
                        if self.shift + (self.term.size().0 / 8) < self.cursor_pos.0 + 1
                            && self.shift < self.tab[0].measures.len() as u16
                        {
                            self.shift += 1;
                        }
                        self.cursor_pos.1 = 0;
                    } else {
                        self.cursor_pos.1 += 1;
                    }
                    self.write_tab();
                }
            }
        } else {
            match direction {
                Direction::Up => {
                    self.cursor_pos.2 -= 1;
                    self.write_tab();
                }
                Direction::Down => {
                    self.cursor_pos.2 += 1;
                    self.write_tab();
                }
                Direction::Left => {
                    self.cursor_pos.1 -= 1;
                    self.write_tab();
                }
                Direction::Right => {
                    self.cursor_pos.1 += 1;
                    self.write_tab();
                }
            }
        }
    }
}
