pub const FONT: [[u8; 8]; 62] = [
    // Uppercase letters (A-Z)
    [0x7E, 0x11, 0x11, 0x11, 0x7E, 0x00, 0x00, 0x00], // A
    [0x7F, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00, 0x00], // B
    [0x3E, 0x41, 0x41, 0x41, 0x22, 0x00, 0x00, 0x00], // C
    [0x7F, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00, 0x00], // D
    [0x7F, 0x49, 0x49, 0x49, 0x41, 0x00, 0x00, 0x00], // E
    [0x7F, 0x09, 0x09, 0x09, 0x01, 0x00, 0x00, 0x00], // F
    [0x3E, 0x41, 0x41, 0x51, 0x71, 0x00, 0x00, 0x00], // G
    [0x7F, 0x08, 0x08, 0x08, 0x7F, 0x00, 0x00, 0x00], // H
    [0x00, 0x41, 0x7F, 0x41, 0x00, 0x00, 0x00, 0x00], // I
    [0x20, 0x40, 0x41, 0x3F, 0x01, 0x00, 0x00, 0x00], // J
    [0x7F, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00], // K
    [0x7F, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00, 0x00], // L
    [0x7F, 0x02, 0x04, 0x02, 0x7F, 0x00, 0x00, 0x00], // M
    [0x7F, 0x04, 0x08, 0x10, 0x7F, 0x00, 0x00, 0x00], // N
    [0x3E, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00, 0x00], // O
    [0x7F, 0x09, 0x09, 0x09, 0x06, 0x00, 0x00, 0x00], // P
    [0x3E, 0x41, 0x51, 0x21, 0x5E, 0x00, 0x00, 0x00], // Q
    [0x7F, 0x09, 0x19, 0x29, 0x46, 0x00, 0x00, 0x00], // R
    [0x46, 0x49, 0x49, 0x49, 0x31, 0x00, 0x00, 0x00], // S
    [0x01, 0x01, 0x7F, 0x01, 0x01, 0x00, 0x00, 0x00], // T
    [0x3F, 0x40, 0x40, 0x40, 0x3F, 0x00, 0x00, 0x00], // U
    [0x1F, 0x20, 0x40, 0x20, 0x1F, 0x00, 0x00, 0x00], // V
    [0x7F, 0x20, 0x18, 0x20, 0x7F, 0x00, 0x00, 0x00], // W
    [0x77, 0x08, 0x7F, 0x08, 0x77, 0x00, 0x00, 0x00], // X
    [0x07, 0x08, 0x7F, 0x08, 0x07, 0x00, 0x00, 0x00], // Y
    [0x71, 0x49, 0x45, 0x43, 0x00, 0x00, 0x00, 0x00], // Z

    // Digits (0-9)
    [0x3E, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00, 0x00], // 0
    [0x00, 0x42, 0x7F, 0x40, 0x00, 0x00, 0x00, 0x00], // 1
    [0x42, 0x61, 0x51, 0x49, 0x46, 0x00, 0x00, 0x00], // 2
    [0x21, 0x41, 0x45, 0x4B, 0x31, 0x00, 0x00, 0x00], // 3
    [0x18, 0x14, 0x12, 0x7F, 0x10, 0x00, 0x00, 0x00], // 4
    [0x27, 0x45, 0x45, 0x45, 0x39, 0x00, 0x00, 0x00], // 5
    [0x3C, 0x4A, 0x49, 0x49, 0x30, 0x00, 0x00, 0x00], // 6
    [0x01, 0x71, 0x09, 0x05, 0x03, 0x00, 0x00, 0x00], // 7
    [0x36, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00, 0x00], // 8
    [0x06, 0x49, 0x49, 0x29, 0x1E, 0x00, 0x00, 0x00], // 9

    // Lowercase letters (a-z)
    [0x3E, 0x51, 0x49, 0x49, 0x4E, 0x00, 0x00, 0x00], // a
    [0x7F, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00, 0x00], // b
    [0x3E, 0x41, 0x41, 0x41, 0x22, 0x00, 0x00, 0x00], // c
    [0x7F, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00, 0x00], // d
    [0x7F, 0x49, 0x49, 0x49, 0x41, 0x00, 0x00, 0x00], // e
    [0x7F, 0x09, 0x09, 0x09, 0x01, 0x00, 0x00, 0x00], // f
    [0x3E, 0x41, 0x41, 0x51, 0x71, 0x00, 0x00, 0x00], // g
    [0x7F, 0x08, 0x08, 0x08, 0x7F, 0x00, 0x00, 0x00], // h
    [0x00, 0x41, 0x7F, 0x41, 0x00, 0x00, 0x00, 0x00], // i
    [0x20, 0x40, 0x41, 0x3F, 0x01, 0x00, 0x00, 0x00], // j
    [0x7F, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00, 0x00], // k
    [0x7F, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00, 0x00], // l
    [0x7F, 0x02, 0x04, 0x02, 0x7F, 0x00, 0x00, 0x00], // m
    [0x7F, 0x04, 0x08, 0x10, 0x7F, 0x00, 0x00, 0x00], // n
    [0x3E, 0x41, 0x41, 0x41, 0x3E, 0x00, 0x00, 0x00], // o
    [0x7F, 0x09, 0x09, 0x09, 0x06, 0x00, 0x00, 0x00], // p
    [0x3E, 0x41, 0x51, 0x21, 0x5E, 0x00, 0x00, 0x00], // q
    [0x7F, 0x09, 0x19, 0x29, 0x46, 0x00, 0x00, 0x00], // r
    [0x46, 0x49, 0x49, 0x49, 0x31, 0x00, 0x00, 0x00], // s
    [0x01, 0x01, 0x7F, 0x01, 0x01, 0x00, 0x00, 0x00], // t
    [0x3F, 0x40, 0x40, 0x40, 0x3F, 0x00, 0x00, 0x00], // u
    [0x1F, 0x20, 0x40, 0x20, 0x1F, 0x00, 0x00, 0x00], // v
    [0x7F, 0x20, 0x18, 0x20, 0x7F, 0x00, 0x00, 0x00], // w
    [0x77, 0x08, 0x7F, 0x08, 0x77, 0x00, 0x00, 0x00], // x
    [0x07, 0x08, 0x7F, 0x08, 0x07, 0x00, 0x00, 0x00], // y
    [0x71, 0x49, 0x45, 0x43, 0x00, 0x00, 0x00, 0x00], // z
];

pub fn get_char_bitmap(c: char) -> Option<[u8; 8]> {
    let index = match c {
        'A'..='Z' => c as usize - 'A' as usize,
        '0'..='9' => c as usize - '0' as usize + 26,
        'a'..='z' => c as usize - 'a' as usize + 36,
        _ => return None,
    };

    Some(FONT[index])
}