use crate::core::{rucksack::Rucksack, *};


pub fn part_2(s: &str) -> Answer {
    const BADGE_SIZE: usize = 3;
    let rucksacks = str2rucksacks(s);
    rucksacks
        .chunks(BADGE_SIZE)
        .map(|c| Rucksack::badge(&c[0], &c[1], &c[2]).value() as Answer)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::INPUT;

    #[test]
    fn test_part_2() {
        assert_eq!(70, part_2(INPUT));
    }
}
