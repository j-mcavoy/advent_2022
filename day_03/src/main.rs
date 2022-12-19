mod core;
mod part_1;
mod part_2;

use part_1::part_1;
use part_2::part_2;

fn main() {
    let input = include_str!("../input");
    println!("Part 1 Answer: {}", part_1(input));
    println!("Part 2 Answer: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    
    pub const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
