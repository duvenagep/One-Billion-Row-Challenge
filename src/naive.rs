use std::{collections::HashMap, fs::read_to_string};

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn naive(path: &str) -> HashMap<String, f32> {
    let mut hm: HashMap<String, f32> = HashMap::new();
    let lines = read_lines(path);
    for line in lines {
        let (station, temp) = line.split_once(";").unwrap();
        hm.insert(station, temp)
    }

    1.0
}
