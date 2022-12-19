use crate::core::*;

pub fn part_1(s: &str) -> Answer {
    let rucksacks = str2rucksacks(s);
    let mut answer = 0;
    for rs in rucksacks {
        answer += rs.common_item().value() as Answer;
    }
    answer
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part_1() {}
}
