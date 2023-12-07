use std::collections::HashSet;
use crate::utils::read_lines;

pub fn solve() {
    let lines = read_lines("./data/data4.txt");
    let res = lines.iter()
        .map(split_sequences)
        .map(|(win, guess)|(get_set(&win), get_set(&guess)))
        .map(get_intersection_len)
        .filter(|&x| x > 0)
        .map(|x| 2_i32.pow(x - 1))
        .sum::<i32>();
    println!("Ver 1: {res}");
}

fn split_sequences(line: &String) -> (String, String) {
    let split = line.split('|').collect::<Vec<_>>();
    let winning = split[0];
    let winning = winning.split(':').collect::<Vec<_>>()[1].trim();
    let guessed = split[1].trim();
    (winning.to_string(), guessed.to_string())
}

fn get_set(numbers: &String) -> HashSet<i32> {
    numbers.split(' ')
        .filter(|&x| x != "")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn get_intersection_len(sets: (HashSet<i32>, HashSet<i32>)) -> u32 {
    let (set1, set2) = sets;
    set1.intersection(&set2).collect::<Vec<_>>().len() as u32
}