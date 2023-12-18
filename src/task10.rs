use crate::utils::read_lines;

type PipePlan = Vec<Vec<Option<Pipe>>>;
pub fn solve() {
    let lines = read_lines("./data/data10.txt");
    let pipes = get_pipes_plan(&lines);
    let sol = find_path_length(&pipes);
    println!("Ver 1: {sol}");
}

fn find_path_length(plan: &PipePlan) -> usize {
    let (x, y) = find_start(plan);
    if is_left_ok(plan, x, y) {
        if let Some(val) = try_get_cycle_length(plan, x-1, y, Direction::Right) {
            return val / 2;
        }
    }
    if is_right_ok(plan, x, y) {
        if let Some(val) = try_get_cycle_length(plan, x+1, y, Direction::Left) {
            return val / 2;
        }
    }
    if is_up_ok(plan, x, y) {
        if let Some(val) = try_get_cycle_length(plan, x, y-1, Direction::Down) {
            return val / 2;
        }
    }
    if is_down_ok(plan, x, y) {
        if let Some(val) = try_get_cycle_length(plan, x, y+1, Direction::Up) {
            return val / 2;
        }
    }
    panic!("no solution")
}

fn is_left_ok(plan: &PipePlan, sx: usize, sy: usize) -> bool {
    if sx == 0 { return false; }
    match plan[sy][sx-1] {
        None => false,
        Some(pipe) => pipe.has_right()
    }
}

fn is_right_ok(plan: &PipePlan, sx: usize, sy: usize) -> bool {
    if sx + 1 == plan[0].len() { return false; }
    match plan[sy][sx+1] {
        None => false,
        Some(pipe) => pipe.has_left(),
    }
}

fn is_up_ok(plan: &PipePlan, sx: usize, sy: usize) -> bool {
    if sy == 0 { return false; }
    match plan[sy-1][sx] {
        None => false,
        Some(pipe) => pipe.has_down(),
    }
}

fn is_down_ok(plan: &PipePlan, sx: usize, sy: usize) -> bool {
    if sy + 1 == plan.len() { return false; }
    match plan[sy+1][sx] {
        None => false,
        Some(pipe) => pipe.has_up(),
    }
}

fn try_get_cycle_length(plan: &PipePlan, x_start: usize, y_start: usize, came_from: Direction) -> Option<usize> {
    let mut current = plan[y_start][x_start];
    let mut x = x_start as i32;
    let mut y = y_start as i32;
    let mut came_from = came_from;
    if let None = current { return None; }
    let mut counter = 1;
    while current.is_some() {
        let temp = current.unwrap();
        if temp == Pipe::Start { return Some(counter); }
        let (dx, dy) = temp.leads_to(came_from);
        if dx == -1 { came_from = Direction::Right; }
        if dx == 1 { came_from = Direction::Left; }
        if dy == -1 { came_from = Direction::Down; }
        if dy == 1 { came_from = Direction::Up; }

        if x + dx < 0 || x + dx == plan[0].len() as i32 { return None; }
        if y + dy < 0 || y + dy == plan.len() as i32 { return None; }

        x = x + dx;
        y = y + dy;
        current = plan[y as usize][x as usize];
        counter += 1;
    }
    None
}

fn get_pipes_plan(lines: &Vec<String>) -> PipePlan {
    lines.iter()
        .map(|line| line.chars()
            .map(Pipe::from_char)
            .collect()
        )
        .collect()
}

fn find_start(plan: &PipePlan) -> (usize, usize) {
    for (y, row) in plan.iter().enumerate() {
        for (x, &pipe) in row.iter().enumerate() {
            if let Some(Pipe::Start) = pipe {
                return (x, y);
            }
        }
    }
    panic!("there is no start (S) pipe in the PipePlan");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pipe {
    LeftRight,  // -
    UpDown,    //  |
    LeftDown,  //  7
    LeftUp,    //  J
    RightDown, //  F
    RightUp,   //  L
    Start,     //  S
}

impl Pipe {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '-' => Some(Pipe::LeftRight),
            '|' => Some(Pipe::UpDown),
            '7' => Some(Pipe::LeftDown),
            'J' => Some(Pipe::LeftUp),
            'F' => Some(Pipe::RightDown),
            'L' => Some(Pipe::RightUp),
            'S' => Some(Pipe::Start),
            _ => None
        }
    }

    fn which_neighbours_check(&self) -> (bool, bool, bool, bool) {
        (self.has_left(), self.has_right(), self.has_up(), self.has_down())
    }

    fn has_left(&self) -> bool {
        match self {
            Self::LeftDown | Self::LeftUp | Self::LeftRight => true,
            _ => false
        }
    }

    fn has_right(&self) -> bool {
        match self {
            Self::LeftRight | Self::RightUp | Self::RightDown => true,
            _ => false
        }
    }

    fn has_up(&self) -> bool {
        match self {
            Self::UpDown | Self::LeftUp | Self::RightUp => true,
            _ => false
        }
    }

    fn has_down(&self) -> bool {
        match self {
            Self::UpDown | Self::LeftDown | Self::RightDown => true,
            _ => false
        }
    }

    fn get_pretty_repr(&self) -> char {
        match self {
            Self::LeftRight => '─',
            Self::UpDown => '│',
            Self::LeftUp => '┘',
            Self::LeftDown => '┐',
            Self::RightDown => '┌',
            Self::RightUp => '└',
            Self::Start => 'S',
        }
    }

    fn leads_to(&self, came_from: Direction) -> (i32, i32) {
        match came_from {
            Direction::Left => match self {
                Pipe::LeftRight => (1, 0),
                Pipe::LeftUp => (0, -1),
                Pipe::LeftDown => (0, 1),
                _ => panic!("impossible direction {self:?}")
            }
            Direction::Right => match self {
                Pipe::LeftRight => (-1, 0),
                Pipe::RightUp => (0, -1),
                Pipe::RightDown => (0, 1),
                _ => panic!("impossible direction {self:?}")
            }
            Direction::Up => match self {
                Pipe::UpDown => (0, 1),
                Pipe::LeftUp => (-1, 0),
                Pipe::RightUp => (1, 0),
                _ => panic!("impossible direction {self:?}")
            }
            Direction::Down => match self {
                Pipe::UpDown => (0, -1),
                Pipe::LeftDown => (-1, 0),
                Pipe::RightDown => (1, 0),
                _ => panic!("impossible direction {self:?}")
            }
        }
    }
}

fn print_pipes_plan(plan: &PipePlan) {
    let mut res = String::new();
    for row in plan {
        for pipe in row {
            let c = match pipe {
                Some(pipe) => pipe.get_pretty_repr(),
                None => ' ',
            };
            res.push(c);
        }
        res.push('\n')
    }
    println!("{res}");
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left, Right, Up, Down
}

