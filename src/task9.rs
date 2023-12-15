use crate::utils::{lines_iter, line_to_numbers};

pub fn solve() {
    let sol = lines_iter("./data/data9.txt")
        .map(|x| line_to_numbers::<i64>(&x))
        .map(predict_value)
        .sum::<i64>();
    println!("Ver 1: {sol}");
}

fn predict_value(values: Vec<i64>) -> i64 {
    let mut cache = Vec::new();
    let mut seq = values;
    while seq.iter().any(|&x| x != 0) {
        let last = *seq.last().unwrap();
        cache.push(last);
        seq = (1..seq.len()).map(|i| seq[i] - seq[i-1]).collect();
    }
    seq.push(0);
    cache.iter().sum()
}