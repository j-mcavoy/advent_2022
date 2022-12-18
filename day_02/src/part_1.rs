use super::*;
pub fn part_1(s: &str) -> Score {
    let games = str2games(s);
    
    games.score()
}

fn str2games(s: &str) -> Games {
    s.lines()
        .map(|line| {
            let vec: Vec<&str> = line.split_whitespace().collect();
            assert!(vec.len() == 2);
            let (_, opponent) = vec[0].char_indices().next().unwrap();
            let (_, me) = vec[1].char_indices().next().unwrap();
            let game: Game = (
                <char as TryInto<Move>>::try_into(opponent).unwrap(),
                <char as TryInto<Move>>::try_into(me).unwrap(),
            )
                .into();
            game
        })
        .collect::<Vec<Game>>()
        .into()
}

impl TryFrom<char> for Move {
    type Error = String;
    fn try_from(value: char) -> Result<Move, String> {
        use Move::*;
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err("Invalid Move".into()),
        }
    }
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
                Game(Rock, Paper),
                Game(Paper, Rock),
                Game(Scissors, Scissors),
            ]),
            str2games(INPUT),
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(15, part_1(INPUT));
    }
}
