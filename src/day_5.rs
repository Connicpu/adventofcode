use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

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

pub fn puzzle_2() {
    let file = fs::File::open("./input/day5.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut nice_strings = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        
        let mut word_pairs: HashMap<(char, char), usize> = HashMap::new();
        let mut double_pair = false;
        for (i, pair) in line.chars().zip(line.chars().skip(1)).enumerate() {
            match word_pairs.entry(pair) {
                Entry::Vacant(entry) => { entry.insert(i); },
                Entry::Occupied(entry) => if *entry.get() < i - 1 {
                    double_pair = true;
                    break;
                }
            }
        }
        if !double_pair {
            continue;
        }
        
        let mut triples = false;
        for (c1, c2) in line.chars().zip(line.chars().skip(2)) {
            if c1 == c2 {
                triples = true;
                break;
            }
        }
        if !triples {
            continue;
        }
        
        nice_strings += 1;
    }
    
    println!("Nice strings: {}", nice_strings);
}
