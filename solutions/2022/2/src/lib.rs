#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissor,
}
#[derive(Debug, Clone, Copy)]
enum Column2 {
    X,
    Y,
    Z,
}
#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl RPSMove {
    fn against(self, p2move: Self) -> GameResult {
        use GameResult::*;
        use RPSMove::*;
        match (self, p2move) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissor) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissor) => Lose,
            (Scissor, Rock) => Lose,
            (Scissor, Paper) => Win,
            (Scissor, Scissor) => Draw,
        }
    }

    fn round_score(self, p2move: Self) -> i64 {
        let shape_score = match self {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissor => 3,
        };
        let result_score = match self.against(p2move) {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        };
        shape_score + result_score
    }
}

fn read_input(input: &str) -> Vec<(RPSMove, Column2)> {
    input
        .split("\n")
        .filter_map(|line| {
            let line = line.trim();
            if let Some((p1, p2)) = line.split_once(" ") {
                use Column2::*;
                use RPSMove::*;

                let p1 = match p1 {
                    "A" => Rock,
                    "B" => Paper,
                    "C" => Scissor,
                    _ => panic!("Unrecognized move {p1}"),
                };
                let p2 = match p2 {
                    "X" => X,
                    "Y" => Y,
                    "Z" => Z,
                    _ => panic!("Unrecognized move {p2}"),
                };
                Some((p1, p2))
            } else if line.trim() == "" {
                None // skip empty lines
            } else {
                panic!("No space in line")
            }
        })
        .collect()
}

pub fn part1(input: &str) -> i64 {
    read_input(input)
        .into_iter()
        .map(|(p2, p1)| {
            let p1 = match p1 {
                Column2::X => RPSMove::Rock,
                Column2::Y => RPSMove::Paper,
                Column2::Z => RPSMove::Scissor,
            };
            p1.round_score(p2)
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    read_input(input)
        .into_iter()
        .map(|(p2, p1)| {
            use GameResult::*;
            use RPSMove::*;
            // converting into the needed result
            let p1 = match p1 {
                Column2::X => Lose,
                Column2::Y => Draw,
                Column2::Z => Win,
            };
            // finding the right move
            let p1 = match (p1, p2) {
                (Win, Rock) => Paper,
                (Win, Paper) => Scissor,
                (Win, Scissor) => Rock,
                (Lose, Rock) => Scissor,
                (Lose, Paper) => Rock,
                (Lose, Scissor) => Paper,
                (Draw, p2) => p2,
            };
            p1.round_score(p2)
        })
        .sum()
}
