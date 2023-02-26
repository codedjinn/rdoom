pub fn from_4_bytes_to_int(arr_u8: &[u8]) -> u32 {
    if arr_u8.len() != 4 {
        panic!("byte array was not of length 4");
    }

    let x1 = u32::from(arr_u8[0]);
    let x2 = u32::from(arr_u8[1]) << 8;
    let x3 = u32::from(arr_u8[2]) << 16;
    let x4 = u32::from(arr_u8[3]) << 24;

    x1 | x2 | x3 | x4
}

pub fn from_2_bytes_to_int(arr_u8: &[u8]) -> u16 {
    if arr_u8.len() != 2 {
        panic!("byte array was not of length 2");
    }

    let x1 = u16::from(arr_u8[0]);
    let x2 = u16::from(arr_u8[1]) << 8;

    return x1 | x2
}

// TODO: Refactor function not efficient
pub fn from_bytes_to_string(arr_u8: &[u8]) -> String {
    if arr_u8.len() != 8 {
        panic!("byte array was not of length 8");
    }

    let name_from_bytes = String::from_utf8(arr_u8.to_vec())
                                            .expect("Could not parse string!");

    return String::from(name_from_bytes.trim_end_matches(char::from(0)));
}
