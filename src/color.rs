use std::fmt;

// color(red, green, blue)
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[macro_export]
macro_rules! new {
    ($red:expr, $green:expr, $blue:expr) => {
        Color {
            red: $red, 
            green: $green, 
            blue: $blue,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

#[allow(dead_code)]
pub const COLOR_BLACK: Color = new!(0, 0, 0);
#[allow(dead_code)]
pub const COLOR_WHITE: Color = new!(255, 255, 255);
#[allow(dead_code)]
pub const COLOR_RED: Color = new!(255, 0, 0);
#[allow(dead_code)]
pub const COLOR_GREEN: Color = new!(0, 255, 0);
#[allow(dead_code)]
pub const COLOR_BLUE: Color = new!(0, 0, 255);
#[allow(dead_code)]
pub const COLOR_TEAL: Color = new!(0, 255, 255);
#[allow(dead_code)]
pub const COLOR_PURPLE: Color = new!(255, 0, 255);
#[allow(dead_code)]
pub const COLOR_YELLOW: Color = new!(255, 255, 0);
