use crate::guitar::fretboard::{
    Fretboard, Point
};

use eframe::egui::{
    Color32, Painter, Pos2, Rect, Stroke
};

use derive_getters::Getters;


type StringName = u8;
type FretNum = u8;

struct StringComponent {
    center_y_pos: f32,
    mouse_response_rect: Rect
}

enum FretMark {
    SingleDot,
    DoubleDot
}

struct FretComponent {
    center_x_pos: f32,
    fret_x_pos: f32,
    mouse_response_rect: Rect,
    fret_mark : Option<FretMark>
}

#[derive(Getters)]
pub struct FretboardComponent {
    strings : Vec<(StringName, StringComponent)>,
    frets : Vec<(FretNum, FretComponent)>
}

impl FretboardComponent {
    pub fn new(fretboard : &Fretboard, actual_area : Rect) -> Self {
        let fret_bar_cnt = fretboard.fret_bar_cnt();
        let mut fret_represents_vec : Vec<(FretNum, FretComponent)> = Vec::new();
        for i in 0..fret_bar_cnt {
            let left_x_unit_pos = i as f32 / (fret_bar_cnt + 1) as f32;
            let right_x_unit_pos = (i + 1) as f32 / (fret_bar_cnt + 1) as f32;
            let center_x_unit_pos = (left_x_unit_pos + right_x_unit_pos) / 2.;
            let mouse_response_unit_box = Rect {
                min: Pos2 { x: left_x_unit_pos, y: 0. },
                max: Pos2 { x: right_x_unit_pos, y: 1. }
            };

            let fret_component = FretComponent {
                center_x_pos: actual_area.left() + actual_area.width() * center_x_unit_pos,
                fret_x_pos: actual_area.left() + actual_area.width() * right_x_unit_pos,
                mouse_response_rect: redeploy_by_parent(&mouse_response_unit_box, &actual_area),
                fret_mark: match i {
                    3 | 5 | 7 | 9 | 15 | 17 | 19 | 21 => Some(FretMark::SingleDot),
                    12 | 24 => Some(FretMark::DoubleDot),
                    _ => None
                }
            };

            fret_represents_vec.push((i, fret_component));
        }

        let string_name_vec = fretboard.string_name_vec();
        let string_cnt = string_name_vec.len();

        let string_border = 0.05;
        let rect_fret_board_top = string_border;
        let rect_fret_board_height = 1. - string_border * 2.;

        let string_gap_detect_ratio = 1.;
        let string_detect_half_width : f32 = rect_fret_board_height / ( (string_cnt - 1) as f32 * (2. + string_gap_detect_ratio / 2.));

        // Calculate the horizontal string representations
        let mut string_represent_vec : Vec<(StringName, StringComponent)> = Vec::new();
        for i in string_name_vec {
            let center_y_unit_pos = rect_fret_board_top + ((i - 1) as f32 / (string_cnt - 1) as f32) * rect_fret_board_height;

            let mouse_response_unit_rect = Rect {
                min : Pos2 { x: 0., y: center_y_unit_pos - string_detect_half_width },
                max : Pos2 { x: 1., y: center_y_unit_pos + string_detect_half_width },
            };

            let string_component = StringComponent {
                center_y_pos: actual_area.top() + actual_area.height() * center_y_unit_pos,
                mouse_response_rect: redeploy_by_parent(&mouse_response_unit_rect, &actual_area)
            };

            string_represent_vec.push((i, string_component));
        }

        FretboardComponent {
            strings : string_represent_vec,
            frets : fret_represents_vec
        }
    }

    pub fn get_mouse_inside_point_and_rect(&self, mouse_pos : Pos2) -> Option<(Point, Rect)> {
        for (string_num, string) in self.strings() {
            let string_mouse_rect = string.mouse_response_rect;
            if ! string_mouse_rect.contains(mouse_pos) {
                continue;
            }

            for (fret_num, fret) in self.frets() {
                let fret_mouse_rect = fret.mouse_response_rect;
                if ! fret_mouse_rect.contains(mouse_pos) {
                    continue;
                }

                return Some((Point::of(*string_num, *fret_num),
                      string_mouse_rect.intersect(fret_mouse_rect)));
            }
        }
        None
    }

    pub fn draw_fretboard(&self, painter: &Painter, rect: Rect) {
        self.draw_fret_dots(painter, rect, &self.frets);

        // Draw the vertical frets
        for (_, fret) in &self.frets {
            let x_pos = fret.fret_x_pos;
            painter.vline(x_pos, rect.y_range(), Stroke::new(2.0, Color32::GRAY));
        }

        // Draw the horizontal strings
        for (string_num, string) in &self.strings {
            let y_pos = string.center_y_pos;
            let string_stroke = Stroke::new(
                1. + string_num.clone() as f32 * 0.3, // Vary thickness for different strings
                Color32::from_gray(200),
            );
            painter.hline(rect.x_range(), y_pos, string_stroke);
        }
    }

    fn draw_fret_dots(&self, painter: &Painter, rect: Rect, frets : &Vec<(FretNum, FretComponent)>) {
        for (_, fret) in frets {
            match fret.fret_mark {
                Some(FretMark::SingleDot) => {
                    // Draw fret dots at specific positions (e.g., frets 3, 5, 7, 9, 12)
                    let x_center = fret.center_x_pos;
                    let y_center = rect.center().y;
                    painter.circle_filled(Pos2::new(x_center, y_center), 5.0, Color32::from_gray(100));
                },
                Some(FretMark::DoubleDot) => {
                    // Fret 12 often has two dots
                    let x_center = fret.center_x_pos;
                    painter.circle_filled(Pos2::new(x_center, rect.center().y - 20.0), 5.0, Color32::from_gray(100));
                    painter.circle_filled(Pos2::new(x_center, rect.center().y + 20.0), 5.0, Color32::from_gray(100));
                },
                None => continue,
            }
        }
    }
}
fn redeploy_by_parent(cur_rect : &Rect, parent : &Rect) -> Rect {
    let parent_size = parent.size();
    let parent_translate = parent.left_top();
    let x_resize = parent_size.x;
    let y_resize = parent_size.y;
    let x_translate = parent_translate.x;
    let y_translate = parent_translate.y;
    Rect {
        min : Pos2 {
            x: cur_rect.min.x * x_resize + x_translate,
            y: cur_rect.min.y * y_resize + y_translate
        },
        max : Pos2 {
            x: cur_rect.max.x * x_resize + x_translate,
            y: cur_rect.max.y * y_resize + y_translate
        }
    }
}