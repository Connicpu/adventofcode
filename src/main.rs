#![feature(io, slice_patterns)]

extern crate crypto;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    let puzzles: Vec<Vec<fn()>> = vec![
        vec![day_1::puzzle_1, day_1::puzzle_2],
        vec![day_2::puzzle_1, day_2::puzzle_2],
        vec![day_3::puzzle_1, day_3::puzzle_2],
        vec![day_4::puzzle_1, day_4::puzzle_2],
        vec![day_5::puzzle_1, day_5::puzzle_2],
    ];
    
    for (i, day) in puzzles.iter().enumerate().skip(4) {
        for (j, puzzle) in day.iter().enumerate() {
            println!("Day {} Puzzle {}:", i + 1, j + 1);
            puzzle();
        }
    }
}
