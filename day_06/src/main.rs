use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    println!("Part 1 answer: {}", part_1(input));
    println!("Part 2 answer: {}", part_2(input));
}

fn part_1(input: &str) -> Answer {
    get_sop_marker_idx(input)
}

fn part_2(input: &str) -> Answer {
    get_som_marker_idx(input)
}
type Answer = usize;
fn get_sop_marker_idx(signal: &str) -> Answer {
    const WINDOW_SIZE: usize = 4;
    let len = signal.len();

    let mut head = 0usize;
    while head < len - WINDOW_SIZE {
        let window = &signal[head..head + WINDOW_SIZE];

        let mut duplicate = false;
        let mut seen = HashSet::new();
        for (_j, ch) in window.chars().enumerate() {
            if !duplicate && seen.contains(&ch) {
                duplicate = true;
            } else {
                seen.insert(ch);
            }
        }
        if !duplicate {
            return head + WINDOW_SIZE;
        } else {
            head += 1
        }
    }
    panic!("No signal start found")
}
fn get_som_marker_idx(signal: &str) -> Answer {
    const WINDOW_SIZE: usize = 14;
    let len = signal.len();

    let mut head = 0usize;
    while head < len - WINDOW_SIZE {
        let window = &signal[head..head + WINDOW_SIZE];

        let mut duplicate = false;
        let mut seen = HashSet::new();
        for (_j, ch) in window.chars().enumerate() {
            if !duplicate && seen.contains(&ch) {
                duplicate = true;
            } else {
                seen.insert(ch);
            }
        }
        if !duplicate {
            return head + WINDOW_SIZE;
        } else {
            head += 1
        }
    }
    panic!("No signal start found")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_sop_marker() {
        assert_eq!(5, get_sop_marker_idx("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, get_sop_marker_idx("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, get_sop_marker_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, get_sop_marker_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
    #[test]
    fn test_get_som_marker() {
        assert_eq!(19, get_som_marker_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, get_som_marker_idx("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, get_som_marker_idx("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, get_som_marker_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, get_som_marker_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
