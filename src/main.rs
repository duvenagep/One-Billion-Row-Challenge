/// List of Implementations and optimisations
///
/// 1. Naive ->
mod naive;

use naive::{naive, read_lines};
use std::time::Instant;

fn main() {
    println!("Welcome to the 1BRC");
    let now = Instant::now();
    let p = "data/test.txt";
    let hm = naive(p);
    println!("{:?}", hm);
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
