use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap().replace("\r", "");
    println!("sum of priorities: {}", sum_of_priorities(&data));
    println!(
        "sum of group priorities: {}",
        sum_of_group_priorities(&data)
    );
}

fn sum_of_priorities(data: &str) -> i32 {
    data.split("\n")
        .map(|line| {
            let len = line.len();
            let half_len = len / 2;
            let first_comp = &line[0..half_len];
            let second_comp = &line[half_len..len];
            first_comp
                .chars()
                .filter(|c| second_comp.contains(*c))
                .unique()
                .map(|c| item_to_priority(c))
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn sum_of_group_priorities(data: &str) -> i32 {
    data.split("\n")
        .chunks(3)
        .into_iter()
        .filter_map(|mut chunk| {
            let mut set: HashSet<_> = chunk.next()?.chars().collect();
            for rucksacks in chunk {
                set.retain(|e| rucksacks.chars().contains(e))
            }
            set.into_iter().next()
        })
        .map(|c| item_to_priority(c))
        .sum()
}

fn item_to_priority(c: char) -> i32 {
    assert!(c.is_alphabetic());
    if c.is_lowercase() {
        (c as u8 - b'a' + 1) as i32
    } else {
        (c as u8 - b'A' + 27) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_item_priority() {
        assert_eq!(item_to_priority('c'), 3);
        assert_eq!(item_to_priority('C'), 29);
    }

    #[test]
    fn test_part1() {
        assert_eq!(sum_of_priorities(EXAMPLE), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_of_group_priorities(EXAMPLE), 70);
    }
}
