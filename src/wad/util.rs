pub fn byte_array_4_to_int(arr_u8: &[u8]) -> u32 {
    if arr_u8.len() != 4 {
        panic!("byte array was not of length 4");
    }

    let x1 = u32::from(arr_u8[0]);
    let x2 = u32::from(arr_u8[1]) << 8;
    let x3 = u32::from(arr_u8[2]) << 16;
    let x4 = u32::from(arr_u8[3]) << 24;

    x1 | x2 | x3 | x4
}