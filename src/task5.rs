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

    pub fn push(&mut self, range: RangeMap) {
        self.seq.push(range);
    }
}

pub fn solve() {
    let lines = read_lines("./data/data5.txt");

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

    let seeds = get_seeds_range(&lines[0]);
    let maps = get_range_modules(&lines);
    let mut min = usize::MAX;
    for (i, &seed) in seeds.iter().enumerate() {
        if i % 1000 == 0 {
            println!("{}%", i as f64 / seeds.len() as f64 * 100.0);
        }
        let mut val = seed;
        for map in &maps {
            val = map.map(val);
        }
        if val < min {
            min = val;
        }
    }
    println!("Ver 2: {min}");
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

fn get_seeds_range(input: &str) -> Vec<usize> {
    let temp = input.split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap_or_else(|e| {println!("{x}"); panic!()}))
        .collect::<Vec<usize>>();
    let mut ret = Vec::new();
    for (i, &start) in temp.iter().enumerate().step_by(2) {
        let len = temp[i+1];
        println!("{} | {}", i as f64 / temp.len() as f64 * 100.0, len);
        ret.append(&mut Vec::from_iter(start..(start+len)));
    }
    ret
    // println!("Preparing HashSet");
    // let ret = HashSet::from_iter(ret);
    // println!("Returning HashSet");
    // ret
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