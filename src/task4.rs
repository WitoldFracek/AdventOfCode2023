use std::collections::{HashSet, HashMap};
use std::str::FromStr;
use crate::utils::read_lines;

struct Scratchcard {
    id: usize,
    wins: HashSet<i32>,
    guesses: HashSet<i32>,
}

impl Scratchcard {
    fn new(id: usize, wins: HashSet<i32>, guesses: HashSet<i32>) -> Self {
        Self {id, wins, guesses}
    }

    fn how_many_matches(&self) -> usize {
        self.wins.intersection(&self.guesses)
            .collect::<Vec<_>>()
            .len()
    }

    fn get_scratchcards_ids_to_copy(&self, copies: &HashMap<usize, usize>) -> Vec<usize> {
        let matches_count = self.how_many_matches();
        let mut ret = Vec::new();
        for _ in 0..copies[&self.id] {
            ret.append(&mut Vec::from_iter(((self.id+1)..=(self.id+matches_count))))
        }
        ret
    }
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('|').collect::<Vec<_>>();
        let guessed = split[1].trim();
        let split = split[0].split(':').collect::<Vec<_>>();
        let game_id = split[0].split(' ').last().unwrap().parse::<usize>().unwrap();
        let winning = split[1].trim();

        let winning = get_set(&winning);
        let guessed = get_set(&guessed);
        Ok(Self::new(game_id, winning, guessed))
    }
}

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

    let cards = lines.iter()
        .map(|line| Scratchcard::from_str(line).unwrap())
        .collect::<Vec<Scratchcard>>();
    let mut copies = HashMap::new();
    for card in cards.iter() {
        copies.insert(card.id, 1);
    }
    for card in cards.iter() {
        let to_copy = card.get_scratchcards_ids_to_copy(&copies);
        for index in to_copy {
            copies.insert(index, copies[&index] + 1);
        }
    }
    let res = copies.iter().map(|(_, v)| *v).sum::<usize>();
    println!("Ver 2: {res}");
}

fn split_sequences(line: &String) -> (String, String) {
    let split = line.split('|').collect::<Vec<_>>();
    let winning = split[0];
    let winning = winning.split(':').collect::<Vec<_>>()[1].trim();
    let guessed = split[1].trim();
    (winning.to_string(), guessed.to_string())
}

fn get_set(numbers: &str) -> HashSet<i32> {
    numbers.split(' ')
        .filter(|&x| x != "")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn get_intersection_len(sets: (HashSet<i32>, HashSet<i32>)) -> u32 {
    let (set1, set2) = sets;
    set1.intersection(&set2).collect::<Vec<_>>().len() as u32
}