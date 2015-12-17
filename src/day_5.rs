use std::fs;
use std::io::{BufRead, BufReader};

pub fn puzzle_1() {
    let file = fs::File::open("./input/day5.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut nice_strings = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        
        let num_vowels = line.chars().filter(|c| match *c {
            'a'|'e'|'i'|'o'|'u' => true,
            _ => false,
        }).count();
        if num_vowels < 3 { continue };
        
        let mut mean_pairs = false;
        let mut dubs = false;
        for (c1, c2) in line.chars().zip(line.chars().skip(1)) {
            match (c1, c2) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => {
                    mean_pairs = true;
                    break;
                }
                _ => {},
            }
            
            if c1 == c2 {
                dubs = true;
            }
        }
        
        if !mean_pairs && dubs {
            nice_strings += 1;
        }
    }
    
    println!("Nice strings: {}", nice_strings);
}
