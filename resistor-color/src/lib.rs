use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, IntEnum, Copy, Clone, IntoEnumIterator)]
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

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    if value > 10 {
        return String::from("value out of range");
    }

    match ResistorColor::from_int(value) {
        Ok(a) => format!("{:?}", a),
        _ => panic!("i don't know what is this"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let iter = ResistorColor::into_enum_iter();
    let mut vec = Vec::with_capacity(iter.len());
    for a in iter {
        vec.push(a);
    }
    vec
}
