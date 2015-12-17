#![feature(io, slice_patterns)]

mod day_1;
mod day_2;

fn main() {
    let puzzles: Vec<Vec<fn()>> = vec![
        vec![day_1::puzzle_1, day_1::puzzle_2],
        vec![day_2::puzzle_1, day_2::puzzle_2],
    ];
    
    for (i, day) in puzzles.iter().enumerate() {
        for (j, puzzle) in day.iter().enumerate() {
            println!("Day {} Puzzle {}:", i + 1, j + 1);
            puzzle();
        }
    }
}
