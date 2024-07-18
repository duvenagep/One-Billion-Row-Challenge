/// List of Implementations and optimisations
///
/// 1. Naive -> 223.3567s
mod naive;

use naive::{naive, read_lines};
use std::time::Instant;

fn main() {
    println!("Welcome to the 1BRC");
    let now = Instant::now();
    let p = "data/measurements.txt";
    let hm = naive(p);
    let mut v = hm.into_iter().collect::<Vec<_>>();
    // v.sort_unstable_by_key(|p| p.0);
    for (name, r) in v {
        println!("{name}: {:.1}/{:.1}/{:.1}", r.1, r.2, r.3);
    }
    // println!("{:?}", hm);
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
