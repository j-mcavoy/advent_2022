use super::*;
pub fn part_2(s: &str) -> Score {
    let games = str2games(s);
    
    games.score()
}

impl Move {
    pub fn get_move_for_outcome(&self, outcome: Outcome) -> Move {
        use Move::*;
        use Outcome::*;
        match (outcome, self) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, _) => *self,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = String;
    fn try_from(value: char) -> Result<Self, String> {
        use Outcome::*;
        match value {
            'X' => Ok(Lose),
            'Y' => Ok(Draw),
            'Z' => Ok(Win),
            _ => Err("Not a valid Outcome".into()),
        }
    }
}

fn str2games(s: &str) -> Games {
    s.lines()
        .map(|line| {
            let vec: Vec<&str> = line.split_whitespace().collect();
            assert!(vec.len() == 2);
            let (_, opponent): (_, char) = vec[0].char_indices().next().unwrap();
            let (_, me): (_, char) = vec[1].char_indices().next().unwrap();
            let game: Game = (
                <char as TryInto<Move>>::try_into(opponent).unwrap(),
                <char as TryInto<Outcome>>::try_into(me).unwrap(),
            )
                .into();
            game
        })
        .collect::<Vec<Game>>()
        .into()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::INPUT;
    use Move::*;

    #[test]
    fn tets_str2games() {
        assert_eq!(
            Games(vec![
                Game(Rock, Rock),
                Game(Paper, Rock),
                Game(Scissors, Rock),
            ]),
            str2games(INPUT),
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(12, part_2(INPUT));
    }
}
