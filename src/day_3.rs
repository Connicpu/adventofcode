use std::{mem, fs};
use std::io::Read;
use std::collections::HashSet;

pub fn puzzle_1() {
    let file = fs::File::open("./input/day3.txt").unwrap();
    
    let mut deliveries = HashSet::new();
    let mut pos = (0, 0);
    deliveries.insert(pos);
    
    for c in file.chars() {
        match c {
            Ok('<') => pos.0 -= 1,
            Ok('>') => pos.0 += 1,
            Ok('^') => pos.1 -= 1,
            Ok('v') => pos.1 += 1,
            _ => continue,
        }
        
        deliveries.insert(pos);
    }
    
    println!("Deliveries: {}", deliveries.len());
}

pub fn puzzle_2() {
    let file = fs::File::open("./input/day3.txt").unwrap();
    
    let mut deliveries = HashSet::new();
    let mut pos = (0, 0);
    let mut other_pos = (0, 0);
    deliveries.insert(pos);
    
    for c in file.chars() {
        match c {
            Ok('<') => pos.0 -= 1,
            Ok('>') => pos.0 += 1,
            Ok('^') => pos.1 -= 1,
            Ok('v') => pos.1 += 1,
            _ => continue,
        }
        
        deliveries.insert(pos);
        mem::swap(&mut pos, &mut other_pos);
    }
    
    println!("Deliveries: {}", deliveries.len());
}
