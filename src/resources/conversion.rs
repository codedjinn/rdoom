use crate::wad_parser;

#[derive(Debug, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn from(r:u8, g:u8, b:u8) -> Self {
        Color {
            r,g,b
        }
    }

    pub fn max() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255
        }
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b
        }
    }
}

struct Palette {
    data: Vec<Color>
}

impl Palette {
    pub fn new() -> Self {
        Palette { data: Vec::new() }
    }

    pub fn set(&mut self, data: &[u8]) {
        self.data.clear();

        let mut index = 0usize;
        while index < data.len() {
            let r = data[index];
            let g = data[index + 1];
            let b = data[index + 2];
            self.data.push(Color::from(r, g, b));
            index = index + 3;
        }
    }

    pub fn get(&self, index:usize) -> Color {
        if index > self.data.len() {
            panic!("Supplied index was out of bounds. {}", index);
        }
        self.data[index]
    }
}

struct Picture {
    name: String,
    width: u32,
    height: u32,
    left: u32,
    top: u32,
    pixels: Vec<Color>
}

impl Picture {
    pub fn create_from_resource(name: &str, palette: &Palette, wad_data: &[u8]) -> Self {
        let width = wad_parser::convert::u8ref2_to_u32(&wad_data[0..2]);
        let height = wad_parser::convert::u8ref2_to_u32(&wad_data[2..4]);
        let left = wad_parser::convert::u8ref2_to_u32(&wad_data[4..6]);
        let top = wad_parser::convert::u8ref2_to_u32(&wad_data[6..8]);

        let size = (width * height) as usize;
        let mut pixels: Vec<Color> = Vec::with_capacity(size);
        let mut pixel_data: Vec<u8> = Vec::with_capacity(size);
        for _ in 0..size {
            pixel_data.push(255);
            pixels.push(Color::max());
        }

        for col in 0..width {
            let pointer_index = ((col * 4) + 8) as usize;
            let mut pointer = wad_parser::convert::u8ref_to_u32(&wad_data[pointer_index..pointer_index+4]) as usize;

            loop {
                let row = wad_data[pointer];

                pointer = pointer + 1;
                let post_height = wad_data[pointer];

                if row != 255 && post_height != 255 {
                    pointer = pointer + 1;

                    for i in 0..post_height {
                        if row + i < height as u8 && pointer < wad_data.len() - 1 {

                            pointer = pointer + 1;

                            let pixel_index = (row as u32 + i as u32) * width as u32 + col as u32;
                            pixel_data[pixel_index as usize] = wad_data[pointer];
                        }
                    }

                    pointer = pointer + 1;
                }
                else {
                    break;
                }
                if pointer < wad_data.len() - 1 {
                    break;
                }
                pointer = pointer + 1;
                if wad_data[pointer] != 255 {
                    break;
                }
            }

           let mut bmp = bmp::Image::new(width, height);

            for y in 0..height {
                for x in 0..width {
                    let index = ((y * width) + x) as usize;
                    let palette_index = pixel_data[index];
                    if palette_index == 255 {
                        continue;
                    }
                    let value = palette.data[palette_index as usize];
                    pixels[index] = value;
                    bmp.set_pixel(x, y, bmp::Pixel::new(value.r, value.g, value.b));
                }
            }
            bmp.save("d:\\test.bmp");
        }
        return Picture {
            name: String::from(name),
            width, height,
            top, left,
            pixels: pixels
        };
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        let index = (y * self.width + x) as usize;
        if index > self.pixels.len() {
            panic!("Coordinates was out of bounds, '{}' x - {} y - {}, image size - {}", self.name, x, y, self.pixels.len());
        }
        return self.pixels[index];
    }
}

pub struct ConvertedResources {
    palette_loaded: bool,
    palette: Palette,
    pictures: Vec<Picture>,
}

impl ConvertedResources {
    pub fn new() -> Self {
        ConvertedResources {
            palette_loaded: false,
            palette: Palette::new(),
            pictures: Vec::new()
        }
    }

    pub fn is_palette_loaded(&self) -> bool {
        self.palette_loaded
    }

    pub fn set_palette(&mut self, data: &[u8]) {
        if self.palette_loaded {
            return;
        }
        self.palette.set(data);
        self.palette_loaded = true;
    }

    pub fn add_picture(&mut self, name: &str, wad_data: &[u8]) {
        self.pictures.push(Picture::create_from_resource(name, &self.palette, wad_data));
    }

    pub fn get_picture(&self, name: &str) -> Option<&Picture> {
        return self.pictures.iter().find(|&x| x.name == name);
    }
}