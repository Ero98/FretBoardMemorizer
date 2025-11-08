pub mod music;
pub mod guitar;
pub mod service;
pub mod ui;

use eframe::{NativeOptions, run_native};
use crate::ui::window::{FretboardApp};


fn main() {
    let win_option = NativeOptions::default();
    let _ = run_native("Guitar App", win_option, 
        Box::new(|_cc| Ok(Box::new(FretboardApp::new()))));
}