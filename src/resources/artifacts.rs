

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

    pub fn from(palette_data: &[u8]) -> Self {
        let mut data: Vec<Color> = Vec::new();

        let mut index = 0usize;
        while index < palette_data.len() {
            let r = palette_data[index];
            let g = palette_data[index + 1];
            let b = palette_data[index + 2];
            data.push(Color::from(r, g, b));
            index = index + 3;
        }

        return Palette {
            data
        };
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
        if index < 0 || index > self.data.len() {
            panic!("Supplied index was out of bounds. {}", index);
        }
        self.data[index]
    }

    fn create_from_data(data: &[u8]) -> Vec<Color> {
        let mut result: Vec<Color> = Vec::new();

        let mut index = 0usize;
        while index < data.len() {
            let r = data[index];
            let g = data[index + 1];
            let b = data[index + 2];
            result.push(Color::from(r, g, b));
            index = index + 3;
        }
        return result;
    }
}

struct Picture {
    name: String,
    width: u8,
    height: u8,
    left: u8,
    top: u8,
    pixels: Vec<Color>
}

struct WadResources {
    palette: Palette,
    pictures: Vec<Picture>
}

impl WadResources {
    pub fn new() -> Self {
        WadResources { 
            palette: Palette::new(), 
            pictures: Vec::new() 
        }
    }

    pub fn set_palette(&mut self, data: &[u8]) {
        self.palette.set(data);
    }

    pub fn add_picture(&mut self, name: &str, width: u8, height: u8, top: u8, left: u8, data: &[u8]) {
        let picture = Picture {
            name: String::from(name),
            width, height,
            top, left,
            pixels: Vec::new()
        };
        self.pictures.push(picture);
    }
}