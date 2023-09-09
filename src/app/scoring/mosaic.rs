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

    /*pub fn pixel_is_valid(&self, x: usize, y: usize, iterations: i8) -> bool {
        let iterations = iterations - 1;


        match self.pixels[x][y].color {
            PixelColor::Blank => {
                false
            }
            PixelColor::White => {
                false
            }
            _ => {
                if iterations < 0 {
                    return true;
                }
                // 3 pixels in a line is always invalid
                if x > 0 && self.pixel_is_valid(x - 1, y, 0) && self.pixel_is_valid(x + 1, y, 0) {
                    false
                } else if y > 0 && self.pixel_is_valid(x, y - 1, 0) && self.pixel_is_valid(x, y + 1, 0) {
                    false
                } else if y > 0 && self.pixel_is_valid(x + 1, y - 1, 0) && self.pixel_is_valid(x + 1, y + 1, 0) {
                    false
                } else {
                    let mut valid_pixels = 1;
                    for pixel in self.get_pixel_neighbors(x, y) {
                        if self.pixel_is_valid(pixel.0, pixel.1, iterations) {
                            println!("Valid pixel: {}, {}", pixel.0, pixel.1);
                            valid_pixels = valid_pixels + 1;
                        }
                    }
                    valid_pixels == 3
                }
            }
        }
    }*/

    pub fn pixel_is_valid(&self, x: usize, y: usize, color: Option<PixelColor>) -> bool {
        match self.pixels[x][y].color {
            PixelColor::Blank => {
                false
            }
            PixelColor::White => {
                false
            }
            c1 => {
                match color {
                    None => {
                        true
                    }
                    Some(c2) => {
                        c1 == c2
                    }
                }
            }
        }
    }

    pub fn get_pixel_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut tmp_pixels = vec![];

        if x > 0 {
            tmp_pixels.push((x - 1, y));
        }
        if y > 0 {
            tmp_pixels.push((x, y - 1));
        }
        if y < self.max_height {
            tmp_pixels.push((x, y + 1));
        }

        match x % 2 {
            0 => {
                if x < self.even_row_count {
                    tmp_pixels.push((x + 1, y));
                    if y > 0 {
                        tmp_pixels.push((x + 1, y - 1));
                    }
                    if y < self.max_height {
                        tmp_pixels.push((x + 1, y + 1));
                    }
                }
            }
            1 => {
                if x < self.odd_row_count {
                    tmp_pixels.push((x + 1, y));
                    if y > 0 {
                        tmp_pixels.push((x + 1, y - 1));
                    }
                    if y < self.max_height {
                        tmp_pixels.push((x + 1, y + 1));
                    }
                }
            }
            _ => {}
        }

        let mut pixels: Vec<(usize, usize)> = vec![];

        for (x, y) in tmp_pixels {
            match self.pixels[x][y].color {
                PixelColor::Blank => {}
                PixelColor::White => {}
                _ => { pixels.push((x, y)) }
            }
        }


        pixels
    }

    pub fn get_mosaic_score(&self) -> u32 {
        let mut score = 0;

        let mut row_width = 0;

        for y in (0..self.max_height).step_by(1) {
            if y % 2 == 0 {
                row_width = self.even_row_count;
            } else {
                row_width = self.odd_row_count;
            }

            for x in 0..row_width {
                match self.pixel_is_valid(x, y, None) {
                    true => {
                        let mut valid_pixels = 1;
                        for (nx, ny) in self.get_pixel_neighbors(x, y) {
                            if self.pixel_is_valid(nx, ny, Some(self.pixels[x][y].color)) {
                                valid_pixels = valid_pixels + 1;
                            }
                        }

                        if valid_pixels == 3 {
                            score += 10;
                        }
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