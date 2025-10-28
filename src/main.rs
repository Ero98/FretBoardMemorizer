pub mod note;
pub mod fretboard;
pub mod scale;
use std::marker;

use eframe::{egui::{Align2, Area, CentralPanel, Color32, FontFamily, FontId, Id, Painter, Pos2, Rect, Sense, Stroke, Vec2, WidgetRect}, run_native, App, NativeOptions};
use crate::{fretboard::{Fretboard, Point}, note::{Accidental, Note, MIDDLE_C}};

type StringName = u8;
type FretNum = u8;

struct StringRepresent {
    unit_rect_y_pos : f32,
    mouse_response_unit_box : Rect 
}

enum FretMark {
    SINGLE_DOT, DOUBLE_DOT
}

struct FretRepresent {
    center_unit_rect_x_pos : f32,
    fret_unit_rect_x_pos : f32,
    mouse_response_unit_box : Rect,
    fret_mark : Option<FretMark>
}

struct FretboardApp {
    fret_board : Fretboard,
    strings : Vec<(StringName, StringRepresent)>,
    frets : Vec<(FretNum, FretRepresent)>
}

impl FretboardApp {
    fn new(cc: &eframe::CreationContext<'_>, fret_board : Fretboard) -> Self {
        let fret_bar_cnt = fret_board.fret_bar_cnt();
        let mut fret_represents_vec : Vec<(FretNum, FretRepresent)> = Vec::new();
        for i in 0..fret_bar_cnt {
            let left_x_unit_pos = i as f32 / (fret_bar_cnt + 1) as f32;
            let right_x_unit_pos = (i + 1) as f32 / (fret_bar_cnt + 1) as f32;
            let center_x_unit_pos = (left_x_unit_pos + right_x_unit_pos) / 2.;
            fret_represents_vec.push((i, FretRepresent {
                center_unit_rect_x_pos : center_x_unit_pos,
                fret_unit_rect_x_pos : right_x_unit_pos,
                mouse_response_unit_box : Rect { 
                    min: Pos2 { x: left_x_unit_pos, y: 0. }, 
                    max: Pos2 { x: right_x_unit_pos, y: 1. } 
                },
                fret_mark : match i {
                    3 | 5 | 7 | 9 | 15 | 17 | 19 | 21 => Some(FretMark::SINGLE_DOT),
                    12 | 24 => Some(FretMark::DOUBLE_DOT),
                    _ => None
                }
            }));
        }

        let string_name_vec = fret_board.string_name_vec();
        let string_cnt = string_name_vec.len();

        let string_border = 0.05;
        let rect_fret_board_top = string_border;
        let rect_fret_board_height = 1. - string_border * 2.;

        let string_gap_detect_ratio = 1.;
        let string_detect_half_width : f32 = rect_fret_board_height / ( (string_cnt - 1) as f32 * (2. + string_gap_detect_ratio / 2.));

        // Calculate the horizontal string representations
        let mut string_represent_vec : Vec<(StringName, StringRepresent)> = Vec::new();
        for i in string_name_vec {
            let center_y_unit_pos = rect_fret_board_top + ((i - 1) as f32 / (string_cnt - 1) as f32) * rect_fret_board_height;
            string_represent_vec.push((i, StringRepresent {
                unit_rect_y_pos : center_y_unit_pos,
                mouse_response_unit_box : Rect {
                    min : Pos2 { x: 0., y: center_y_unit_pos - string_detect_half_width },
                    max : Pos2 { x: 1., y: center_y_unit_pos + string_detect_half_width },
                }
            }));
        }

        FretboardApp {
            fret_board : fret_board,
            strings : string_represent_vec,
            frets : fret_represents_vec
        }
    }

    fn draw_fretboard(&self, painter: &Painter, rect: Rect) {
        self.draw_fret_dots(painter, rect, &self.frets);

        // Draw the vertical frets
        for (_, fret) in &self.frets {
            let x_pos = rect.left() + rect.width() * fret.fret_unit_rect_x_pos;
            painter.vline(x_pos, rect.y_range(), Stroke::new(2.0, Color32::GRAY));
        }

        // Draw the horizontal strings
        for (string_num, string) in &self.strings {
            let y_pos = rect.top() + rect.height() * string.unit_rect_y_pos;
            let string_stroke = Stroke::new(
                1. + string_num.clone() as f32 * 0.3, // Vary thickness for different strings
                Color32::from_gray(200),
            );
            painter.hline(rect.x_range(), y_pos, string_stroke);
        }
    }

