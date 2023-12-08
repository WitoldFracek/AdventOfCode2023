use crate::utils::read_lines;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct RangeMap {
    size: i32,
    src: (i32, i32),
    dst: (i32, i32)
}

impl RangeMap {
    pub fn new(size: i32, src_start: i32, dst_start: i32) -> Self {
        Self {
            size,
            src: (src_start, src_start + size),
            dst: (dst_start, dst_start + size)
        }
    }

    pub fn map(&self, input: i32) -> i32 {
        let (start, stop) = self.src;
        if input < start || input >= stop {
            return input;
        }
        let d = input - start;
        self.dst.0 + d
    }
}

pub fn solve() {
    let lines = read_lines("./data/data5_debug.txt");
    let seeds = get_seeds(&lines[0]);

    let mut maps = Vec::new();
    let mut row = Vec::new();
    for line in &lines[2..] {
        let line = line.trim();
        if line.is_empty() {
            maps.push(Vec::from_iter(row.into_iter()));
            continue
        }
        if line.contains('-') {
            row = Vec::new();
        } else {
            row.push(get_range_map(line));
        }
    }


}

fn get_seeds(input: &str) -> Vec<i32> {
    input.split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|&x| x != "")
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn get_range_map(line: &str) -> RangeMap {
    let numbers = line.split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    RangeMap::new(numbers[2], numbers[1], numbers[0])
}