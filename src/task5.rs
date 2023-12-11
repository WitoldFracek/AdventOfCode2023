use crate::utils::read_lines;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct RangeMap {
    size: usize,
    src: (usize, usize),
    dst: (usize, usize)
}

impl RangeMap {
    pub fn new(size: usize, src_start: usize, dst_start: usize) -> Self {
        Self {
            size,
            src: (src_start, src_start + size),
            dst: (dst_start, dst_start + size)
        }
    }

    pub fn map(&self, input: usize) -> usize {
        let (start, stop) = self.src;
        if input < start || input >= stop {
            return input;
        }
        let d = input - start;
        self.dst.0 + d
    }

    pub fn reverse_map(&self, input: usize) -> usize {
        let (start, stop) = self.dst;
        if input < start || input >= stop {
            return input;
        }
        let d = input - start;
        self.src.0 + d
    }
}

#[derive(Copy, Clone, Debug)]
struct Range {
    min: usize,
    max: usize
}

trait InRange {
    fn is_in(&self, range: &Range) -> bool;
}

impl Range {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: usize) -> bool {
        value >= self.min && value < self.max
    }

}

impl InRange for usize {
    fn is_in(&self, range: &Range) -> bool {
        range.contains(*self)
    }
}

#[derive(Debug)]
struct RangeMapModule {
    seq: Vec<RangeMap>
}

impl  RangeMapModule {
    pub fn new(seq: Vec<RangeMap>) -> Self {
        Self { seq }
    }

    pub fn map(&self, item: usize) -> usize {
        let mut orig = item;
        for range in self.seq.iter() {
            let temp = range.map(orig);
            if temp != orig {
                return temp;
            }
            orig = temp;
        }
        orig
    }

    pub fn reverse_map(&self, item: usize) -> usize {
        self.seq.iter()
            .rev()
            .fold(item, |prev, c| c.reverse_map(prev))
    }

    pub fn push(&mut self, range: RangeMap) {
        self.seq.push(range);
    }
}

pub fn solve() {
    let lines = read_lines("./data/data5_debug.txt");
    solve_a(&lines);
    solve_b(&lines);
}

fn solve_a(lines: &Vec<String>) {
    let seeds = get_seeds(&lines[0]);
    let maps = get_range_modules(&lines);
    let mut min = usize::MAX;
    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            val = map.map(val);
        }
        if val < min {
            min = val;
        }
    }
    println!("Ver 1: {min}");
}

fn solve_b(lines: &Vec<String>) {
    let seed_ranges = get_seeds_range(&lines[0]);
    let maps = get_range_modules(&lines);
    let mut res = usize::MAX;
    'main: for i in 0..usize::MAX {
        let mut val = i;
        for map in maps.iter().rev() {
            val = map.reverse_map(val);
        }
        for range in seed_ranges.iter() {
            if val.is_in(range) {
                res = i;
                break 'main
            }
        }
    }
    println!("Ver 2: {res}");
}

fn get_seeds(input: &str) -> Vec<usize> {
    input.split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap_or_else(|e| {println!("{x}"); panic!()}))
        .collect()
}

// fn get_min_max(input: &RangeMapModule) -> (usize, usize) {
//     let mut min = usize::MAX;
//     let mut max = usize::MIN;
//     for map in input.seq {
//         if map.dst.0 < min {
//             min = map.dst.1;
//         }
//         if map.
//     }
// }

fn get_seeds_range(input: &str) -> Vec<Range> {
    let temp = get_seeds(input);
    let mut ret = Vec::new();
    for (i, &start) in temp.iter().enumerate().step_by(2) {
        let len = temp[i+1];
        ret.push(Range::new(start, start + len));
    }
    ret
}

fn get_range_map(line: &str) -> RangeMap {
    let numbers = line.split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    RangeMap::new(numbers[2], numbers[1], numbers[0])
}

fn get_range_modules(lines: &Vec<String>) -> Vec<RangeMapModule> {
    let mut maps = Vec::new();
    let mut row = Vec::new();
    for line in lines[2..].iter() {
        let line = line.trim();
        if line.is_empty() {
            maps.push(RangeMapModule::new(Vec::from_iter(row.iter().map(|&x| x))));
            continue
        }
        if line.contains('-') {
            row = Vec::new();
        } else {
            row.push(get_range_map(line));
        }
    }
    maps.push(RangeMapModule::new(Vec::from_iter(row.iter().map(|&x| x))));
    maps
}