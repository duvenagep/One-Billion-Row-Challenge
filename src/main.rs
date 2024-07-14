/// List of Implementations and optimisations
///
/// 1. Naive ->
mod naive;

use naive::read_lines;
use std::time::Instant;

fn main() {
    println!("Welcome to the 1BRC");
    let now = Instant::now();
    let p = "data/test.txt";
    let l = read_lines(p);
    for i in l {
        println!("{:?}", i);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
