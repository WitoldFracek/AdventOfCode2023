use std::str::FromStr;
use crate::utils::read_lines;
use std::collections::{HashSet, HashMap};
trait ToCard {
    type Err;
    fn into(&self) -> Result<Card, Self::Err>;
}
trait ToHand {
    type Err;
    fn into(&self) -> Result<Hand, Self::Err>;
}

pub fn solve() {
    let lines = read_lines("./data/data7_debug.txt");
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
    Two = 0
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Figure {
    FiveOfAKind = 6,  // unique 1
    FourOfAKind = 5,  // unique 2
    FullHouse = 4,    // unique 2
    ThreeOfAKind = 3, // unique 3
    TwoPair = 2,      // unique 3
    OnePair = 1,      // unique 4
    HighCard = 0,     // unique 5
}

impl ToCard for char {
    type Err = String;

    fn into(&self) -> Result<Card, Self::Err> {
        match self {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            other => Err(format!("Unrecognised character '{other}'")),
        }
    }
}

struct Hand {
    cards: [Card; 5]
}

impl ToHand for &str {
    type Err = String;

    fn into(&self) -> Result<Hand, Self::Err> {
        if self.len() != 5 {
            return Err(format!("sequence length different than 5. Given {}", self.len()));
        }

        let cards: Vec<Card> = self.chars()
            .map(|c| <char as ToCard>::into(&c).unwrap())
            .collect();
        Ok(Hand::new([cards[0], cards[1], cards[2], cards[3], cards[4]]))
    }
}

impl Hand {
    fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }

    fn figure(&self) -> Figure {
        let mut set = HashMap::new();
        for elem in self.cards {
            let count = set.entry(elem).or_insert(0);
            *count += 1;
        }
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
}

struct Game {
    hand: Hand,
    value: i32,
    figure: Figure,
}

impl Game {
    fn new(hand: Hand, value: i32) -> Self {
        Self {hand, value, figure: hand.figure()}
    }
}