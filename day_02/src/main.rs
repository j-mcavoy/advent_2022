mod part_1;
use part_1::part_1;
mod part_2;
use part_2::part_2;

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn play(self, opponent: Self) -> Outcome {
        use Move::*;
        use Outcome::*;
        match (self, opponent) {
            (Rock, Scissors) => Win,
            (Scissors, Paper) => Win,
            (Paper, Rock) => Win,
            (x, y) if x == y => Draw,
            _ => Lose,
        }
    }
}
// Games(Opponent's Move, My Move)
#[derive(PartialEq, Clone, Copy, Debug)]
struct Game(Move, Move);
impl Game {
    fn play(&self) -> Score {
        let outcome = self.1.play(self.0);
        self.1 as Score + outcome as Score
    }
}

impl From<(Move, Move)> for Game {
    fn from(value: (Move, Move)) -> Self {
        Self(value.0, value.1)
    }
}
impl From<(Move, Outcome)> for Game {
    fn from(value: (Move, Outcome)) -> Self {
        let opponent_move = value.0;
        let my_move = opponent_move.get_move_for_outcome(value.1);
        Game(opponent_move, my_move)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Games(Vec<Game>);
impl Games {
    fn score(&self) -> Score {
        self.0.iter().map(|g| g.play()).sum()
    }
}
impl From<Vec<Game>> for Games {
    fn from(value: Vec<Game>) -> Self {
        Self(value)
    }
}

type Score = u32;

#[cfg(test)]
mod tests {

    pub const INPUT: &str = "A Y
B X
C Z";
}
