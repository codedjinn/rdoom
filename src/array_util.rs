
// There isn't really concept of 2d arrays in Rust, so helper
// methods to utilize flat array as 2d

pub fn create_array_u32(width: usize, height: usize) -> Vec<u32> {
    return Vec::with_capacity(width * height);
}

pub fn get_index(width: u32, x: u32, y: u32) -> usize {
    return width as usize * y as usize + x as usize;
}