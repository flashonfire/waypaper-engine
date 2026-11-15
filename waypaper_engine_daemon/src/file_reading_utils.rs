use std::io::{BufRead, Cursor, Read};

pub fn read_u32(cursor: &mut Cursor<Vec<u8>>) -> u32 {
    let position = cursor.position();
    let mut first_4_bytes: [u8; 4] = [0; 4];
    cursor
        .read_exact(&mut first_4_bytes)
        .unwrap_or_else(|e| panic!("Failed to read u32 at byte {position}: {e}"));

    u32::from_le_bytes(first_4_bytes)
}

pub fn read_i32(cursor: &mut Cursor<Vec<u8>>) -> i32 {
    let position = cursor.position();
    let mut first_4_bytes: [u8; 4] = [0; 4];
    cursor
        .read_exact(&mut first_4_bytes)
        .unwrap_or_else(|e| panic!("Failed to read i32 at byte {position}: {e}"));

    i32::from_le_bytes(first_4_bytes)
}

pub fn read_f32(cursor: &mut Cursor<Vec<u8>>) -> f32 {
    let position = cursor.position();
    let mut first_4_bytes: [u8; 4] = [0; 4];
    cursor
        .read_exact(&mut first_4_bytes)
        .unwrap_or_else(|e| panic!("Failed to read f32 at byte {position}: {e}"));

    f32::from_le_bytes(first_4_bytes)
}

pub fn read_null_terminated_str(cursor: &mut Cursor<Vec<u8>>) -> String {
    let position = cursor.position();
    let mut bytes = vec![];

    cursor.read_until(0x00, &mut bytes).unwrap();
    assert_eq!(
        bytes.pop(), // Remove the null terminator
        Some(0x00),
        "Unterminated string at byte {position}"
    );

    String::from_utf8(bytes)
        .unwrap_or_else(|e| panic!("String at byte {position} is not valid utf-8: {e}"))
}

pub fn read_str(data: &mut Cursor<Vec<u8>>) -> String {
    let size = read_u32(data);
    read_sized_str(data, size)
}

fn read_sized_str(data: &mut Cursor<Vec<u8>>, size: u32) -> String {
    let position = data.position();
    let mut bytes = vec![];
    data.take(u64::from(size)).read_to_end(&mut bytes).unwrap();
    assert_eq!(
        bytes.len(),
        size as usize,
        "Truncated string at byte {position}: expected {size} bytes, only {} left",
        bytes.len()
    );

    String::from_utf8(bytes)
        .unwrap_or_else(|e| panic!("String at byte {position} is not valid UTF-8: {e}"))
}

pub(crate) fn read_color(data: &mut Cursor<Vec<u8>>) -> (u8, u8, u8, u8) {
    let number = read_u32(data);

    number.to_le_bytes().into()
}
