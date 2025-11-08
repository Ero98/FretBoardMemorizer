pub(crate) use crate::{
    guitar::fretboard::Fretboard, ui::component::fretboard::FretboardComponent,
};
use std::cell::Cell;

use crate::guitar::fretboard::Point;
use eframe::egui::{Rect, RichText, Ui};
use eframe::{
    App,
    egui::{Align2, CentralPanel, Color32, FontFamily, FontId, Sense, Vec2},
};
use eframe::epaint::Hsva;
use crate::music::note::Note;
use crate::music::scale;
use crate::service::scale_map;

pub struct FretboardApp {
    fretboard: Fretboard,
    cur_select_point : Cell<Option<Point>>,
    show_scale_map : Cell<bool>,
}

impl FretboardApp {
    pub fn new() -> FretboardApp {
        FretboardApp {
            fretboard: Fretboard::of_fret_cnt(14),
            cur_select_point: Cell::new(None),
            show_scale_map: Cell::new(false),
        }
    }

    fn show_note_in_rect(ui: &mut Ui, note: Note, display_rect: Rect, fill_color: Color32) {
        let note_text = note.string_representation();
        ui.painter().rect_filled(display_rect, 10, fill_color);
        ui.painter().text(display_rect.center(), Align2::CENTER_CENTER,
                          note_text,
                          FontId::new(19.0, FontFamily::Proportional),
                          Color32::BLACK);
    }
}

impl App for FretboardApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let cur_string_label_text = "String:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => point.on_string().to_string(),
            };
            let cur_fret_label_text = "Fret:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => point.behind_fret().to_string(),
            };
            let cur_note_label_text = "Note:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => self
                    .fretboard
                    .note_of_point(&point)
                    .string_representation(),
            };
            let cur_note_as_root_major_scale = "Major Scale:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => scale::major_scale_of(
                    self.fretboard.note_of_point(&point).note_name())
                    .map(|note_name| note_name.string_representation())
                    .map(|str| format!("{: <2}", str))
                    .join("  "),
            };
            let cur_note_as_root_minor_scale = "Minor Scale:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => scale::minor_scale_of(
                    self.fretboard.note_of_point(&point).note_name())
                    .map(|note_name| note_name.string_representation())
                    .map(|str| format!("{: <2}", str))
                    .join("  "),
            };
            let cur_note_as_root_pentatonic_major_scale = "Pentatonic Major Scale:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => scale::major_pentatonic_scale_of(
                    self.fretboard.note_of_point(&point).note_name())
                    .map(|note_name| note_name.string_representation())
                    .map(|str| format!("{: <2}", str))
                    .join("  "),
            };
            let cur_note_as_root_pentatonic_minor_scale = "Pentatonic Minor Scale:\t".to_owned()
                + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => scale::minor_pentatonic_scale_of(
                    self.fretboard.note_of_point(&point).note_name())
                    .map(|note_name| note_name.string_representation())
                    .map(|str| format!("{: <2}", str))
                    .join("  "),
            };

            // View
            // 标题 Heading
            ui.heading("Guitar Fretboard");
            // 指板显示 Fretboard view
            let (fretboard_id, fretboard_rect) = ui.allocate_space(Vec2::new(ui.available_width(), 200.0));
            let fret_board_component: FretboardComponent = FretboardComponent::new(&self.fretboard, fretboard_rect);
            ui.painter().rect_filled(fretboard_rect, 0.0, Color32::from_rgb(100, 50, 0)); // draw fretboard background: Brown
            fret_board_component.draw_fretboard_widgets(ui.painter(), fretboard_rect);
            // 选中提示 Selection indicator
            ui.label(RichText::new(cur_string_label_text).font(FontId::new(19.0, FontFamily::Proportional)));
            ui.label(RichText::new(cur_fret_label_text).font(FontId::new(19.0, FontFamily::Proportional)));
            ui.label(RichText::new(cur_note_label_text).font(FontId::new(19.0, FontFamily::Proportional)));
            ui.label(RichText::new(cur_note_as_root_major_scale).font(FontId::new(19.0, FontFamily::Monospace)));
            ui.label(RichText::new(cur_note_as_root_minor_scale).font(FontId::new(19.0, FontFamily::Monospace)));
            ui.label(RichText::new(cur_note_as_root_pentatonic_major_scale).font(FontId::new(19.0, FontFamily::Monospace)));
            ui.label(RichText::new(cur_note_as_root_pentatonic_minor_scale).font(FontId::new(19.0, FontFamily::Monospace)));
            let scale_map_button = ui.button("Click me");

            // Controller
            // 音名悬浮显示、选择 Note name hover-display and selection
            let fretboard_response = ui.interact(fretboard_rect, fretboard_id, Sense::click());
            if fretboard_response.hovered() {
                if let Some(mouse_pos) = fretboard_response.hover_pos() {
                    if let Some((mouse_inside_point, mouse_inside_rect)) =
                        fret_board_component.get_mouse_on_point_and_rect(mouse_pos) {

                        if fretboard_response.clicked() {
                            self.cur_select_point.set(Some(mouse_inside_point));
                        } else {
                            Self::show_note_in_rect(ui, self.fretboard.note_of_point(&mouse_inside_point), mouse_inside_rect, Color32::WHITE);
                        }
                    }
                };
            }

            if scale_map_button.clicked() {
                if let Some(..) = self.cur_select_point.get() {
                    self.show_scale_map.set(!self.show_scale_map.get());
                }
            }

            if self.show_scale_map.get() {
                if let Some(selected_point) = self.cur_select_point.get() {
                    let scale_points = scale_map::scale_notes_on_fretboard(
                        &self.fretboard,
                        scale::major_scale_of(
                            self.fretboard.note_of_point(&selected_point).note_name()));
                    for point in scale_points {
                        let fill_color = if point == self.cur_select_point.get().unwrap() {
                            Color32::LIGHT_GREEN
                        } else {
                            Color32::WHITE
                        };

                        let note = self.fretboard.note_of_point(&point);
                        let selected_note = self.fretboard.note_of_point(&selected_point);
                        let note_color = get_color_of_note(&note, &selected_note);

                        Self::show_note_in_rect(ui, note,
                            fret_board_component.get_rect_on_point(point).unwrap(), note_color);
                    }
                }
            }
        });
    }
}

fn get_color_of_note(note: &Note, base_note: &Note) -> Color32 {
    let hue = match note.octave() {
        2 => 0.,
        3 => 0.25,
        4 => 0.5,
        5 => 0.75,
        _ => 0.,
    };

    let semitone_from_base = (note.minus_note(base_note).semitone_diff() % 12 + 12) % 12;
    let hsva = Hsva::new(hue, (semitone_from_base + 1) as f32 / 12., 1., 1.);
    Color32::from(hsva)
}