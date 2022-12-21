use std::collections::HashMap;

type BigInt = usize;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    compute_monkey_business_level(input, 20, &Some(WorryDivider::Enabled))
}

fn part2(input: &str) -> usize {
    compute_monkey_business_level(input, 10000, &None)
}

fn compute_monkey_business_level(
    input: &str,
    iterations: i32,
    worry: &Option<WorryDivider>,
) -> usize {
    let mut monkeys = parse_monkeys(input);
    let items_inspected = run_monkeys_process_for(&mut monkeys, iterations, worry);

    let mut sorted_items = items_inspected
        .iter()
        .map(|(_, &count)| count)
        .collect::<Vec<_>>();

    sorted_items.sort_unstable_by(|a, b| b.cmp(a));
    sorted_items.iter().take(2).product()
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
enum Operand {
    Value(usize),
    Old,
}

#[derive(Debug, PartialEq)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: i32,
    items: Vec<usize>,
    operation: Operation,
    test_divisible_by: usize,
    true_throw_monkey_id: i32,
    false_throw_monkey_id: i32,
}

fn parse_monkey_id(line: &str) -> i32 {
    let (monkey_str, id_str) = line.split_once(" ").unwrap();
    if monkey_str != "Monkey" {
        panic!("Invalid monkey id: {}", line);
    }
    let id_str = id_str.trim_end_matches(":");
    id_str.parse().unwrap()
}

fn parse_starting_items(line: &str) -> Vec<usize> {
    let mut items = Vec::new();
    let mut line = line.trim_start_matches("Starting items: ");
    while let Some((item_str, rest)) = line.split_once(", ") {
        items.push(item_str.parse().unwrap());
        line = rest;
    }
    items.push(line.parse().unwrap());
    items
}

fn parse_operation(line: &str) -> Operation {
    let line = line.trim_start_matches("Operation: new = old ");
    let (operator, operand) = line.split_once(" ").unwrap();
    let operator = match operator {
        "+" => Operator::Add,
        "*" => Operator::Multiply,
        _ => panic!("Invalid operator: {}", operator),
    };
    let operand = match operand {
        "old" => Operand::Old,
        _ => Operand::Value(operand.parse().unwrap()),
    };
    Operation {
        operator: operator,
        operand: operand,
    }
}

fn parse_test_disivible_by(line: &str) -> usize {
    let line = line.trim_start_matches("Test: divisible by ");
    line.parse().unwrap()
}

fn parse_if_true_throw_monkey_id(line: &str) -> i32 {
    let line = line.trim_start_matches("If true: throw to monkey ");
    line.parse().unwrap()
}
fn parse_if_false_throw_monkey_id(line: &str) -> i32 {
    let line = line.trim_start_matches("If false: throw to monkey ");
    line.parse().unwrap()
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let input = input.replace("\r", "");

    let paragraphs = input.split("\n\n");
    for paragraph in paragraphs {
        let mut lines = paragraph.lines();
        let monkey_id = parse_monkey_id(lines.next().unwrap().trim());
        let starting_items = parse_starting_items(lines.next().unwrap().trim());
        let operation = parse_operation(lines.next().unwrap().trim());
        let divisible_by = parse_test_disivible_by(lines.next().unwrap().trim());
        let true_throw_monkey_id = parse_if_true_throw_monkey_id(lines.next().unwrap().trim());
        let false_throw_monkey_id = parse_if_false_throw_monkey_id(lines.next().unwrap().trim());

        monkeys.push(Monkey {
            id: monkey_id,
            items: starting_items,
            operation: operation,
            test_divisible_by: divisible_by,
            true_throw_monkey_id: true_throw_monkey_id,
            false_throw_monkey_id: false_throw_monkey_id,
        });
    }

    monkeys
}

fn get_monkey_by_id(monkeys: &Vec<Monkey>, id: i32) -> &Monkey {
    monkeys.iter().find(|monkey| monkey.id == id).unwrap()
}

fn get_monkey_by_id_mut(monkeys: &mut Vec<Monkey>, id: i32) -> &mut Monkey {
    monkeys.iter_mut().find(|monkey| monkey.id == id).unwrap()
}

#[derive(Debug, PartialEq)]
struct MonkeyProcessResult {
    monkey_id: i32,
    item: usize,
}

fn do_operator(operator: &Operator, item: &usize, value: &BigInt) -> BigInt {
    match operator {
        Operator::Add => item + value,
        Operator::Multiply => item * value,
    }
}

