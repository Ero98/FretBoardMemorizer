use strum_macros::IntoStaticStr;

/// 自然音
#[derive(IntoStaticStr)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum NaturalNoteName {
    C, D, E, F, G, A, B
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Accidental {
    Sharp, Flat
}

/// 音名
#[derive(Clone)]
#[derive(Debug)]
pub struct NoteName {
    natural_note_name : NaturalNoteName,
    accidental : Option<Accidental>
}

impl NaturalNoteName {
    pub const fn natural(self) -> NoteName {
        NoteName { natural_note_name: self, accidental: None }
    }

    pub const fn sharp(self) -> NoteName {
        NoteName { natural_note_name: self, accidental: Some(Accidental::Sharp) }
    }

    pub const fn flat(self) -> NoteName {
        NoteName { natural_note_name: self, accidental: Some(Accidental::Flat) }
    }
}

/// 表示同一个音的音名
impl PartialEq for NoteName {
    fn eq(&self, other: &Self) -> bool {
        self.integer_notation() == other.integer_notation()
    }
}

impl NoteName {
    pub const fn of_integer_notation_as_sharp(integer_notation : u8) -> Self {
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
    pub const fn directional_integer_notation(&self) -> i8 {
        let natural_part = match self.natural_note_name {
                NaturalNoteName::C => 0,
                NaturalNoteName::D => 2,
                NaturalNoteName::E => 4,
                NaturalNoteName::F => 5,
                NaturalNoteName::G => 7,
                NaturalNoteName::A => 9,
                NaturalNoteName::B => 11,
        };

        let accidental_part = match &self.accidental {
            Some(acc) => match acc {
                Accidental::Sharp => 1,
                Accidental::Flat => -1,
            },
            None => 0,
        };

        natural_part + accidental_part
    }

    pub const fn integer_notation(&self) -> u8 {
        (self.directional_integer_notation() % 12) as u8
    }
}



type OctaveNumber = u8;
#[derive(Clone)]
#[derive(Debug)]
pub struct Note {
    name : NoteName,
    octave : OctaveNumber
}

impl NoteName {
    pub const fn on_octave(self, octave : OctaveNumber) -> Note {
        Note { name: self, octave : octave }
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.minus_note(other) == Interval::UNISON
    }
}

// C₄
pub const MIDDLE_C : Note = Note{name : NoteName{natural_note_name: NaturalNoteName::C, accidental: None}, octave : 4};

#[derive(PartialEq)]
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
    pub fn note_name(&self) -> NoteName {
        self.name.clone()
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

    pub fn minus_note(&self, other_note : &Note) -> Interval {
        let note_name_part = self.name.directional_integer_notation() - other_note.name.directional_integer_notation();
        let octave_part = (self.octave as i8 - other_note.octave as i8) * 12;
        
        Interval { semitone_diff: note_name_part + octave_part }
    }

    pub fn add_interval(&self, interval : &Interval) -> Note {
        let self_from_octave_zero = self.octave as i8 * 12 + self.name.directional_integer_notation();
        let res_from_octave_zero = self_from_octave_zero + interval.semitone_diff;
        if res_from_octave_zero < 0 {
            panic!("Octave below zero is not considered...")
        }

        let res_int_nota = res_from_octave_zero as u8 % 12;
        let res_octave = res_from_octave_zero as u8 / 12;

        NoteName::of_integer_notation_as_sharp(res_int_nota).on_octave(res_octave)
    }
}