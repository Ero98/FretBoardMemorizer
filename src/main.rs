pub mod music;
pub mod guitar;
pub mod service;
pub mod ui;

use eframe::{NativeOptions, run_native};
use crate::guitar::fretboard::Fretboard;
use crate::ui::window::{FretboardApp};


fn main() {
    let win_option = NativeOptions::default();
    let fretboard = Fretboard::of_fret_cnt(14);
    let _ = run_native("Guitar App", win_option, 
        Box::new(|_cc| Ok(Box::new(FretboardApp::new(fretboard)))));
}