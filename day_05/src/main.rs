fn main() {
    let input = include_str!("../input");
    println!("Part 1 Answer: {}", part_1(input));
    println!("Part 2 Answer: {}", part_2(input));
}

type Answer = String;
fn part_1(s: &str) -> Answer {
    let (stacks, moves) = s.split_once("\n\n").unwrap();
    let mut stacks: Stacks = str2stacks(stacks);
    let moves: Moves = str2moves(moves);

    for _move in moves {
        stacks.move_part_1(_move);
    }

    stacks.top_crates()
}
fn part_2(s: &str) -> Answer {
    let (stacks, moves) = s.split_once("\n\n").unwrap();
    let mut stacks: Stacks = str2stacks(stacks);
    let moves: Moves = str2moves(moves);

    for _move in moves {
        stacks.move_part_2(_move);
    }

    stacks.top_crates()
}

fn str2stacks(s: &str) -> Stacks {
    let mut iter = s.lines().rev();
    let stack_width = iter
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap() as usize;

    let mut stacks: Stacks = Stacks(vec![Stack(vec![]); stack_width]);
    iter.for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        for c_id in 0..stack_width {
            let char = chars[1 + 4 * c_id];
            if char != ' ' {
                stacks.0[c_id].push(Crate(char));
            }
        }
    });
    stacks
}
fn str2moves(s: &str) -> Moves {
    s.lines().map(Move::from).collect()
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);
#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);
impl Stack {
    fn pop(&mut self) -> Crate {
        self.0.pop().unwrap()
    }
    fn push(&mut self, _crate: Crate) {
        self.0.push(_crate);
    }
}

#[derive(Debug)]
struct Stacks(pub Vec<Stack>);
impl Stacks {
    pub fn move_part_1(&mut self, _move: Move) {
        let stacks = &mut self.0;

        let from = _move.from - 1;
        let to = _move.to - 1;

        for _ in 0.._move.count {
            let from = stacks.get_mut(from).unwrap();
            let pop = from.pop();

            let to = stacks.get_mut(to).unwrap();
            to.push(pop);
        }
    }
    pub fn move_part_2(&mut self, _move: Move) {
        let stacks = &mut self.0;

        let from = _move.from - 1;
        let to = _move.to - 1;

        let pops: Vec<Crate> = (0.._move.count)
            .into_iter()
            .map(|_| {
                let from = stacks.get_mut(from).unwrap();
                from.pop()
            })
            .collect();
        let to = stacks.get_mut(to).unwrap();
        pops.iter().rev().for_each(|pop| to.0.push(*pop));
    }
    pub fn top_crates(&self) -> String {
        let mut out = String::new();
        for stack in &self.0 {
            if let Some(c) = stack.0.last() {
                out += c.0.to_string().as_str();
            }
        }
        out
    }
}
type StackId = usize;

type Count = u64;

type Moves = Vec<Move>;
#[derive(Debug)]
struct Move {
    count: Count,
    from: StackId,
    to: StackId,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        use text_io::scan;
        let (count, from, to);
        scan!(s.bytes() => "move {} from {} to {}", count, from, to);
        Self { count, from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(String::from("CMZ"), part_1(INPUT));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(String::from("MCD"), part_2(INPUT));
    }
}
