use crate::{guitar::fretboard::{Fretboard, Point}, music::note::NoteName};

pub fn scale_notes_on_fretboard<const N: usize>(fretboard : &Fretboard, scale : [NoteName; N]) -> Vec<Point> {
    let mut scale_notes = Vec::new();
    for string_name in fretboard.string_name_vec() {
        for fret_bar in 0..fretboard.fret_bar_cnt() {
            let cur_point = Point::of(string_name, fret_bar);
            let cur_note = fretboard.note_of_point(&cur_point);
            if scale.contains(&cur_note.note_name()) {
                scale_notes.push(cur_point);
            }
        }
    }
    scale_notes
}

#[cfg(test)]
mod scale_map {
    use std::hash::Hash;
    use std::collections::HashSet;

    use super::*; // Import everything from the parent module
    use crate::music::note::NaturalNoteName::*;
    use crate::music::scale;

    #[test]
    fn major_c_on_0_to_4() {
        assert!(is_unordered_equal(
            &[
                Point::of(1, 0), Point::of(1, 1),                  Point::of(1, 3),
                Point::of(2, 0), Point::of(2, 1),                  Point::of(2, 3),
                Point::of(3, 0),                  Point::of(3, 2),
                Point::of(4, 0),                  Point::of(4, 2), Point::of(4, 3),
                Point::of(5, 0),                  Point::of(5, 2), Point::of(5, 3),
                Point::of(6, 0), Point::of(6, 1),                  Point::of(6, 3),
            ],
            &scale_notes_on_fretboard(
                &Fretboard::of_fret_cnt(4),
                scale::major_scale_of(C.natural()))
        ));
    }

    fn is_unordered_equal<T>(a: &[T], b: &[T]) -> bool
    where
        T: Eq + Hash,
    {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();

        a == b
    }
}