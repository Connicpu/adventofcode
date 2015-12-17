use std::fs;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

pub fn puzzle_1() {
    let file = fs::File::open("./input/day2.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut area = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split('x').collect();
        let (l, w, h): (i32, i32, i32) = match &parts[..] {
            [l, w, h, ..] => (l.parse().unwrap(), w.parse().unwrap(), h.parse().unwrap()),
            _ => continue,
        };
        
        let side1 = l*w;
        let side2 = w*h;
        let side3 = h*l;
        let short = min(side1, min(side2, side3));
        area += 2*side1 + 2*side2 + 2*side3 + short;
    }
    
    println!("Total Wrapping Paper: {}", area);
}

pub fn puzzle_2() {
    let file = fs::File::open("./input/day2.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut ribbon = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split('x').collect();
        let (l, w, h): (i32, i32, i32) = match &parts[..] {
            [l, w, h, ..] => (l.parse().unwrap(), w.parse().unwrap(), h.parse().unwrap()),
            _ => continue,
        };
        
        let long = max(l, max(w, h));
        let (short1, short2) = if long == l {
            (w, h)
        } else if long == w {
            (l, h)
        } else {
            (l, w)
        };
        
        let volume = l*w*h;
        
        ribbon += short1*2 + short2*2 + volume;
    }
    
    println!("Feet of Ribbon: {}", ribbon);
}
