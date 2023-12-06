use crate::utils::read_lines;

pub fn solve() {
    let lines = read_lines("./data/data3.txt");
    let schema = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let number_indices = schema.iter()
        .map(find_numbers)
        .collect::<Vec<Vec<(usize, usize)>>>();

    let input = String::from("123..^.23..").chars().collect::<Vec<char>>();
    let res = find_numbers(&input);
    println!("{res:?}");
}

fn find_numbers(line: &Vec<char>) -> Vec<(usize, usize)> {
    let mut number_found = false;
    let mut ret = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for (i, &c) in line.iter().enumerate() {
        if c.is_numeric() {
            if !number_found {
                start = i;
            }
            number_found = true;
        } else {
            if number_found {
                end = i - 1;
                ret.push((start, end));
            }
            number_found = false;
        }
    }
    ret
}

fn get_bounding_box(
    word: (usize, usize),
    line_no: usize,
    schema_lines_count: usize,
    schema_chars_count: usize
) -> Vec<(usize, usize)> {
    let line_no = line_no as i32;
    let schema_lines_count = schema_lines_count as i32;
    let schema_chars_count = schema_chars_count as i32;

    let mut ret = Vec::new();
    let(start, stop) = (word.0 as i32, word.1 as i32);

    let mut sx = start - 1;
    let mut sy = line_no - 1;
    let mut ex = stop + 1;
    let mut ey = line_no + 1;

    if start == 0 {
        sx = 0;
    }
    if stop + 1 == schema_chars_count {
        ex = stop
    }


    ret
}