    fn draw_fret_dots(&self, painter: &Painter, rect: Rect, frets : &Vec<(FretNum, FretRepresent)>) {
        for (_, fret) in frets {
            match fret.fret_mark {
                Some(FretMark::SINGLE_DOT) => {
                    // Draw fret dots at specific positions (e.g., frets 3, 5, 7, 9, 12)
                    let x_center = rect.left() + rect.width() * fret.center_unit_rect_x_pos;
                    let y_center = rect.center().y;
                    painter.circle_filled(Pos2::new(x_center, y_center), 5.0, Color32::from_gray(100));
                },
                Some(FretMark::DOUBLE_DOT) => {
                    // Fret 12 often has two dots
                    let x_center = rect.left() + rect.width() * fret.center_unit_rect_x_pos;
                    painter.circle_filled(Pos2::new(x_center, rect.center().y - 20.0), 5.0, Color32::from_gray(100));
                    painter.circle_filled(Pos2::new(x_center, rect.center().y + 20.0), 5.0, Color32::from_gray(100));
                },
                None => continue,
            }
        }
    } 
}

trait Redeployable {
    fn redeploy(&self, x_resize : f32, y_resize : f32, x_translate : f32, y_translate : f32) -> Self;
    fn redeploy_by_parent(&self, parent : Rect) -> Self where Self: Sized {
        let parent_size = parent.size();
        let parent_translate = parent.left_top();
        self.redeploy(parent_size.x, parent_size.y, parent_translate.x, parent_translate.y)
    }
}

impl Redeployable for Rect {
    fn redeploy(&self, x_resize : f32, y_resize : f32, x_translate : f32, y_translate : f32) -> Rect {
        Rect {
            min : Pos2 { 
                x: self.min.x * x_resize + x_translate, 
                y: self.min.y * y_resize + y_translate 
            },
            max : Pos2 {
                x: self.max.x * x_resize + x_translate, 
                y: self.max.y * y_resize + y_translate
            }
        }
    }
}

impl App for FretboardApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Fretboard");

            // Allocate space for our custom drawing
            let (id, rect) = ui.allocate_space(Vec2::new(ui.available_width(), 200.0));
            
            // Draw a background for the fretboard
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::from_rgb(100, 50, 0)); // Brown fretboard
            self.draw_fretboard(painter, rect);

            let response = ui.interact(rect, id, Sense::hover());
            if response.hovered() {
                match response.hover_pos() {
                    None => {},
                    Some(mouse_pos) =>
                        for (string_num, string) in &self.strings {
                            let string_mouse_rect = string.mouse_response_unit_box.redeploy_by_parent(rect);
                            if ! string_mouse_rect.contains(mouse_pos) {
                                continue;
                            }
                            
                            for (fret_num, fret) in &self.frets {
                                let fret_mouse_rect = fret.mouse_response_unit_box.redeploy_by_parent(rect);
                                if ! fret_mouse_rect.contains(mouse_pos) {
                                    continue;
                                }

                                let point_rect = string_mouse_rect.intersect(fret_mouse_rect);
                                let point_display = self.fret_board
                                        .note_of_point(Point::of(string_num.clone(), fret_num.clone()))
                                        .string_representation();
                                
                                // Draw the custom rectangle using the allocated rect
                                let painter = ui.painter();
                                painter.rect_filled(point_rect, 10, Color32::WHITE);
                                painter.text(point_rect.center(), Align2::CENTER_CENTER, point_display, 
                                    FontId::new(19.0, FontFamily::Proportional), Color32::BLACK);
                            }
                        }
                };
            }
        });
    }
}

fn main() {
    let win_option = NativeOptions::default();
    let _ = run_native("Guitar App", win_option, 
        Box::new(|cc| Ok(Box::new(FretboardApp::new(cc, Fretboard::of_fret_cnt(14))))));
}