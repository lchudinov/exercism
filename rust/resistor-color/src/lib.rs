use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(color) => color_to_string(color),
        Err(_) => String::from("value out of range"),
    }
}

fn color_to_string(color: ResistorColor) -> String {
    match color {
        ResistorColor::Black => String::from("Black"),
        ResistorColor::Blue => String::from("Blue"),
        ResistorColor::Brown => String::from("Brown"),
        ResistorColor::Green => String::from("Green"),
        ResistorColor::Grey => String::from("Grey"),
        ResistorColor::Orange => String::from("Orange"),
        ResistorColor::Red => String::from("Red"),
        ResistorColor::Violet => String::from("Violet"),
        ResistorColor::White => String::from("White"),
        ResistorColor::Yellow => String::from("Yellow"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
