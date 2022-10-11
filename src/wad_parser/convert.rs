#![allow(dead_code)]

pub fn u8x4_to_u32(u8s: [u8; 4]) -> u32 {
    let p1 = u32::from(u8s[0]);
    let p2 = u32::from(u8s[1]) << 8;
    let p3 = u32::from(u8s[2]) << 16;
    let p4 = u32::from(u8s[3]) << 24;

    p1 | p2 | p3 | p4
}

pub fn u8ref2_to_u32(u8s: &[u8]) -> u32 {
    if u8s.len() != 2 {
        panic!("the u8 array provided to u8ref_to_u32 doesn't have the correct length (4)");
    }

    let p1 = u32::from(u8s[0]);
    let p2 = u32::from(u8s[1]) << 8;

    p1 | p2
}

pub fn u8ref_to_u32(u8s: &[u8]) -> u32 {
    if u8s.len() != 4 {
        panic!("the u8 array provided to u8ref_to_u32 doesn't have the correct length (4)");
    }

    let p1 = u32::from(u8s[0]);
    let p2 = u32::from(u8s[1]) << 8;
    let p3 = u32::from(u8s[2]) << 16;
    let p4 = u32::from(u8s[3]) << 24;

    p1 | p2 | p3 | p4
}

pub fn u8ref_to_u32_2(array: &[u8], index: usize) -> u32 {
    if index + 2 > array.len() {
        panic!("Supplied 'index' should be less that 2 of length of array");
    }

    let p1 = u32::from(array[index]);
    let p2 = u32::from(array[index + 1]);

    p1 | p2
}

pub fn u8ref_to_u32_4(array: &[u8], index: usize) -> u32 {
    if index + 4 > array.len() {
        panic!("Supplied 'index' should be less that 2 of length of array");
    }

    let p1 = u32::from(array[index]);
    let p2 = u32::from(array[index] + 1);
    let p3 = u32::from(array[index] + 2);
    let p4 = u32::from(array[index] + 3);
    
    p1 | p2 | p3 | p4
}
