use std::cmp::Ordering;
use std::str::FromStr;
use crate::utils::{lines_iter};
use std::collections::{HashMap};
trait ToCard {
    type Err;
    fn into(&self) -> Result<Card, Self::Err>;
    fn joker_into(&self) -> Result<Card, Self::Err>;
}
trait ToHand {
    type Err;
    fn into(&self) -> Result<Hand, Self::Err>;
}

pub fn solve() {
    // solution A
    let mut games = lines_iter("./data/data7.txt")
        .map(get_game)
        .collect::<Vec<Game>>();
    games.sort();
    let sol = games.iter()
        .enumerate()
        .fold(0, |prev, (i, game)| prev + (i as i32 + 1) * game.value);
    println!("Ver 1: {sol}");

    // Solution B
    let mut games = lines_iter("./data/data7.txt")
        .map(get_game_with_jokers)
        .collect::<Vec<Game>>();
    games.sort();
    let sol = games.iter()
        .enumerate()
        .fold(0, |prev, (i, game)| prev + (i as i32 + 1) * game.value);
    println!("Ver 2: {sol}");
}

fn get_game(line: String) -> Game {
    let data: Vec<&str> = line.trim()
        .split(' ')
        .collect();
    let hand: Hand = data[0].into();
    let value = data[1].parse::<i32>().unwrap();
    Game::new(hand, value)
}

fn get_game_with_jokers(line: String) -> Game {
    let data: Vec<&str> = line.trim()
        .split(' ')
        .collect();
    let hand = Hand::from_str_with_joker(data[0]);
    let value = data[1].parse::<i32>().unwrap();
    Game::new(hand, value)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd)]
enum Card {
    A = 12,
    K = 11,
    Q = 10,
    J = 9,
    T = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
    Joker = -1,
}

impl Card {
    fn from_char_with_joker(arg: char) -> Self {
        match arg.into() {
            Card::J => Card::Joker,
            e => e
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd)]
enum Figure {
    FiveOfAKind = 6,  // unique 1
    FourOfAKind = 5,  // unique 2
    FullHouse = 4,    // unique 2
    ThreeOfAKind = 3, // unique 3
    TwoPair = 2,      // unique 3
    OnePair = 1,      // unique 4
    HighCard = 0,     // unique 5
}

impl Figure {
    fn upgrade(&self) -> Self {
        match self {
            Figure::FiveOfAKind => Figure::FiveOfAKind,
            Figure::FourOfAKind => Figure::FiveOfAKind,
            Figure::FullHouse => Figure::FourOfAKind,
            Figure::ThreeOfAKind => Figure::FourOfAKind,
            Figure::TwoPair => Figure::FullHouse,
            Figure::OnePair => Figure::ThreeOfAKind,
            Figure::HighCard => Figure::OnePair,
        }
    }

    fn upgrade_n(&self, n: i32) -> Self {
        let mut ret = *self;
        for _ in 0..n {
            ret = ret.upgrade();
        }
        ret
    }
}

impl Into<Card> for char {
    fn into(self) -> Card {
        match self {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            other => panic!("Unrecognised character '{other}'"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5]
}

impl Into<Hand> for &str {
    fn into(self) -> Hand {
        if self.len() != 5 {
            return panic!("sequence length different than 5. Given {}", self.len());
        }

        let cards: Vec<Card> = self.chars()
            .map(|c| c.into())
            .collect();
        Hand::new([cards[0], cards[1], cards[2], cards[3], cards[4]])
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }

    fn from_str_with_joker(arg: &str) -> Self {
        if arg.len() != 5 {
            return panic!("sequence length different than 5. Given {}", arg.len());
        }
        let cards: Vec<Card> = arg.chars()
            .map(Card::from_char_with_joker)
            .collect();
        Hand::new([cards[0], cards[1], cards[2], cards[3], cards[4]])
    }

    fn get_figure(&self) -> Figure {
        let mut set = HashMap::new();
        for elem in self.cards {
            let count = set.entry(elem).or_insert(0);
            *count += 1;
        }
        let joker_count = *set.get(&Card::Joker).unwrap_or(&0);
        return if joker_count == 0 {
            Self::get_figure_default(set)
        } else {
            let set: HashMap<Card, i32> = set.into_iter()
                .filter(|(card, _)| *card != Card::Joker)
                .collect();
            Self::get_figure_with_jokers(set, joker_count)
        };
    }

    fn get_figure_default(set: HashMap<Card, i32>) -> Figure {
        let unique_count = set.len();
        if unique_count == 1 {
            return Figure::FiveOfAKind;
        }
        if unique_count == 5 {
            return Figure::HighCard;
        }
        if unique_count == 4 {
            return Figure::OnePair;
        }
        if unique_count == 2 {
            return if set.iter().any(|(_, &count)| count == 4) {
                Figure::FourOfAKind
            } else {
                Figure::FullHouse
            }
        }
        return if set.iter().any(|(_, &count)| count == 3) {
            Figure::ThreeOfAKind
        } else {
            Figure::TwoPair
        }
    }

    fn get_figure_with_jokers(set: HashMap<Card, i32>, joker_count: i32) -> Figure {
        // We have at least one jocker
        let unique_count = set.len();
        let max = set.iter().map(|(_, &count)| count).max().unwrap_or(0);
        if unique_count == 2 {
            return match (max, joker_count) {
                (3, 1) => Figure::ThreeOfAKind.upgrade(),
                (2, 1) => Figure::TwoPair.upgrade(),
                (2, 2) => Figure::OnePair.upgrade_n(2),
                (1, 3) => Figure::HighCard.upgrade_n(3),
                _ => panic!("unrecognised state 'max = {max}, jocker count = {joker_count}'")
            };
        }
        if unique_count == 3 {
            return if joker_count == 1 {
                Figure::OnePair.upgrade()
            } else {
              Figure::HighCard.upgrade_n(2)
            };
        }
        if unique_count == 4 {
            return Figure::HighCard.upgrade();
        }
        Figure::FiveOfAKind
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    value: i32,
    figure: Figure,
}

impl Game {
    fn new(hand: Hand, value: i32) -> Self {
        let figure = hand.get_figure();
        Self {hand, value, figure}
    }
}

impl Eq for Game { }

impl PartialEq<Self> for Game {
    fn eq(&self, other: &Self) -> bool {
        if self.figure != other.figure {
            return false;
        }
        for (e1, e2) in self.hand.cards.iter().zip(other.hand.cards.iter()) {
            if e1 != e2 {
                return false;
            }
        }
        true
    }
}

impl PartialOrd<Self> for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.figure > other.figure {
            return Some(Ordering::Greater)
        }
        if self.figure < other.figure {
            return Some(Ordering::Less)
        }
        for (e1, e2) in self.hand.cards.iter().zip(other.hand.cards.iter()) {
            if e1 > e2 {
                return Some(Ordering::Greater);
            } else if e1 < e2 {
                return Some(Ordering::Less);
            }
        }
        None
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => panic!("cannot compare")
        }
    }
}