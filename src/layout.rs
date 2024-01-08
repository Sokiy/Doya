use crate::define;
use fltk::enums::{Color, FrameType};
use fltk::{prelude::*, window::Window};

pub fn set_wind_style(wind: &mut Window) {
    let color = Color::from_hex_str(define::color::BASE_BACKGROUND_COLOR).unwrap();
    wind.set_frame(FrameType::FlatBox);
    wind.set_color(color);
}
