use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../input");
    let inventories = split_inventories(input);
    let answer = part_1(&inventories);
    println!("Part 1: {answer}");
    let answer = part_2(&inventories);
    println!("Part 2: {answer:?}");
}

type Calories = u32;
type Inventory = Vec<Calories>;

fn split_inventories(str: &str) -> Vec<Inventory> {
    let lines = str.lines().peekable();
    let mut out = vec![];

    let count = lines.clone().count();

    let mut curr_inv = vec![];
    for (idx, line) in lines.enumerate() {
        if idx == count - 1 {
            curr_inv.push(Calories::from_str_radix(line, 10).unwrap());
            out.push(curr_inv.clone());
        } else if line.is_empty() {
            out.push(curr_inv.clone());
            curr_inv.clear();
        } else {
            curr_inv.push(Calories::from_str_radix(line, 10).unwrap());
        }
    }
    out
}

fn part_1(inventories: &[Inventory]) -> Calories {
    inventories
        .iter()
        .map(|inv| inv.iter().sum::<Calories>())
        .max()
        .unwrap()
}
fn part_2(inventories: &[Inventory]) -> Calories {
    const TOP_N: u32 = 3;
    let mut heap: BinaryHeap<Calories> = BinaryHeap::new();
    for inv in inventories {
        heap.push(inv.iter().sum());
    }
    (0..TOP_N).map(|_| heap.pop().unwrap()).sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn test_split_inventories() {
        assert_eq!(
            vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000]
            ],
            split_inventories(INPUT)
        );
    }

    #[test]
    fn test_part_1() {
        let answer = part_1(&split_inventories(INPUT));
        assert_eq!(24000, answer);
    }

    #[test]
    fn test_part_2() {
        let answer = part_2(&split_inventories(INPUT));
        assert_eq!(45000, answer);
    }
}
