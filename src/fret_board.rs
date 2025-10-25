use crate::note::{Interval, NaturalNoteName::*};
use crate::note::{Note};

type StringName = u8;
pub const ZERO_FRETS_STANDARD_TUNING : [(StringName, Note); 6] = [
    (1, Note::of_natural(E, 4)),
    (2, Note::of_natural(B, 3)),
    (3, Note::of_natural(G, 3)),
    (4, Note::of_natural(D, 3)),
    (5, Note::of_natural(A, 2)),
    (6, Note::of_natural(E, 2)),
];

/// When you play a note, you put your finger on a string and "behind" a fret, or lift your finger if it is the zeroth fret.
pub struct Point {
    on_string : StringName,
    behind_fret : u8
}

impl Point {
    pub fn of(on_string : StringName, behind_fret : u8) -> Point {
        if on_string == 0 {
            panic!("0th String dosen't exist...")
        }

        Point { on_string: on_string, behind_fret: behind_fret }
    }
}

pub struct FretBoard {
    zero_frets_tuning : Vec<(StringName, Note)>,
    fret_bar_cnt : u8
}

impl FretBoard {
    pub fn of_standard() -> FretBoard {
        FretBoard { zero_frets_tuning: Vec::from(ZERO_FRETS_STANDARD_TUNING), fret_bar_cnt: 24 }
    }

    pub fn string_name_vec(&self) -> Vec<StringName> {
        self.zero_frets_tuning.iter().map(|string_tuning| { string_tuning.0 }).collect()
    }

    pub fn fret_bar_cnt(&self) -> u8 {
        self.fret_bar_cnt
    }

    pub fn note_of_point(&self, point : Point) -> Note {
        self.zero_fret_note_of_string(point.on_string)
            .add_interval(Interval::of_semitone_diff(point.behind_fret as i8))
    }

    fn zero_fret_note_of_string(&self, string : StringName) -> Note {
        for string_zero_fret_note in &self.zero_frets_tuning {
            if string_zero_fret_note.0 == string {
                return string_zero_fret_note.1.clone()
            }
        }
        panic!("String of number {string} dosen't exist...")
    }
}



struct Chord;
struct Position;