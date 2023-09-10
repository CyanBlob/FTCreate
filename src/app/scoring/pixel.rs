use crate::app::scoring::pixel::PixelColor::*;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub color: PixelColor,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PixelColor {
    White,
    Green,
    Purple,
    Yellow,
    Blank,
}

impl Pixel {
    pub fn default() -> Self {
        Pixel {
            color: Blank,
        }
    }

    pub fn inc_color(&mut self) {
        self.color = match ((self.color as u32) + 1) % 5 {
            0 => { White }
            1 => { Green }
            2 => { Purple }
            3 => { Yellow }
            4 => { Blank }
            _ => { Blank }
        }
    }
}