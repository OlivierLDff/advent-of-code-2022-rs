use itertools::Itertools;

fn main() {
    // Read input.txt file
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    find_first_marker_offset(input).unwrap() as i32
}

fn part2(input: &str) -> i32 {
    find_start_of_message(input).unwrap() as i32
}

fn find_idx_after_unique_char_count(input: &str, required_len: usize) -> Option<usize> {
    let mut last_chars = Vec::<char>::new();

    for (idx, char) in input.chars().enumerate() {
        if !char.is_alphabetic() {
            return None;
        }

        if last_chars.len() == required_len {
            last_chars.remove(0);
        }

        last_chars.push(char);

        // Check that last_chars are unique
        if last_chars.iter().unique().count() == required_len {
            return Some(idx + 1);
        }
    }
    None
}

fn find_first_marker_offset(input: &str) -> Option<usize> {
    find_idx_after_unique_char_count(input, 4)
}

fn find_start_of_message(input: &str) -> Option<usize> {
    find_idx_after_unique_char_count(input, 14)
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(find_first_marker_offset(""), None);

        assert_eq!(find_first_marker_offset("abc"), None);

        assert_eq!(find_first_marker_offset("ab(c)"), None);

        assert_eq!(find_first_marker_offset("ab(c)d"), None);

        assert_eq!(
            find_first_marker_offset("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(7)
        );

        assert_eq!(
            find_first_marker_offset("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );

        assert_eq!(
            find_first_marker_offset("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );

        assert_eq!(
            find_first_marker_offset("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );

        assert_eq!(
            find_first_marker_offset("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_start_of_message(""), None);
        assert_eq!(find_start_of_message("abc"), None);
        assert_eq!(
            find_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(19)
        );
    }
}
