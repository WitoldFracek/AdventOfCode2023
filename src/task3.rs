use crate::utils::read_lines;

type Schema = Vec<Vec<char>>;

pub fn solve() {
    let lines = read_lines("./data/data3.txt");
    let schema = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Schema>();

    let res = schema.clone().iter()
        .map(find_numbers)
        .enumerate()
        .flat_map(|(i, v)| v.iter().map(|&x| (i, x)).collect::<Vec<_>>())
        .filter(|&(i, range)| is_valid(range, i, &schema))
        .map(|(i, range)| number_from_range(range, i, &schema))
        .sum::<i32>();
    println!("Ver 1: {res}");

    let res = schema.iter()
        .map(find_stars)
        .enumerate()
        .flat_map(|(line_index, v)| v.iter()
            .map(|&char_index| (line_index, char_index))
            .collect::<Vec<_>>())
        .map(|(line_no, char_no)| get_star_numbers(line_no, char_no, &schema))
        .filter(|x| x.len() == 2)
        .map(|v| get_gear_ratio(&v, &schema))
        .sum::<i32>();
    println!("Ver 2: {res:?}");

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
            } else if i == line.len() - 1 {
                end = i;
                ret.push((start, end));
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

fn find_stars(line: &Vec<char>) -> Vec<usize> {
    line.iter()
        .enumerate()
        .filter(|&(i, c)| *c == '*')
        .map(|(i, c)| i)
        .collect()
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
    ret
}

fn is_valid(number_bounds: (usize, usize), line_no: usize, schema: &Schema) -> bool {
    let bb = get_bounding_box(number_bounds, line_no, schema.len(), schema[0].len());
    bb.iter()
        .map(|&(x, y)|schema[x][y] != '.' && !schema[x][y].is_numeric())
        .any(|x| x)
}

fn number_from_range(range: (usize, usize), line_no: usize, schema: &Schema) -> i32 {
    let (start, stop) = range;
    String::from_iter(schema[line_no][start..=stop].iter()).parse::<i32>().unwrap()
}

fn get_star_bounding_box(line_no: usize, char_no: usize, schema: &Schema) -> Vec<(usize, usize)> {
    let lines_count = schema.len() as i32;
    let chars_count = schema[0].len() as i32;
    let cs = i32::max(char_no as i32 - 1, 0);
    let ls = i32::max(line_no as i32 - 1, 0);
    let ce = i32::min(char_no as i32 + 1, chars_count - 1);
    let le = i32::min(line_no as i32 + 1, lines_count - 1);
    (ls..=le).flat_map(|l| (cs..=ce).map(move |c| (l as usize, c as usize)))
        .filter(|&x| x != (line_no, char_no))
        .collect()
}

fn get_star_numbers(line_no: usize, char_no: usize, schema: &Schema) -> Vec<(usize, usize)> {
    let bb = get_star_bounding_box(line_no, char_no, schema);
    if bb.len() < 1 {
        return Vec::new();
    }
    let mut count = 0;
    let mut prev = bb[0];
    let mut number_found = false;

    let mut ret = Vec::new();
    if schema[prev.0][prev.1].is_numeric() {
        count += 1;
        number_found = true;
        ret.push((prev.0, prev.1));
    }
    let mut curr = (0, 0);
    for &elem in bb[1..].iter() {
        let c = schema[elem.0][elem.1];
        curr = elem;
        if schema[elem.0][elem.1].is_numeric() {
            if !number_found {
                number_found = true;
                ret.push((elem.0, elem.1));
            } else if !is_adjacent(curr, prev) {
                ret.push((elem.0, elem.1));
            }
        } else {
            number_found = false;
        }
        prev = curr;
    }
    ret
}

fn is_adjacent(this: (usize, usize), other: (usize, usize)) -> bool {
    usize::abs_diff(this.1, other.1) == 1
}

fn get_gear_ratio(indices: &Vec<(usize, usize)>, schema: &Schema) -> i32 {
    let g1 = indices[0];
    let g2 = indices[1];
    get_one_gear(g1, schema) * get_one_gear(g2, schema)
}

fn get_one_gear(index: (usize, usize), schema: &Schema) -> i32 {
    let mut c = schema[index.0][index.1];
    let mut number = String::from(c);
    if index.1 > 0 {
        let mut i = index.1 - 1;
        c = schema[index.0][i];
        while  i >= 0 && c .is_numeric() {
            number = format!("{c}{number}");
            if i == 0 { break }
            i -= 1;
            c = schema[index.0][i];
        }
    }
    if index.1 < schema[0].len() - 1 {
        let mut i = index.1 + 1;
        c = schema[index.0][i];
        while i < schema[0].len() && c.is_numeric() {
            number.push(c);
            i += 1;
            if i < schema[0].len() {
                c = schema[index.0][i];
            }
        }
    }
    number.parse::<i32>().unwrap()
}