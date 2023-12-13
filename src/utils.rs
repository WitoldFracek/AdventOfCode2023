use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use num::Num;

pub fn read_lines(path: &str) -> Vec<String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };
    let reader = BufReader::new(file);
    let mut ret = vec![];
    for line in reader.lines() {
        match line {
            Ok(l) => ret.push(l),
            _ => {},
        }
    }
    ret
}

pub fn lines_iter(path: &str) -> impl Iterator<Item=String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };
    let reader = BufReader::new(file);
    reader.lines()
        .map(|line| {
            match line {
                Ok(l) => l,
                _ => String::new(),
            }
        })
        .filter(|s| !s.is_empty())
}

pub fn line_to_items<T>(line: &str) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    line.split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

pub fn line_to_numbers<N: Num + FromStr>(line: &str) -> Vec<N> where <N as FromStr>::Err: Debug {
    line.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<N>().unwrap())
        .collect()
}




