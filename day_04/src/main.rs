mod id;
mod range;
use range::*;
fn main() {
    let input = include_str!("../input");
    println!("Part 1 answer: {}", part_1(input));
    println!("Part 2 answer: {}", part_2(input));
}

type Answer = u32;
fn part_1(_s: &str) -> Answer {
    let input = include_str!("../input");
    let ranges = str2ranges(input);
    ranges
        .iter()
        .map(|r| Range::fully_intersects(&r.0, &r.1))
        .filter(|result| *result)
        .count() as Answer
}

fn part_2(_s: &str) -> Answer {
    let input = include_str!("../input");
    let ranges = str2ranges(input);
    ranges
        .iter()
        .map(|r| Range::intersects(&r.0, &r.1))
        .filter(|result| *result)
        .count() as Answer
}
fn str2ranges(s: &str) -> Vec<(Range, Range)> {
    s.lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            assert_eq!(split.len(), 2);
            let (lhs, rhs) = (split.first().unwrap(), split.get(1).unwrap());

            let left = lhs.split_once('-').unwrap();
            let right = rhs.split_once('-').unwrap();

            (left.try_into().unwrap(), right.try_into().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_str2ranges() {
        assert_eq!(
            vec![
                (Range::new(2, 4), Range::new(6, 8)),
                (Range::new(2, 3), Range::new(4, 5)),
                (Range::new(5, 7), Range::new(7, 9)),
                (Range::new(2, 8), Range::new(3, 7)),
                (Range::new(6, 6), Range::new(4, 6)),
                (Range::new(2, 6), Range::new(4, 8))
            ],
            str2ranges(INPUT)
        );
    }
}
