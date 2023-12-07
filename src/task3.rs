use crate::utils::read_lines;

pub fn solve() {
    let lines = read_lines("./data/data3_debug.txt");
    let schema = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // println!("{} {}", schema.len(), schema[0].len());

    let res = schema.iter()
        .map(find_numbers)
        .enumerate()
        .flat_map(|(i, v)| v.iter().map(|&x| (i, x)).collect::<Vec<_>>())
        .filter(|&(i, range)| is_valid(range, i, &schema))
        .collect::<Vec<(usize, (usize, usize))>>();
    for elem in res {
        println!("{elem:?}");
    }

    // let res = schema.iter()
    //     .map(find_numbers)
    //     .enumerate()
    //     .flat_map(|(i, v)| v.iter().map(|&x| (i, x)).collect::<Vec<_>>())
    //     .filter(|&(i, range)| is_valid(range, i, &schema))
    //     .map(|(i, range)| number_from_range(range, i, &schema))
    //     .sum::<i32>();
    // println!("{res}");


    // let input = String::from("..123..^.23..").chars().collect::<Vec<char>>();
    // let res = find_numbers(&input);
    // let res = get_bounding_box(res[0], 1, 3, 13);
    // println!("{res}");
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
    let(start, stop) = (word.0 as i32, word.1 as i32);

    let mut sx = i32::max(start - 1, 0);
    let mut sy = i32::max(line_no - 1, 0);
    let mut ex = i32::min(stop + 1, schema_chars_count - 1);
    let mut ey = i32::min(line_no + 1, schema_lines_count - 1);

    let mut ret = Vec::new();
    for x in sx..=ex {
        for y in sy..=ey {
            if y == line_no {
                if x < start || x > stop {
                    ret.push((y as usize, x as usize))
                }
            } else {
                ret.push((y as usize, x as usize))
            }
        }
    }
    // println!("{ret:?}");
    ret
}

fn is_valid(number_bounds: (usize, usize), line_no: usize, schema: &Vec<Vec<char>>) -> bool {
    let bb = get_bounding_box(number_bounds, line_no, schema.len(), schema[0].len());
    bb.iter()
        .map(|&(x, y)|schema[x][y] != '.' && !schema[x][y].is_numeric())
        .any(|x| x)
}

fn number_from_range(range: (usize, usize), line_no: usize, schema: &Vec<Vec<char>>) -> i32 {
    let (start, stop) = range;
    String::from_iter(schema[line_no][start..=stop].iter()).parse::<i32>().unwrap()
}