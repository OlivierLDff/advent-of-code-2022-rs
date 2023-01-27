use anyhow::Context;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Number(u32),
    List(Vec<Value>),
}

type ValuePair = (Value, Value);

fn part1(input: &str) -> usize {
    parse_input_as_list_of_pairs(input)
        .unwrap()
        .iter()
        .enumerate()
        .map(|(idx, (left, right))| {
            if let Some(result) = is_in_order(left, right) {
                match result {
                    true => idx + 1,
                    false => 0,
                }
            } else {
                panic!("Unexpected equal values: {:?} {:?}", left, right);
            }
        })
        .sum()
}

fn parse_value(value: &str) -> anyhow::Result<Value> {
    // Iterate over each character in the string
    // If we see a [ then we know we're starting a list
    // If we see a ] then we know we're ending a list
    // If we see a , then we know we're ending a value
    // If we see a number then we know we're starting a value
    let value = value.trim();

    let mut current_lists = Vec::new();

    let mut current_string: Option<String> = None;

    for c in value.chars() {
        match c {
            '[' => {
                current_lists.push(Value::List(Vec::new()));
            }
            ']' | ',' => {
                if let Some(current_string) = current_string.take() {
                    let value_int = current_string.parse::<u32>()?;
                    if let Some(last_value) = current_lists.last_mut() {
                        if let Value::List(previous_list) = last_value {
                            previous_list.push(Value::Number(value_int));
                        } else {
                            panic!("Expected Value::List in current_lists");
                        }
                    } else {
                        panic!("Expected Value::List in current_lists");
                    }
                }

                if c == ']' {
                    let list = current_lists.pop().context("Read ] without matching [")?;
                    if let Some(last_value) = current_lists.last_mut() {
                        if let Value::List(previous_list) = last_value {
                            previous_list.push(list);
                        } else {
                            panic!("Expected Value::List in current_lists");
                        }
                    } else {
                        return Ok(list);
                    }
                }
            }
            '0'..='9' => {
                if current_string.is_none() {
                    current_string = Some(String::new());
                }
                current_string.as_mut().unwrap().push(c)
            }
            c => anyhow::bail!("Unexpected character: {}", c),
        }
    }

    anyhow::bail!("Unclosed list, missing ] for {:?}", value)
}

fn parse_value_pair(input: &str) -> anyhow::Result<(Value, Value)> {
    let mut parts = input.split("\n");
    let left = parts.next().context("Left isn't present")?;
    let right = parts.next().context("Right isn't present")?;
    Ok((
        parse_value(left).context("Fail to parse left")?,
        parse_value(right).context("Fail to parse right")?,
    ))
}

/// Create pairs of values from input
/// Each pair is separated by a blank line
/// Each value is separated by a newline
///
/// # Arguments
/// * `input` - The input string
fn parse_input_as_list_of_pairs(input: &str) -> anyhow::Result<Vec<ValuePair>> {
    let input = input.replace("\r", "\n");
    input.split("\n\n").map(parse_value_pair).collect()
}

/// Create a list of values from input
/// Each value is separated by a newline
/// Blank lines are ignored
///
/// # Arguments
///
/// * `input` - The input string
///
/// # Returns
///
/// A list of values parsed
///
/// # Errors
///
/// If any of the values fail to parse
fn parse_input_as_list_of_values(input: &str) -> anyhow::Result<Vec<Value>> {
    let input = input.replace("\r", "\n");
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_value)
        .collect()
}

fn is_in_order(left: &Value, right: &Value) -> Option<bool> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => match left.cmp(right) {
            std::cmp::Ordering::Less => Some(true),
            std::cmp::Ordering::Greater => Some(false),
            std::cmp::Ordering::Equal => None,
        },
        (Value::List(left), Value::List(right)) => {
            for (left, right) in left.iter().zip(right.iter()) {
                if let Some(result) = is_in_order(left, right) {
                    return Some(result);
                }
            }
            match left.len().cmp(&right.len()) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Greater => Some(false),
                std::cmp::Ordering::Equal => None,
            }
        }
        (Value::Number(left), right) => {
            is_in_order(&Value::List(vec![Value::Number(*left)]), right)
        }
        (left, Value::Number(right)) => {
            is_in_order(left, &Value::List(vec![Value::Number(*right)]))
        }
    }
}