fn do_operation(operation: &Operation, item: &BigInt) -> BigInt {
    match operation.operand {
        Operand::Value(value) => do_operator(&operation.operator, item, &value),
        Operand::Old => do_operator(&operation.operator, item, item),
    }
}

#[derive(Debug, PartialEq, Clone)]
enum WorryDivider {
    Enabled,
    Lcd(BigInt),
}

fn do_monkey_process_item(
    monkey: &Monkey,
    item: &BigInt,
    worry_op: &WorryDivider,
) -> MonkeyProcessResult {
    let item = do_operation(&monkey.operation, &item);
    let item = match worry_op {
        WorryDivider::Enabled => (item as f64 / 3.).floor() as usize,
        WorryDivider::Lcd(lcd) => item % lcd,
    };

    let throw_monkey_id = if &item % BigInt::from(monkey.test_divisible_by) == 0 {
        monkey.true_throw_monkey_id
    } else {
        monkey.false_throw_monkey_id
    };

    MonkeyProcessResult {
        monkey_id: throw_monkey_id,
        item: item,
    }
}

// Process all items for a single monkey
fn do_monkey_process_items(monkey: &mut Monkey, worry: &WorryDivider) -> Vec<MonkeyProcessResult> {
    let mut results = Vec::new();
    while !monkey.items.is_empty() {
        let item = monkey.items.remove(0);
        let result = do_monkey_process_item(monkey, &item, &worry);
        results.push(result);
    }
    results
}

fn do_monkeys_process(
    monkeys: &mut Vec<Monkey>,
    worry: &Option<WorryDivider>,
) -> HashMap<i32, usize> {
    let mut items_manipulated_per_monkeys = HashMap::new();
    let monkey_ids = monkeys.iter().map(|monkey| monkey.id).collect::<Vec<i32>>();
    let lcd: BigInt = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product();
    let worry = match worry {
        None => WorryDivider::Lcd(lcd),
        Some(worry) => worry.clone(),
    };

    for monkey_id in monkey_ids {
        let mut monkey = get_monkey_by_id_mut(monkeys, monkey_id);
        *items_manipulated_per_monkeys.entry(monkey_id).or_insert(0) += monkey.items.len();
        let results = do_monkey_process_items(&mut monkey, &worry);
        for result in results {
            let monkey = get_monkey_by_id_mut(monkeys, result.monkey_id);
            monkey.items.push(result.item);
        }
    }
    items_manipulated_per_monkeys
}

