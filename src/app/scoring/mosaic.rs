use crate::app::scoring::pixel::PixelColor;
use super::pixel::Pixel;

const MAX_HEIGHT: usize = 11;

pub struct Mosaic {
    pub even_row_count: usize,
    pub odd_row_count: usize,
    pub max_height: usize,
    pub pixels: [[Pixel; MAX_HEIGHT]; 7],
    pub auton_pixels: [[Pixel; MAX_HEIGHT]; 7],
}

impl Mosaic {
    pub fn default() -> Self {
        let mut mosaic = Mosaic {
            even_row_count: 6,
            odd_row_count: 7,
            max_height: MAX_HEIGHT,
            pixels: [[Pixel::default(); MAX_HEIGHT]; 7],
            auton_pixels: [[Pixel::default(); MAX_HEIGHT]; 7],
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

    pub fn pixel_is_valid(&self, x: usize, y: usize, color: Option<PixelColor>, used_colors: &Vec<PixelColor>) -> bool {
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
                        let mut c1_count = 0;
                        let mut other_count = 0;

                        //let _ = used_colors.iter().map(|c| );
                        for c in used_colors {
                            if c == &c1 {
                                c1_count += 1;
                            } else {
                                other_count += 1;
                            }
                        }

                        if used_colors.len() == 0 {
                            return true;
                        }

                        if c1 == c2 && other_count == 0 {
                            return true;
                        }

                        if (!used_colors.contains(&c2)) && c1_count == 0 {
                            return true;
                        }

                        false
                    }
                }
            }
        }
    }

    pub fn get_pixel_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut tmp_pixels = vec![];

        // left
        if x > 0 {
            tmp_pixels.push((x - 1, y));
        }

        // right
        if x < self.odd_row_count - 1 {
            tmp_pixels.push((x + 1, y));
        }

        match y % 2 {
            0 => {
                if x < self.odd_row_count - 1 {
                    // bottom right
                    if y > 0 {
                        tmp_pixels.push((x + 1, y - 1));
                    }
                    // top right
                    if y < self.max_height - 1 {
                        tmp_pixels.push((x + 1, y + 1));
                    }
                    // bottom left
                    if y > 0 {
                        tmp_pixels.push((x, y - 1));
                    }

                    // top left
                    if y < self.max_height - 1 {
                        tmp_pixels.push((x, y + 1));
                    }
                }
            }
            1 => {
                if x < self.odd_row_count {
                    // bottom right
                    if y > 0 {
                        tmp_pixels.push((x, y - 1));
                    }
                    // top right
                    if y < self.max_height - 1 {
                        tmp_pixels.push((x, y + 1));
                    }
                    if x > 0 {
                        // top left
                        if y < self.max_height - 1 {
                            tmp_pixels.push((x - 1, y + 1));
                        }
                        // bottom left
                        if y > 0 {
                            tmp_pixels.push((x - 1, y - 1));
                        }
                    }
                }
            }
            _ => {}
        }

        let mut pixels: Vec<(usize, usize)> = vec![];
        let mut colors = vec![];

        for (x, y) in tmp_pixels {
            colors.push(self.pixels[x][y].color);

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

        let mut row_width;

        let mut used_pixels: Vec<(usize, usize)> = vec![];

        for y in (0..self.max_height).step_by(1) {
            if y % 2 == 0 {
                row_width = self.even_row_count;
            } else {
                row_width = self.odd_row_count;
            }

            for x in 0..row_width {
                match self.pixel_is_valid(x, y, None, &vec![]) {
                    true => {
                        let mut valid_pixels = 0;
                        let mut neighbors = self.get_pixel_neighbors(x, y);

                        neighbors.push((x, y));

                        let mut used_colors = vec![];

                        for i in 0..neighbors.len() {
                            let (nx, ny) = neighbors[i];
                            if used_pixels.contains(&(nx, ny)) {
                                valid_pixels = 10;
                            }

                            let neighbor_count = self.get_pixel_neighbors(nx, ny).len();

                            if neighbor_count > 2 {
                                valid_pixels = 10;
                            }

                            if neighbor_count == 2 && self.pixel_is_valid(nx, ny, Some(self.pixels[x][y].color), &used_colors) {
                                used_colors.push(self.pixels[nx][ny].color);
                                valid_pixels = valid_pixels + 1;
                            }
                        }

                        if valid_pixels == 3 {
                            for i in 0..3 {
                                used_pixels.push(neighbors.clone().to_owned().iter().nth(i).unwrap().clone().to_owned());
                            }

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
                self.auton_pixels[x][y] = Pixel::default();
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                self.auton_pixels[x][y] = Pixel::default();
            }
        }
    }

    pub fn clear_teleop(&mut self) {
        for y in (0..self.max_height).step_by(2) {
            for x in 0..self.even_row_count {
                self.pixels[x][y] = Pixel::default();
            }
        }
        for y in (1..self.max_height).step_by(2) {
            for x in 0..self.odd_row_count {
                self.pixels[x][y] = Pixel::default();
            }
        }
    }
}