fn cmp_values(left: &Value, right: &Value) -> std::cmp::Ordering {
    if let Some(result) = is_in_order(left, right) {
        if result {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    } else {
        std::cmp::Ordering::Equal
    }
}

fn sort_values(values: &mut [Value]) {
    values.sort_by(cmp_values);
}

fn get_divider_packets() -> anyhow::Result<Vec<Value>> {
    parse_input_as_list_of_values(include_str!("../dividerpackets.txt"))
}

fn part2(data: &str) -> usize {
    let mut parsed = parse_input_as_list_of_values(data).unwrap();
    let divider_packets = get_divider_packets().unwrap();
    parsed.extend_from_slice(&divider_packets);
    sort_values(&mut parsed);

    divider_packets
        .iter()
        .map(|divider_packet| {
            parsed
                .iter()
                .position(|value| value == divider_packet)
                .unwrap()
                + 1
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_data() -> Vec<ValuePair> {
        vec![
            (
                Value::List(vec![
                    Value::Number(1),
                    Value::Number(1),
                    Value::Number(3),
                    Value::Number(1),
                    Value::Number(1),
                ]),
                Value::List(vec![
                    Value::Number(1),
                    Value::Number(1),
                    Value::Number(5),
                    Value::Number(1),
                    Value::Number(1),
                ]),
            ),
            (
                Value::List(vec![
                    Value::List(vec![Value::Number(1)]),
                    Value::List(vec![Value::Number(2), Value::Number(3), Value::Number(4)]),
                ]),
                Value::List(vec![Value::List(vec![Value::Number(1)]), Value::Number(4)]),
            ),
            (
                Value::List(vec![Value::Number(9)]),
                Value::List(vec![Value::List(vec![
                    Value::Number(8),
                    Value::Number(7),
                    Value::Number(6),
                ])]),
            ),
            (
                Value::List(vec![
                    Value::List(vec![Value::Number(4), Value::Number(4)]),
                    Value::Number(4),
                    Value::Number(4),
                ]),
                Value::List(vec![
                    Value::List(vec![Value::Number(4), Value::Number(4)]),
                    Value::Number(4),
                    Value::Number(4),
                    Value::Number(4),
                ]),
            ),
            (
                Value::List(vec![
                    Value::Number(7),
                    Value::Number(7),
                    Value::Number(7),
                    Value::Number(7),
                ]),
                Value::List(vec![Value::Number(7), Value::Number(7), Value::Number(7)]),
            ),
            (Value::List(vec![]), Value::List(vec![Value::Number(3)])),
            (
                Value::List(vec![Value::List(vec![Value::List(vec![])])]),
                Value::List(vec![Value::List(vec![])]),
            ),
            (
                Value::List(vec![
                    Value::Number(1),
                    Value::List(vec![
                        Value::Number(2),
                        Value::List(vec![
                            Value::Number(3),
                            Value::List(vec![
                                Value::Number(4),
                                Value::List(vec![
                                    Value::Number(5),
                                    Value::Number(6),
                                    Value::Number(7),
                                ]),
                            ]),
                        ]),
                    ]),
                    Value::Number(8),
                    Value::Number(9),
                ]),
                Value::List(vec![
                    Value::Number(1),
                    Value::List(vec![
                        Value::Number(2),
                        Value::List(vec![
                            Value::Number(3),
                            Value::List(vec![
                                Value::Number(4),
                                Value::List(vec![
                                    Value::Number(5),
                                    Value::Number(6),
                                    Value::Number(0),
                                ]),
                            ]),
                        ]),
                    ]),
                    Value::Number(8),
                    Value::Number(9),
                ]),
            ),
        ]
    }

    #[test]
    fn parse_example() {
        let input = include_str!("../example.txt");
        let parsed = parse_input_as_list_of_pairs(input).unwrap();
        assert_eq!(parsed, get_example_data());
    }

    #[test]
    fn test_is_in_order_1() {
        assert!(is_in_order(
            &parse_value("[1,1,3,1,1]").unwrap(),
            &parse_value("[1,1,5,1,1]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_2() {
        assert!(is_in_order(
            &parse_value("[[1],[2,3,4]]").unwrap(),
            &parse_value("[[1],4]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_3() {
        assert!(!is_in_order(
            &parse_value("[9]").unwrap(),
            &parse_value("[[8,7,6]]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_4() {
        assert!(is_in_order(
            &parse_value("[[4,4],4,4]").unwrap(),
            &parse_value("[[4,4],4,4,4]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_5() {
        assert!(!is_in_order(
            &parse_value("[7,7,7,7]").unwrap(),
            &parse_value("[7,7,7]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_6() {
        assert!(is_in_order(&parse_value("[]").unwrap(), &parse_value("[3]").unwrap()).unwrap());
    }

    #[test]
    fn test_is_in_order_7() {
        assert!(!is_in_order(
            &parse_value("[[[]]]").unwrap(),
            &parse_value("[[]]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_is_in_order_8() {
        assert!(!is_in_order(
            &parse_value("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
            &parse_value("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap()
        )
        .unwrap());
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_parse_as_list() {
        assert_eq!(
            parse_input_as_list_of_values("[1,2,3]").unwrap(),
            vec![Value::List(vec![
                Value::Number(1),
                Value::Number(2),
                Value::Number(3)
            ])]
        );
    }

    #[test]
    fn test_cmp_values_1() {
        assert_eq!(
            cmp_values(
                &parse_value("[1,2,3]").unwrap(),
                &parse_value("[1,2,3]").unwrap()
            ),
            std::cmp::Ordering::Equal
        );
    }

    #[test]
    fn test_cmd_values_2() {
        assert_eq!(
            cmp_values(
                &parse_value("[1,2,3]").unwrap(),
                &parse_value("[1,2,4]").unwrap()
            ),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_cmd_values_3() {
        assert_eq!(
            cmp_values(
                &parse_value("[1,2,4]").unwrap(),
                &parse_value("[1,2,3]").unwrap()
            ),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn test_cmd_values_4() {
        assert_eq!(
            cmp_values(&parse_value("[[2]]").unwrap(), &parse_value("[3]").unwrap()),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_order_values() {
        let input = include_str!("../example.txt");
        let mut parsed = parse_input_as_list_of_values(input).unwrap();
        parsed.append(&mut get_divider_packets().unwrap());
        sort_values(&mut parsed);
        assert_eq!(
            parsed,
            parse_input_as_list_of_values(
                "
[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[[2]]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[[6]]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]
        "
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 140);
    }
}
