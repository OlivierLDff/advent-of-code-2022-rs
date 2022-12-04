use itertools::Itertools;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    println!("part1 {}", part1(&data));
    println!("part2 {}", part2(&data));
}

type IdsRange = Vec<Vec<Vec<i32>>>;

fn get_ranges_per_pair_per_elf(data: &str) -> IdsRange {
    data.lines()
        .map(|l| l.split(","))
        .map(|split| {
            split
                .into_iter()
                .map(|s| s.split("-").filter_map(|n| n.parse::<i32>().ok()))
                .filter_map(|mut n| {
                    let first = n.next()?;
                    let second = n.next()?;
                    Some((first..second + 1).collect())
                })
                .collect()
        })
        .collect()
}

fn get_duplicates_per_elf_pair(ranges: &IdsRange) -> Vec<usize> {
    ranges
        .iter()
        .map(|ids_range| ids_range.concat().into_iter().duplicates().count())
        .collect()
}

fn part1(data: &str) -> usize {
    let ranges = get_ranges_per_pair_per_elf(data);
    ranges
        .iter()
        .zip(get_duplicates_per_elf_pair(&ranges).iter())
        .filter(|(range, &duplicate_count)| range.iter().any(|r| r.len() == duplicate_count))
        .count()
}

fn part2(data: &str) -> usize {
    get_duplicates_per_elf_pair(&get_ranges_per_pair_per_elf(data))
        .iter()
        .filter(|&&d| d > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_get_ranges_per_pair_per_elf() {
        assert_eq!(
            get_ranges_per_pair_per_elf(EXAMPLE),
            vec![
                vec![vec![2, 3, 4], vec![6, 7, 8]],
                vec![vec![2, 3], vec![4, 5]],
                vec![vec![5, 6, 7], vec![7, 8, 9]],
                vec![vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]],
                vec![vec![6], vec![4, 5, 6]],
                vec![vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]],
            ]
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4)
    }
}
