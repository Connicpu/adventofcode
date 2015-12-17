use std::io::{Write, Cursor};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn make_key(in_buffer: &mut [u8], value: u32) -> usize {
    let key = "yzbqklnj";
    let mut cursor = Cursor::new(in_buffer);
    write!(cursor, "{}{}", key, value).unwrap();
    
    cursor.position() as usize
}

pub fn puzzle_1() {
    let mut md5 = Md5::new();
    let mut in_buffer = [0u8; 256];
    let mut out_buffer = [0u8; 16];
    
    for i in 1.. {
        let bytes = make_key(&mut in_buffer, i);
        
        md5.input(&in_buffer[0..bytes]);
        md5.result(&mut out_buffer);
        md5.reset();
        
        if out_buffer[0..2] == [0, 0] && (out_buffer[2] & 0xF0 == 0) {
            println!("Secret number: {}", i);
            break;
        }
    }
}

pub fn puzzle_2() {
    let mut md5 = Md5::new();
    let mut in_buffer = [0u8; 256];
    let mut out_buffer = [0u8; 16];
    
    for i in 1.. {
        let bytes = make_key(&mut in_buffer, i);
        
        md5.input(&in_buffer[0..bytes]);
        md5.result(&mut out_buffer);
        md5.reset();
        
        if out_buffer[0..3] == [0; 3] {
            println!("Secret number: {}", i);
            break;
        }
    }
}

