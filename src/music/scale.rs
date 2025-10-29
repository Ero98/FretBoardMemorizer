use std::usize;

use crate::note::{Interval, Note, NoteName};

/// 音阶 Scale
fn scale_from_steps<const N : usize>(root : NoteName, semitone_step_arr : [u8; N]) -> [NoteName; N] {
    let step_sum = semitone_step_arr.iter().sum::<u8>();
    if step_sum != 12 {
        panic!("Semitone step arr must have a sum of 12 to produce a valid scale. Getting sum {step_sum} from {semitone_step_arr:?}");
    }

    let root_note = root.clone().on_octave(4);
    
    let mut scale: [NoteName; N] = core::array::from_fn(|_|{root.clone()});
    let mut semitone_acc : u8 = 0;
    for (i, &steps) in semitone_step_arr.iter().enumerate() {
        let interval = Interval::of_semitone_diff(semitone_acc as i8);
        let note = root_note.add_interval(&interval);
        scale[i] = note.note_name();
        semitone_acc += steps;
    }
    scale
}

/// 七声调式
fn heptatonic_scale_from_steps(root : NoteName, semitone_step_arr : [u8; 7]) -> [NoteName; 7] {
    scale_from_steps(root, semitone_step_arr)
}

/// 自然音阶
fn diatonic_scale_from_steps(root : NoteName, semitone_step_arr : [u8; 7]) -> [NoteName; 7] {
    let is_all_step_whole_or_half = semitone_step_arr.iter().all(|step| *step == 1 || *step == 2);
    if ! is_all_step_whole_or_half {
        panic!("Semitone steps must all be 1 or 2 to produce a diatonic scale. Getting {semitone_step_arr:?}");
    }
    scale_from_steps(root, semitone_step_arr)
}

const W : u8 = 2; // 全音 whole-tone
const H : u8 = 1; // 半音 half-tone

pub fn major_scale_of(root : NoteName) -> [NoteName; 7] {
    diatonic_scale_from_steps(root, [W, W, H, W, W, W, H])
}

pub fn minor_scale_of(root : NoteName) -> [NoteName; 7] {
    diatonic_scale_from_steps(root, [W, H, W, W, H, W, W])
}

pub fn dorian_scale_of(root : NoteName) -> [NoteName; 7] {
    diatonic_scale_from_steps(root, [W, H, W, W, W, H, W])
}

/// 五声调式
fn pentatonic_scale_by_omitting(heptatonic : [NoteName; 7], omit_number_notation1 : &usize, omit_number_notation2 : &usize) -> [NoteName; 5] {
    let mut tmp_vec = heptatonic.to_vec();
    tmp_vec.remove(omit_number_notation1 - 1);
    tmp_vec.remove(omit_number_notation2 - 1);

    core::array::from_fn(|i| {
        match tmp_vec.get(i) {
            Some(note_name) => note_name.clone(),
            None => panic!("No element found at index {i} in vec {tmp_vec:?}"),
        }
    })
}

fn major_pentatonic_scale_of(root : NoteName) -> [NoteName; 5] {
    pentatonic_scale_by_omitting(major_scale_of(root), &4, &7)
}

fn minor_pentatonic_scale_of(root : NoteName) -> [NoteName; 5] {
    pentatonic_scale_by_omitting(minor_scale_of(root), &2, &6)
}

/// 音阶绝对化，即将音阶中的每个音名转换成科学表示法（国际表示法）。假设了音阶数组仅覆盖一个八度
trait Absolutifiable<const N: usize> { fn on_octave(self, root_octave : u8) -> [Note; N]; }
impl <const N: usize> Absolutifiable<N> for [NoteName; N] {
    fn on_octave(self, root_octave : u8) -> [Note; N] {
        let root_note_name_int = self[0].clone().integer_notation();
        self.map(|note_name| {
            let note_name_int = note_name.clone().integer_notation();
            if note_name_int >= root_note_name_int {
                note_name.on_octave(root_octave)
            } else {
                note_name.on_octave(root_octave + 1)
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::note::NaturalNoteName::*;
    use super::*; // Import everything from the parent module

    #[test]
    fn major_c() {
        assert_eq!([
            C.natural(),
            D.natural(),
            E.natural(),
            F.natural(),
            G.natural(),
            A.natural(),
            B.natural(),
            ], major_scale_of(C.natural()));
    }
    
    #[test]
    fn minor_c() {
        assert_eq!([
            C.natural(),
            D.natural(),
            E.flat(),
            F.natural(),
            G.natural(),
            A.flat(),
            B.flat(),
            ], minor_scale_of(C.natural()));
    }

    #[test]
    fn dorian_a_on_4() {
        assert_eq!([
            A.natural().on_octave(4),
            B.natural().on_octave(4),
            C.natural().on_octave(5),
            D.natural().on_octave(5),
            E.natural().on_octave(5),
            F.sharp().on_octave(5),
            G.natural().on_octave(5),
        ], dorian_scale_of(A.natural()).on_octave(4));
    }
}