use std::fs;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

type LightOp = fn(&mut Light, &mut usize);
type Grid = [[Light; 1000]; 1000];
type Coord = (usize, usize);
struct State(usize, Grid);

#[derive(Copy, Clone, Debug)]
struct Light(u8);

impl Light {
    fn turn_on(&mut self, count: &mut usize) {
        match self.0 {
            0 => *count += 1,
            _ => {},
        }
        self.0 = 1;
    }
    
    fn turn_off(&mut self, count: &mut usize) {
        match self.0 {
            1 => *count -= 1,
            _ => {},
        }
        self.0 = 0;
    }
    
    fn toggle(&mut self, count: &mut usize) {
        self.0 = match self.0 {
            0 => {
                *count += 1;
                1
            },
            _ => {
                *count -= 1;
                0
            },
        }
    }
    
    fn turn_up(&mut self, count: &mut usize) {
        self.0 += 1;
        *count += 1;
    }
    
    fn turn_up2(&mut self, count: &mut usize) {
        self.0 += 2;
        *count += 2;
    }
    
    fn turn_down(&mut self, count: &mut usize) {
        self.0 = match self.0 {
            0 => 0,
            i => {
                *count -= 1;
                i - 1
            },
        };
    }
}

fn execute(state: &mut State, p1: Coord, p2: Coord, op: LightOp) {
    let start_x = min(p1.0, p2.0);
    let start_y = min(p1.1, p2.1);
    let end_x = max(p1.0, p2.0) + 1;
    let end_y = max(p1.1, p2.1) + 1;
    
    for x in start_x..end_x {
        for y in start_y..end_y {
            op(&mut state.1[x][y], &mut state.0);
        }
    }
}

fn parse_coord(text: &str) -> Option<Coord> {
    let mut split = text.split(',');
    match (split.next(), split.next()) {
        (Some(x), Some(y)) => match str::parse::<usize>(x) {
            Ok(x) => match str::parse::<usize>(y) {
                Ok(y) => Some((x, y)),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

pub fn puzzle_1() {
    let file = fs::File::open("./input/day6.txt").unwrap();
    let reader = BufReader::new(file);
    let mut state = State(0, [[Light(0); 1000]; 1000]);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        
        let op: LightOp = match split.next() {
            Some("turn") => match split.next() {
                Some("on") => Light::turn_on,
                Some("off") => Light::turn_off,
                _ => continue,
            },
            Some("toggle") => Light::toggle,
            _ => continue,
        };
        
        let p1 = match split.next() {
            Some(text) => match parse_coord(text) {
                Some(coord) => coord,
                None => continue,
            },
            None => continue,
        };
        
        match split.next() {
            Some("through") => {},
            _ => continue,
        }
        
        let p2 = match split.next() {
            Some(text) => match parse_coord(text) {
                Some(coord) => coord,
                None => continue,
            },
            None => continue,
        };
        
        execute(&mut state, p1, p2, op);
    }
    
    println!("Enabled lights: {}", state.0);
}

pub fn puzzle_2() {
    let file = fs::File::open("./input/day6.txt").unwrap();
    let reader = BufReader::new(file);
    let mut state = State(0, [[Light(0); 1000]; 1000]);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        
        let op: LightOp = match split.next() {
            Some("turn") => match split.next() {
                Some("on") => Light::turn_up,
                Some("off") => Light::turn_down,
                _ => continue,
            },
            Some("toggle") => Light::turn_up2,
            _ => continue,
        };
        
        let p1 = match split.next() {
            Some(text) => match parse_coord(text) {
                Some(coord) => coord,
                None => continue,
            },
            None => continue,
        };
        
        match split.next() {
            Some("through") => {},
            _ => continue,
        }
        
        let p2 = match split.next() {
            Some(text) => match parse_coord(text) {
                Some(coord) => coord,
                None => continue,
            },
            None => continue,
        };
        
        execute(&mut state, p1, p2, op);
    }
    
    println!("Total brightness: {}", state.0);
}

