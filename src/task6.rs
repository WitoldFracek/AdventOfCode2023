use crate::utils::read_lines;

pub fn solve() {
    let lines = read_lines("./data/data6.txt");
    let durations = extract_numbers(&lines[0]);
    let records = extract_numbers(&lines[1]);
    let res = durations.iter()
        .zip(records.iter())
        .fold(1, |acc, (&d, &r)| acc * get_wins_count(d, r));
    println!("Ver 1: {res}");

    let duration = combine_numbers(&durations);
    let record = combine_numbers(&records);
    let res = get_wins_count(duration, record);
    println!("Ver 2: {res}");


}

fn extract_numbers(line: &str) -> Vec<i64> {
    line.split(':')
        .last()
        .unwrap_or("")
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn combine_numbers(nums: &Vec<i64>) -> i64 {
    nums.iter()
        .map(|n| n.to_string())
        .fold("".to_string(), |acc, s| format!("{acc}{s}"))
        .parse::<i64>()
        .unwrap()
}

fn get_wins_count(duration: i64, record: i64) -> i64 {
    (0..=duration)
        .map(|x| (duration - x) * x)
        .filter(|&x| x > record)
        .collect::<Vec<_>>()
        .len() as i64
}
