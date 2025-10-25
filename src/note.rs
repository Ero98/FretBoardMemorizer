use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr)]
#[derive(Clone)]
pub enum NaturalNoteName {
    C, D, E, F, G, A, B
}

#[derive(Clone)]
pub enum Accidental {
    Sharp, Flat
}

#[derive(Clone)]
pub struct NoteName {
    natural_note_name : NaturalNoteName,
    accidental : Option<Accidental>
}

impl NoteName {
    const fn of_integer_notation_sharp_by_default(integer_notation : u8) -> NoteName {
        let natural_note_name = match integer_notation {
            0..=1 => NaturalNoteName::C,
            2..=3 => NaturalNoteName::D,
            4 => NaturalNoteName::E,
            5..=6 => NaturalNoteName::F,
            7..=8 => NaturalNoteName::G,
            9..=10 => NaturalNoteName::A,
            11 => NaturalNoteName::B,
            12.. => panic!("Integer notation can not be above 11")
        };

        let accdental = match integer_notation {
            1 | 3 | 6 | 8 | 10 => Some(Accidental::Sharp),
            _ => None
        };

        NoteName { natural_note_name: natural_note_name, accidental: accdental }
    }

    /// Over 11 means next octave, below 0 means last octave.
    const fn raw_integer_notation(self) -> i8 {
        let natural_part = match self.natural_note_name {
                NaturalNoteName::C => 0,
                NaturalNoteName::D => 2,
                NaturalNoteName::E => 4,
                NaturalNoteName::F => 5,
                NaturalNoteName::G => 7,
                NaturalNoteName::A => 9,
                NaturalNoteName::B => 11,
        };

        let accidental_part = match self.accidental {
            Some(acc) => match acc {
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
            },
            None => 0,
        };

        natural_part + accidental_part
    }
}



type OctaveNumber = u8;
#[derive(Clone)]
pub struct Note {
    name : NoteName,
    octave : OctaveNumber
}
// C₄
pub const MIDDLE_C : Note = Note{name : NoteName{natural_note_name: NaturalNoteName::C, accidental: None}, octave : 4};

pub struct Interval {
    /// When two notes doing operation: a minus b returns an Interval, in this Interval:
    /// if semitone_diff > 0, it means a apears after b
    /// if semitone_diff < 0, it means a apears before b
    semitone_diff : i8
}

impl Interval {
    pub const UNISON : Interval = Interval{semitone_diff: 0};
    pub const MINOR_SECOND : Interval = Interval{semitone_diff: 1};
    pub const MAJOR_SECOND : Interval = Interval{semitone_diff: 2};
    pub const MINOR_THIRD : Interval = Interval{semitone_diff: 3};
    pub const MAJOR_THIRD : Interval = Interval{semitone_diff: 4};
    pub const PERFECT_FOURTH : Interval = Interval{semitone_diff: 5};
    pub const TRITONE : Interval = Interval{semitone_diff: 6};
    pub const PERFECT_FIFTH : Interval = Interval{semitone_diff: 7};
    pub const MINOR_SIXTH : Interval = Interval{semitone_diff: 8};
    pub const MAJOR_SIXTH : Interval = Interval{semitone_diff: 9};
    pub const MINOR_SEVENTH : Interval = Interval{semitone_diff: 10};
    pub const MAJOR_SEVENTH : Interval = Interval{semitone_diff: 11};
    pub const OCTAVE : Interval = Interval{semitone_diff: 12};

    pub fn of_semitone_diff(semitone_diff : i8) -> Interval {
        Interval { semitone_diff : semitone_diff }
    }
}

impl Note {
    pub const fn of_natural(note_name : NaturalNoteName, octave : OctaveNumber) -> Note {
        Note {
            name: NoteName {
                natural_note_name: note_name, 
                accidental: None
            },
            octave: octave
        }
    }

    pub const fn of(note_name : NaturalNoteName, accidental : Accidental, octave : OctaveNumber) -> Note {
        Note {
            name: NoteName {
                natural_note_name: note_name, 
                accidental: Some(accidental)
            },
            octave: octave
        }
    }

    pub const fn of_integer_notation(integer_notation : u8, octave : OctaveNumber) -> Note {
        Note {
            name : NoteName::of_integer_notation_sharp_by_default(integer_notation),
            octave : octave
        }
    }

    pub fn string_representation(self) -> String {
        let accidental_str = match self.name.accidental {
            Some(acc) => match acc {
                Accidental::Sharp => "#",
                Accidental::Flat => "b",
            },
            None => "",
        };

        let note_natural_part_name_string : &'static str = self.name.natural_note_name.into();
        String::from(note_natural_part_name_string) + accidental_str
    }

    pub fn minus_note(self, other_note : Note) -> Interval {
        let note_name_part = self.name.raw_integer_notation() - other_note.name.raw_integer_notation();
        let octave_part = (self.octave as i8 - other_note.octave as i8) * 12;
        
        Interval { semitone_diff: note_name_part + octave_part }
    }

    pub fn add_interval(self, interval : Interval) -> Note {
        let self_from_octave_zero = self.octave as i8 * 12 + self.name.raw_integer_notation();
        let res_from_octave_zero = self_from_octave_zero + interval.semitone_diff;
        if res_from_octave_zero < 0 {
            panic!("Octave below zero is not considered...")
        }

        let res_int_nota = res_from_octave_zero as u8 % 12;
        let res_octave = res_from_octave_zero as u8 / 12;

        Note::of_integer_notation(res_int_nota, res_octave)
    }
}