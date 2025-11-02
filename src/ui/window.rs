use std::cell::{Cell};
pub(crate) use crate::{
    guitar::fretboard::Fretboard
    ,
    ui::component::fretboard::FretboardComponent
};

use eframe::{
    egui::{
        Align2, CentralPanel, Color32, FontFamily, FontId, Sense, Vec2
    },
    App
};
use eframe::egui::RichText;
use crate::guitar::fretboard::Point;

pub struct FretboardApp {
    fret_board : Fretboard,
    cur_select_point : Cell<Option<Point>>,
}

impl FretboardApp {
    pub fn new(fretboard : Fretboard) -> FretboardApp {
        FretboardApp {
            fret_board : fretboard,
            cur_select_point : Cell::new(None),
        }
    }
}

impl App for FretboardApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Fretboard");

            // Allocate space for our custom drawing
            let (id, rect) = ui.allocate_space(Vec2::new(ui.available_width(), 200.0));
            let fret_board_component : FretboardComponent = FretboardComponent::new(&self.fret_board, rect);
            
            // Draw a background for the fretboard
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::from_rgb(100, 50, 0)); // Brown fretboard
            fret_board_component.draw_fretboard(painter, rect);

            let response = ui.interact(rect, id, Sense::click());
            if response.hovered() {
                match response.hover_pos() {
                    None => {},
                    Some(mouse_pos) => {
                        match fret_board_component.get_mouse_inside_point_and_rect(mouse_pos) {
                            None => {},
                            Some((mouse_inside_point, mouse_inside_rect)) => {
                                if response.clicked() {
                                    self.cur_select_point.set(Some(mouse_inside_point));
                                } else {
                                    let point_display =
                                        self.fret_board.note_of_point(&mouse_inside_point)
                                            .string_representation();

                                    // Draw the custom rectangle using the allocated rect
                                    let painter = ui.painter();
                                    painter.rect_filled(mouse_inside_rect, 10, Color32::WHITE);
                                    painter.text(mouse_inside_rect.center(), Align2::CENTER_CENTER, point_display,
                                                 FontId::new(19.0, FontFamily::Proportional), Color32::BLACK);
                                }
                            }
                        }
                    }
                };
            }

            let cur_fret_label_text = "String:\t".to_owned() + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => point.on_string().to_string(),
            };
            ui.label(RichText::new(cur_fret_label_text).font(FontId::new(19.0, FontFamily::Proportional)));

            let cur_fret_label_text = "Fret:\t".to_owned() + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => point.behind_fret().to_string(),
            };
            ui.label(RichText::new(cur_fret_label_text).font(FontId::new(19.0, FontFamily::Proportional)));

            let cur_note_label_text = "Note:\t".to_owned() + &match self.cur_select_point.get() {
                None => "None".to_owned(),
                Some(point) => self.fret_board.note_of_point(&point).string_representation()
            };
            ui.label(RichText::new(cur_note_label_text).font(FontId::new(19.0, FontFamily::Proportional)));
        });
    }
}