/// Naive implementations akin to a pythonic script
use std::{collections::HashMap, fs::read_to_string};

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

pub fn naive(path: &str) -> HashMap<String, (u32, f32, f32, f32)> {
    let mut hm: HashMap<String, (u32, f32, f32, f32)> = HashMap::new();
    let lines = read_lines(path);
    for line in lines {
        let (station, temp) = line
            .split_once(";")
            .map(|(s, t)| (s.to_string(), t.parse::<f32>().unwrap()))
            .unwrap();

        hm.entry(station)
            .and_modify(|(c, s, m, mx)| {
                *c += 1;
                *s += &temp;
                *m = m.min(temp);
                *mx = mx.max(temp);
            })
            .or_insert((1, temp, temp, temp));
    }
    hm
}
