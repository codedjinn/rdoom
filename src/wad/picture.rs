use array2d::{Array2D, Error};
use super::{WadColor, WadPalette};
use super::{util};

pub struct WadPicture {
    width: u32,
    height: u32,
    x_offset: u32,
    y_offset: u32,
    pixels: Option<Array2D<WadColor>>
}

impl WadPicture {
    // pub fn save(&self, filename: &str) {

    //     //let path = std::path::Path::new(filename);

    //     let img = bmp::Image::new(self.width, self.height);

    //     for y in 0..self.pixels.unwrap().num_rows() {
    //         for x in 0..self.pixels.unwrap().num_columns() {
    //             let value = self.pixels[(x,y)];
    //             img.set_pixel(x, y, self.pixels[(x,y)]);
    //         }
    //     }
        
    // }
}

pub fn parse_picture(data: &[u8], palette: &WadPalette) -> WadPicture {
    let width = data[0] | data[1];
    let height = data[2] | data[3];
    let x_offset = data[4] | data[5];
    let y_offset = data[6] | data[7];

    let mut buffer = Vec::new();

    let size = width as usize * height as usize;
    
    for _ in 0..size {
        buffer.push(255);
    }

    for column in 0..width {
        let index = (column as usize * 4usize) + 8usize;
        
        let mut pointer = util::from_4_bytes_to_int(&data[index..index + 4usize]) as usize;

        loop {
            let row_start = data[pointer];
            if row_start == 255 {
                break;
            }

            pointer = pointer + 1;

            let post_height = data[pointer];
            if post_height == 255 {
                break;
            }

            // Skip dummy value
            pointer = pointer + 1;

            for i in 0..post_height as usize {
                if row_start as usize + i < height as usize && pointer < data.len() - 1 {
                    pointer = pointer + 1;

                    let buffer_index = ((row_start as usize + i) * width as usize) + column as usize;
                    buffer[buffer_index] = data[pointer];
                }
            }

            // Unused
            pointer = pointer + 1;

            if pointer > data.len() - 1 {
                break;
            }
            pointer = pointer + 1;
            if data[pointer] == 255 {
                break;
            }
        }                
    }

    let mut bitmap = Array2D::filled_with(WadColor::transparent(), height as usize, width as usize);

    let mut i = 0;
    for y in 0..height as usize {
        for x in 0..width as usize {
            let pal_index = buffer[i] as usize;
            
            if pal_index == 255 {
                i = i + 1;
                continue;
            }

            let color = &palette.colors[pal_index];
            bitmap.set(y, x, WadColor { r: color.r, g: color.g, b: color.b});

            i = i + 1;
        }
    }

    println!("width,height: {},{}", width, height);

    return WadPicture {
        width: width as u32, 
        height: height as u32,
        x_offset: x_offset as u32,
        y_offset: y_offset as u32,
        pixels: Some(bitmap)
    }
}