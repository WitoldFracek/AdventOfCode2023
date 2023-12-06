use std::str::FromStr;
use crate::utils::read_lines;

pub struct Red(usize);
pub struct Green(usize);
pub struct Blue(usize);
pub type Colors = (Red, Green, Blue);


pub struct Game {
    pub id: usize,
    pub draws: Vec<Colors>
}

impl Game {
    fn get_game_id(arg: &str) -> usize {
        arg.split(' ').last().unwrap().parse().unwrap()
    }

    fn get_game_round(arg: &str) -> Colors {
        //4 green, 3 blue, 11 red
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for color_data in arg.split(',') {
            let data: Vec<&str> = color_data.trim().split(' ').collect();
            match (data[0], data[1]) {
                (n, "red") => {r += n.parse::<usize>().unwrap()}
                (n, "green") => {g += n.parse::<usize>().unwrap()}
                (n, "blue") => {b += n.parse::<usize>().unwrap()}
                _ => panic!()
            }
        }
        (Red(r), Green(g), Blue(b))
    }

    fn power(&self) -> usize {
        let min_r = self.draws.iter().map(|&(Red(r), _, _)| r).max().unwrap_or(0);
        let min_g = self.draws.iter().map(|&(_, Green(g), _)| g).max().unwrap_or(0);
        let min_b = self.draws.iter().map(|&(_, _, Blue(b))| b).max().unwrap_or(0);
        min_r * min_b * min_g
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let temp: Vec<&str> = s.split(':').collect();

        let game = temp[0];
        let id = Self::get_game_id(game);

        let tail = temp[1];
        let games = tail.split(';');
        let draws: Vec<Colors> = games.map(Self::get_game_round).collect();
        Ok(Self { id, draws })
    }
}
pub fn solve() {
    let lines = read_lines("./data/data2.txt");
    let res = lines.iter()
        .map(|s| Game::from_str(s).unwrap())
        .filter(|g| is_game_ok(g, 12, 13, 14))
        .map(|g| g.id)
        .sum::<usize>();
    println!("{res}");

    let res = lines.iter()
        .map(|s| Game::from_str(s).unwrap())
        .map(|g| g.power())
        .sum::<usize>();
    println!("{res}");
}

fn is_game_ok(game: &Game, reds: usize, greens: usize, blues: usize) -> bool {
    game.draws.iter()
        .all(|&(Red(r), Green(g), Blue(b))| r <= reds && g <= greens && b <= blues)
}