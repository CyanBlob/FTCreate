use hexx::*;
use crate::app::scoring::pixel::PixelColor;
use super::pixel::Pixel;

const MAX_HEIGHT: usize = 11;
const SPACING: usize = 1;

pub struct Mosaic {
    pub even_row_count: usize,
    pub odd_row_count: usize,
    pub max_height: usize,
    pub pixels: [[Pixel; MAX_HEIGHT]; 7],
    pub auton_pixels: [[Pixel; MAX_HEIGHT]; 7],
    pub layout: HexLayout,
}

impl Mosaic {
    pub fn default() -> Self {
        let layout = HexLayout {
            hex_size: Vec2::new(1.0, 1.0),
            orientation: HexOrientation::Flat,
            ..Default::default()
        };
        // Get the hex coordinate at the world position `world_pos`.
        let world_pos = Vec2::new(SPACING as f32, SPACING as f32);

        //let point = layout.world_pos_to_hex(world_pos);

        // Get the world position of `point`
        //let point = hex(123, 45);
        //let world_pos = layout.hex_to_world_pos(point);

        let mut mosaic = Mosaic {
            even_row_count: 6,
            odd_row_count: 7,
            max_height: MAX_HEIGHT,
            pixels: [[Pixel::default(); MAX_HEIGHT]; 7],
            auton_pixels: [[Pixel::default(); MAX_HEIGHT]; 7],
            layout: layout,
        };

        mosaic.clear();

        mosaic
    }

    pub fn get_score(&mut self) -> u32 {
        self.get_teleop_score() + self.get_auton_score()
    }

    pub fn get_teleop_score(&mut self) -> u32 {
        self.teleop_raw_pixel_count() + self.get_mosaic_score()
    }

    pub fn get_auton_score(&mut self) -> u32 {
        self.auton_raw_pixel_count()
    }

    pub fn pixel_is_valid(&self, x: usize, y: usize) -> bool {
        match self.pixels[x][y].color {
            PixelColor::Blank => {
                false
            }
            PixelColor::White => {
                false
            }
            _ => { true }
        }
    }

    pub fn get_pixel_neighbors(&self, x: usize, y: usize) -> Vec<Pixel> {
        vec![]
    }

    pub fn get_mosaic_score(&self) -> u32 {
        let mut score = 0;

        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                match self.pixel_is_valid(x, y) {
                    true => {
                        score += 10;
                    }
                    false => {}
                }
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                match self.pixel_is_valid(x, y) {
                    true => {
                        score += 10;
                    }
                    false => {}
                }
            }
        }
        score
    }

    fn teleop_raw_pixel_count(&self) -> u32 {
        let mut score = 0;

        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                match self.pixels[x][y].color {
                    PixelColor::Blank => {}
                    _ => { score = score + 3; }
                }
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                match self.pixels[x][y].color {
                    PixelColor::Blank => {}
                    _ => { score = score + 3; }
                }
            }
        }

        score
    }

    // TODO: Validate auton rules
    fn auton_raw_pixel_count(&self) -> u32 {
        let mut score = 0;

        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                match self.pixels[x][y].color {
                    PixelColor::Blank => {}
                    _ => { score = score + 0; }
                }
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                match self.pixels[x][y].color {
                    PixelColor::Blank => {}
                    _ => { score = score + 0; }
                }
            }
        }

        score
    }

    pub fn copy_auton_to_teleop(&mut self) {
        self.pixels = self.auton_pixels;
    }

    pub fn copy_teleop_to_auton(&mut self) {
        self.auton_pixels = self.pixels;
    }

    pub fn clear(&mut self) {
        self.clear_teleop();
        self.clear_auton();
    }

    pub fn clear_auton(&mut self) {
        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                self.auton_pixels[x][y] = Pixel::new(Vec2::new((x * SPACING) as f32, (y * SPACING) as f32));
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                self.auton_pixels[x][y] = Pixel::new(Vec2::new((x * SPACING) as f32 + SPACING as f32 / 2.0, (y * SPACING) as f32));
            }
        }
    }

    pub fn clear_teleop(&mut self) {
        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                self.pixels[x][y] = Pixel::new(Vec2::new((x * SPACING) as f32, (y * SPACING) as f32));
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                self.pixels[x][y] = Pixel::new(Vec2::new((x * SPACING) as f32 + SPACING as f32 / 2.0, (y * SPACING) as f32));
            }
        }
    }
}