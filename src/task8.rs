use std::cmp::Ordering;
use crate::utils::read_lines;
use std::collections::HashMap;
use regex::Regex;

pub fn solve() {
    solve_a();
    solve_b();
}

fn solve_a() {
    let lines = read_lines("./data/data8.txt");
    let sequence = lines[0].trim();
    let map: HashMap<String, (String, String)> = HashMap::from_iter(
        extract_key_value(&lines[2..]).into_iter()
            .map(|(s, l, r)| (s, (l, r)))
    );
    let start = String::from("AAA");
    let end = String::from("ZZZ");
    let mut current = &start;
    let mut sol = 0;
    for (i, direction) in sequence.chars().cycle().enumerate() {
        if let Ordering::Equal = current.cmp(&end) {
            sol = i;
            break;
        }
        current = match direction {
            'L' => &map.get(current).unwrap().0,
            'R' => &map.get(current).unwrap().1,
            other => panic!("unrecognised direction '{other}'")
        };
    };
    println!("{sol}");
}

fn solve_b() {
    let lines = read_lines("./data/data8.txt");
    let sequence = lines[0].trim();
    let map: HashMap<String, (String, String)> = HashMap::from_iter(
        extract_key_value(&lines[2..]).into_iter()
            .map(|(s, l, r)| (s, (l, r)))
    );
    let starts = map.iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(key, _)| key)
        .collect::<Vec<&String>>();
    let mut cycles = Vec::new();
    for start in starts {

    }

    // let mut res = 0;
    // for (i, direction) in sequence.chars().cycle().enumerate() {
    //     if currents.iter().all(|s| s.ends_with('Z')) {
    //         res = i;
    //         break;
    //     }
    //     currents = currents.into_iter().map(|curr| {
    //             match direction {
    //                 'L' => &map.get(curr).unwrap().0,
    //                 'R' => &map.get(curr).unwrap().1,
    //                 other => panic!("unrecognised direction '{other}'")
    //             }
    //         })
    //         .collect()
    //
    // }
    println!("Ver 2: {res}");
}

fn extract_key_value(lines: &[String]) -> Vec<(String, String, String)> {
    let re = Regex::new(r"([A-Z]{3})").unwrap();
    lines.iter()
        .map(|line| {
            let mut res: Vec<String> = re.captures_iter(line)
                .map(|c| c.extract::<1>().0.to_string())
                .collect();
            (res.remove(0), res.remove(0), res.remove(0))
        })
        .collect()
}

// fn get_cycle_length(start: &String, map: &HashMap<String, (String, String)>) -> usize {
//
// }