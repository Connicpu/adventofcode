use std::fs;
use std::io::Read;

pub fn puzzle_1() {
    let file = fs::File::open("./input/day1.txt").unwrap();
    
    let mut floor = 0;
    for c in file.chars() {
        match c {
            Ok('(') => floor += 1,
            Ok(')') => floor -= 1,
            _ => {},
        }
    }
    
    println!("Floor {}", floor);
}

pub fn puzzle_2() {
    let file = fs::File::open("./input/day1.txt").unwrap();
    
    let mut pos = 0;
    let mut floor = 0;
    for c in file.chars() {
        pos += 1;
        match c {
            Ok('(') => floor += 1,
            Ok(')') => floor -= 1,
            _ => {},
        }
        
        if floor < 0 {
            break;
        }
    }
    
    println!("Position {}", pos);
}
