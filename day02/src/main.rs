use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    println!("Score 1 {}", get_total_score(&data));
    println!("Score 2 {}", get_total_score_2(&data));
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Attack {
    Rock,
    Paper,
    Scissors,
}

fn get_attack_from_letter(us: char) -> Attack {
    match us {
        'A' | 'X' => Attack::Rock,
        'B' | 'Y' => Attack::Paper,
        'C' | 'Z' => Attack::Scissors,
        _ => panic!("Unknown attack {}", us),
    }
}

// Rock     A X +1
// Paper    B Y +2
// Scissors C Z +3
fn get_bonus_score(us: Attack) -> i32 {
    let map: HashMap<Attack, i32> =
        HashMap::from([(Attack::Rock, 1), (Attack::Paper, 2), (Attack::Scissors, 3)]);
    *map.get(&us).unwrap()
}

fn get_attack_to_win(opponent: Attack) -> Attack {
    match opponent {
        Attack::Rock => Attack::Paper,
        Attack::Paper => Attack::Scissors,
        Attack::Scissors => Attack::Rock,
    }
}

fn get_attack_to_lose(opponent: Attack) -> Attack {
    match opponent {
        Attack::Rock => Attack::Scissors,
        Attack::Paper => Attack::Rock,
        Attack::Scissors => Attack::Paper,
    }
}

fn get_attack_to_draw(opponent: Attack) -> Attack {
    opponent
}

enum MatchResult {
    Win,
    Lose,
    Draw,
}

fn get_match_result(us: char) -> MatchResult {
    match us {
        'X' => MatchResult::Lose,
        'Y' => MatchResult::Draw,
        'Z' => MatchResult::Win,
        _ => panic!("Unknown sign {}", us),
    }
}

// Win   +6
// Lose  +0
// Equal +3
fn get_score(opponent: Attack, us: Attack) -> i32 {
    if opponent == us {
        return 3;
    }
    if us == Attack::Rock {
        return if opponent == Attack::Paper { 0 } else { 6 };
    }
    if us == Attack::Paper {
        return if opponent == Attack::Scissors { 0 } else { 6 };
    }
    if us == Attack::Scissors {
        return if opponent == Attack::Rock { 0 } else { 6 };
    }

    panic!("Unknown attack {:?}", us);
}

fn get_total_score(data: &str) -> i32 {
    data.replace("\r", "")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|tour| tour.split(" "))
        .map(|mut attacks| {
            let opponent = get_attack_from_letter(attacks.next().unwrap().chars().next().unwrap());
            let us = get_attack_from_letter(attacks.next().unwrap().chars().next().unwrap());
            let score = get_score(opponent, us);
            let bonus = get_bonus_score(us);
            score + bonus
        })
        .sum()
}

fn get_total_score_2(data: &str) -> i32 {
    data.replace("\r", "")
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|tour| tour.split(" "))
        .map(|mut attacks| {
            let opponent = get_attack_from_letter(attacks.next().unwrap().chars().next().unwrap());
            let expected_result = get_match_result(attacks.next().unwrap().chars().next().unwrap());
            let us = match expected_result {
                MatchResult::Win => get_attack_to_win(opponent),
                MatchResult::Lose => get_attack_to_lose(opponent),
                MatchResult::Draw => get_attack_to_draw(opponent),
            };

            let score = get_score(opponent, us);
            let bonus = get_bonus_score(us);
            score + bonus
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_from_letter() {
        assert_eq!(get_attack_from_letter('A'), Attack::Rock);
        assert_eq!(get_attack_from_letter('X'), Attack::Rock);
        assert_eq!(get_attack_from_letter('B'), Attack::Paper);
        assert_eq!(get_attack_from_letter('Y'), Attack::Paper);
        assert_eq!(get_attack_from_letter('C'), Attack::Scissors);
        assert_eq!(get_attack_from_letter('Z'), Attack::Scissors);
    }

    #[test]
    fn test_get_score() {
        assert_eq!(get_score(Attack::Rock, Attack::Rock), 3);
        assert_eq!(get_score(Attack::Paper, Attack::Rock), 0);
        assert_eq!(get_score(Attack::Scissors, Attack::Rock), 6);
        assert_eq!(get_score(Attack::Rock, Attack::Paper), 6);
        assert_eq!(get_score(Attack::Paper, Attack::Paper), 3);
        assert_eq!(get_score(Attack::Scissors, Attack::Paper), 0);
        assert_eq!(get_score(Attack::Rock, Attack::Scissors), 0);
        assert_eq!(get_score(Attack::Paper, Attack::Scissors), 6);
        assert_eq!(get_score(Attack::Scissors, Attack::Scissors), 3);
    }

    #[test]
    fn test_bonus_score() {
        assert_eq!(get_bonus_score(Attack::Rock), 1);
        assert_eq!(get_bonus_score(Attack::Paper), 2);
        assert_eq!(get_bonus_score(Attack::Scissors), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            get_total_score(
                "A Y
B X
C Z
"
            ),
            15
        )
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            get_total_score_2(
                "A Y
B X
C Z
"
            ),
            12
        )
    }
}
