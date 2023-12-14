use crate::utils::read_lines;

static STR_NUMS: [&str; 20] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "zero"
];

fn str_to_num(numstr: &str) -> String {
    if numstr.len() == 1 {
        String::from(numstr)
    } else {
        match numstr {
            "zero" => String::from("0"),
            "one" => String::from("1"),
            "two" => String::from("2"),
            "three" => String::from("3"),
            "four" => String::from("4"),
            "five" => String::from("5"),
            "six" => String::from("6"),
            "seven" => String::from("7"),
            "eight" => String::from("8"),
            "nine" => String::from("9"),
            _ => panic!(),
        }
    }
}
pub fn solve() {
    let lines = read_lines("./data/data1.txt");

    let res = lines.iter().map(find_number).sum::<i32>();
    println!("Ver 1: {res}");

    let res = lines.iter().map(find_number_ex).sum::<i32>();
    println!("Ver 2: {res}");
}

fn find_number(line: &String) -> i32 {
    let first = find_first_number(line);
    let last = find_last_number(line);
    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn find_number_ex(line: &String) -> i32 {
    let findings = STR_NUMS.iter()
        .map(|s| line.match_indices(s).collect::<Vec<(usize, &str)>>())
        .filter(|v| !v.is_empty())
        .flat_map(|v| v)
        .collect::<Vec<(usize, &str)>>();
    let max = findings.iter().max_by_key(|(i, _)| i).unwrap().1;
    let min = findings.iter().min_by_key(|(i, _)| i).unwrap().1;
    format!("{}{}", str_to_num(min), str_to_num(max)).parse::<i32>().unwrap()
}

fn find_first_number(line: &str) -> char {
    line.chars()
        .filter(|c| c.is_numeric())
        .next()
        .unwrap()
}

fn find_last_number(line: &str) -> char {
    line.chars()
        .filter(|c| c.is_numeric())
        .rev()
        .next()
        .unwrap()
}