fn run_monkeys_process_for(
    monkeys: &mut Vec<Monkey>,
    iterations: i32,
    worry: &Option<WorryDivider>,
) -> HashMap<i32, usize> {
    let mut items_manipulated_per_monkeys = HashMap::new();
    for _ in 0..iterations {
        let round_items_manipulated = do_monkeys_process(monkeys, &worry);
        for (monkey_id, items_manipulated) in round_items_manipulated {
            *items_manipulated_per_monkeys.entry(monkey_id).or_insert(0) += items_manipulated;
        }
    }
    items_manipulated_per_monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    static _EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(_EXAMPLE_INPUT), 10605);
    }

    fn get_example_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                id: 0,
                items: vec![79, 98],
                operation: Operation {
                    operator: Operator::Multiply,
                    operand: Operand::Value(19),
                },
                test_divisible_by: 23,
                true_throw_monkey_id: 2,
                false_throw_monkey_id: 3,
            },
            Monkey {
                id: 1,
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Value(6),
                },
                test_divisible_by: 19,
                true_throw_monkey_id: 2,
                false_throw_monkey_id: 0,
            },
            Monkey {
                id: 2,
                items: vec![79, 60, 97],
                operation: Operation {
                    operator: Operator::Multiply,
                    operand: Operand::Old,
                },
                test_divisible_by: 13,
                true_throw_monkey_id: 1,
                false_throw_monkey_id: 3,
            },
            Monkey {
                id: 3,
                items: vec![74],
                operation: Operation {
                    operator: Operator::Add,
                    operand: Operand::Value(3),
                },
                test_divisible_by: 17,
                true_throw_monkey_id: 0,
                false_throw_monkey_id: 1,
            },
        ]
    }

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(_EXAMPLE_INPUT);
        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys, get_example_monkeys())
    }

    #[test]
    fn test_parse_monkey_id() {
        assert_eq!(parse_monkey_id("Monkey 0:"), 0);
        assert_eq!(parse_monkey_id("Monkey 4:"), 4);
    }

    #[test]
    fn test_parse_starting_items() {
        assert_eq!(parse_starting_items("Starting items: 79, 98"), vec![79, 98]);
        assert_eq!(
            parse_starting_items("Starting items: 54, 65, 75, 74"),
            vec![54, 65, 75, 74]
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("Operation: new = old + 6"),
            Operation {
                operator: Operator::Add,
                operand: Operand::Value(6)
            }
        );
        assert_eq!(
            parse_operation("Operation: new = old * 19"),
            Operation {
                operator: Operator::Multiply,
                operand: Operand::Value(19)
            }
        );
        assert_eq!(
            parse_operation("Operation: new = old * old"),
            Operation {
                operator: Operator::Multiply,
                operand: Operand::Old
            }
        );
        assert_eq!(
            parse_operation("Operation: new = old + old"),
            Operation {
                operator: Operator::Add,
                operand: Operand::Old
            }
        );
        assert_eq!(
            parse_operation("Operation: new = old + 3"),
            Operation {
                operator: Operator::Add,
                operand: Operand::Value(3)
            }
        );
    }

    #[test]
    fn test_parse_divisible_by() {
        assert_eq!(parse_test_disivible_by("Test: divisible by 23"), 23);
        assert_eq!(parse_test_disivible_by("Test: divisible by 19"), 19);
        assert_eq!(parse_test_disivible_by("Test: divisible by 13"), 13);
        assert_eq!(parse_test_disivible_by("Test: divisible by 17"), 17);
    }

    #[test]
    fn test_parse_if_true_throw_monkey_id() {
        assert_eq!(
            parse_if_true_throw_monkey_id("If true: throw to monkey 2"),
            2
        );
        assert_eq!(
            parse_if_true_throw_monkey_id("If true: throw to monkey 1"),
            1
        );
        assert_eq!(
            parse_if_true_throw_monkey_id("If true: throw to monkey 0"),
            0
        );
        assert_eq!(
            parse_if_true_throw_monkey_id("If true: throw to monkey 3"),
            3
        );
    }

    #[test]
    fn test_parse_if_false_throw_monkey_id() {
        assert_eq!(
            parse_if_false_throw_monkey_id("If false: throw to monkey 3"),
            3
        );
        assert_eq!(
            parse_if_false_throw_monkey_id("If false: throw to monkey 0"),
            0
        );
        assert_eq!(
            parse_if_false_throw_monkey_id("If false: throw to monkey 1"),
            1
        );
        assert_eq!(
            parse_if_false_throw_monkey_id("If false: throw to monkey 2"),
            2
        );
    }

    #[test]
    fn test_do_monkey_process_item() {
        assert_eq!(
            do_monkey_process_item(
                &Monkey {
                    id: 0,
                    items: vec![98],
                    operation: Operation {
                        operator: Operator::Multiply,
                        operand: Operand::Value(19)
                    },
                    test_divisible_by: 23,
                    true_throw_monkey_id: 2,
                    false_throw_monkey_id: 3
                },
                &79,
                &WorryDivider::Enabled
            ),
            MonkeyProcessResult {
                monkey_id: 3,
                item: 500
            }
        );
    }

    #[test]
    fn test_do_monkey_process_items() {
        assert_eq!(
            do_monkey_process_items(
                &mut Monkey {
                    id: 0,
                    items: vec![79, 98],
                    operation: Operation {
                        operator: Operator::Multiply,
                        operand: Operand::Value(19)
                    },
                    test_divisible_by: 23,
                    true_throw_monkey_id: 2,
                    false_throw_monkey_id: 3
                },
                &WorryDivider::Enabled
            ),
            vec![
                MonkeyProcessResult {
                    monkey_id: 3,
                    item: 500
                },
                MonkeyProcessResult {
                    monkey_id: 3,
                    item: 620
                }
            ]
        );
    }

    #[test]
    fn test_do_monkeys_process() {
        let mut monkeys = get_example_monkeys();

        // Round 1
        do_monkeys_process(&mut monkeys, &Some(WorryDivider::Enabled));
        assert_eq!(get_monkey_by_id(&monkeys, 0).items, vec![20, 23, 27, 26]);
        assert_eq!(
            get_monkey_by_id(&monkeys, 1).items,
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert!(get_monkey_by_id(&monkeys, 2).items.is_empty());
        assert!(get_monkey_by_id(&monkeys, 3).items.is_empty());

        // Round 2
        do_monkeys_process(&mut monkeys, &Some(WorryDivider::Enabled));
        assert_eq!(
            get_monkey_by_id(&monkeys, 0).items,
            vec![695, 10, 71, 135, 350]
        );
        assert_eq!(
            get_monkey_by_id(&monkeys, 1).items,
            vec![43, 49, 58, 55, 362]
        );
        assert!(get_monkey_by_id(&monkeys, 2).items.is_empty());
        assert!(get_monkey_by_id(&monkeys, 3).items.is_empty());

        // Round 3
        do_monkeys_process(&mut monkeys, &Some(WorryDivider::Enabled));
        assert_eq!(
            get_monkey_by_id(&monkeys, 0).items,
            vec![16, 18, 21, 20, 122]
        );
        assert_eq!(
            get_monkey_by_id(&monkeys, 1).items,
            vec![1468, 22, 150, 286, 739]
        );
        assert!(get_monkey_by_id(&monkeys, 2).items.is_empty());
        assert!(get_monkey_by_id(&monkeys, 3).items.is_empty());

        // Round 4
        do_monkeys_process(&mut monkeys, &Some(WorryDivider::Enabled));
        assert_eq!(
            get_monkey_by_id(&monkeys, 0).items,
            vec![491, 9, 52, 97, 248, 34]
        );
        assert_eq!(get_monkey_by_id(&monkeys, 1).items, vec![39, 45, 43, 258]);
        assert!(get_monkey_by_id(&monkeys, 2).items.is_empty());
        assert!(get_monkey_by_id(&monkeys, 3).items.is_empty());

        // Round 5
        do_monkeys_process(&mut monkeys, &Some(WorryDivider::Enabled));
        assert_eq!(
            get_monkey_by_id(&monkeys, 0).items,
            vec![15, 17, 16, 88, 1037]
        );
        assert_eq!(
            get_monkey_by_id(&monkeys, 1).items,
            vec![20, 110, 205, 524, 72]
        );
        assert!(get_monkey_by_id(&monkeys, 2).items.is_empty());
        assert!(get_monkey_by_id(&monkeys, 3).items.is_empty());
    }

    #[test]
    fn test_run_monkeys_process() {
        let mut monkeys = get_example_monkeys();

        let items_manipulated =
            run_monkeys_process_for(&mut monkeys, 20, &Some(WorryDivider::Enabled));

        assert_eq!(items_manipulated.len(), 4);
        assert_eq!(items_manipulated[&0], 101);
        assert_eq!(items_manipulated[&1], 95);
        assert_eq!(items_manipulated[&2], 7);
        assert_eq!(items_manipulated[&3], 105);
    }

    #[test]
    fn test_run_monkeys_process_no_worry_divided_1() {
        let mut monkeys = get_example_monkeys();

        let items_manipulated = run_monkeys_process_for(&mut monkeys, 1, &None);

        assert_eq!(items_manipulated.len(), 4);
        assert_eq!(items_manipulated[&0], 2);
        assert_eq!(items_manipulated[&1], 4);
        assert_eq!(items_manipulated[&2], 3);
        assert_eq!(items_manipulated[&3], 6);
    }

    #[test]
    fn test_run_monkeys_process_no_worry_divided_20() {
        let mut monkeys = get_example_monkeys();

        let items_manipulated = run_monkeys_process_for(&mut monkeys, 20, &None);

        assert_eq!(items_manipulated.len(), 4);
        assert_eq!(items_manipulated[&0], 99);
        assert_eq!(items_manipulated[&1], 97);
        assert_eq!(items_manipulated[&2], 8);
        assert_eq!(items_manipulated[&3], 103);
    }

    #[test]
    fn test_run_monkeys_process_no_worry_divided_1000() {
        let mut monkeys = get_example_monkeys();

        let items_manipulated = run_monkeys_process_for(&mut monkeys, 1000, &None);

        assert_eq!(items_manipulated.len(), 4);
        assert_eq!(items_manipulated[&0], 5204);
        assert_eq!(items_manipulated[&1], 4792);
        assert_eq!(items_manipulated[&2], 199);
        assert_eq!(items_manipulated[&3], 5192);
    }

    #[test]
    fn test_run_monkeys_process_no_worry_10000() {
        let mut monkeys = get_example_monkeys();

        let items_manipulated = run_monkeys_process_for(&mut monkeys, 10000, &None);

        assert_eq!(items_manipulated.len(), 4);
        assert_eq!(items_manipulated[&0], 52166);
        assert_eq!(items_manipulated[&1], 47830);
        assert_eq!(items_manipulated[&2], 1938);
        assert_eq!(items_manipulated[&3], 52013);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(_EXAMPLE_INPUT), 2713310158);
    }
}
