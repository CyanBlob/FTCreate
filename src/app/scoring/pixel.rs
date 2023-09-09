use hexx::Vec2;
use crate::app::scoring::pixel::PixelColor::*;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub color: PixelColor,
    pub position: Vec2,
}

#[derive(Copy, Clone, Debug)]
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
            position: Vec2::new(-1.0, -1.0),
        }
    }

    pub fn new(position: Vec2) -> Self {
        Pixel {
            color: Blank,
            position: position,
        }
    }

    pub fn add_image(&mut self) {
        match self.color {
            PixelColor::White => {}
            PixelColor::Green => {}
            PixelColor::Purple => {}
            PixelColor::Yellow => {}
            Blank => {}
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