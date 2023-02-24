// #[derive(Clone)]
// pub struct ArrInt<T: Default + Clone> {
//     buffer: Vec<T>,
//     width: u8,
//     height: u8,
//     size: usize
// }

// impl ArrInt<T> where T: Default + Clone {
//     pub fn new(width: u8, height: u8, value: T) -> Self {
//         let size = (width * height) as usize;
//         let buffer:Vec<T> = vec![T::default() ; size];
//         ArrInt {
//             width,
//             height,
//             size,
//             buffer
//         }
//     }

//     pub fn get(&self, x:u8, y:u8) -> &u8 {
//         if x > self.width - 1 || y > self.height - 1 {
//             panic!("Out of bounds");
//         }
//         let index = ArrInt::get_index(self.width, x, y);
//         return &self.buffer[index];
//     }

//     pub fn set(&mut self, x:u8, y:u8, value:u8) {
//         if x > self.width - 1 || y > self.height - 1 {
//             panic!("Out of bounds");
//         }
//         let index = ArrInt::get_index(self.width, x, y);
//         self.buffer[index] = value;
//     }

//     pub fn print(&self) {
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 let index = ArrInt::get_index(self.width, x, y);
//                 print!("{}", self.buffer[index]);
//             }
//             println!();
//         }
//     }

//     fn get_index(width:u8, x: u8, y: u8) -> usize {
//         return width as usize * y as usize + x as usize;
//     }
// }