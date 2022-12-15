fn main() {
    let data = get_puzzle_input();
    let stacks = get_stacks();

    let part1 = part1(data, &mut stacks.clone());
    let part1_str = part1.iter().collect::<String>();
    println!("Part 1: {}", part1_str);

    let part2 = part2(data, &mut stacks.clone());
    let part2_str = part2.iter().collect::<String>();
    println!("Part 2: {}", part2_str);
}

fn get_puzzle_input() -> &'static str {
    include_str!("../input.txt")
}

// [Q]         [N]             [N]
// [H]     [B] [D]             [S] [M]
// [C]     [Q] [J]         [V] [Q] [D]
// [T]     [S] [Z] [F]     [J] [J] [W]
// [N] [G] [T] [S] [V]     [B] [C] [C]
// [S] [B] [R] [W] [D] [J] [Q] [R] [Q]
// [V] [D] [W] [G] [P] [W] [N] [T] [S]
// [B] [W] [F] [L] [M] [F] [L] [G] [J]
//  1   2   3   4   5   6   7   8   9
fn get_stacks() -> Stacks {
    vec![
        vec!['B', 'V', 'S', 'N', 'T', 'C', 'H', 'Q'],
        vec!['W', 'D', 'B', 'G'],
        vec!['F', 'W', 'R', 'T', 'S', 'Q', 'B'],
        vec!['L', 'G', 'W', 'S', 'Z', 'J', 'D', 'N'],
        vec!['M', 'P', 'D', 'V', 'F'],
        vec!['F', 'W', 'J'],
        vec!['L', 'N', 'Q', 'B', 'J', 'V'],
        vec!['G', 'T', 'R', 'C', 'J', 'Q', 'S', 'N'],
        vec!['J', 'S', 'Q', 'C', 'W', 'D', 'M'],
    ]
}

type Crate = char;
type CrateStack = Vec<Crate>;
type Stacks = Vec<CrateStack>;

enum CrateMover {
    CreateMover9000,
    CreateMover9001,
}

fn process_stack_with_crate_mover(
    data: &str,
    stacks: &mut Stacks,
    crate_mover: &CrateMover,
) -> Vec<Crate> {
    let ops = get_ops(data);
    _ = apply_ops(&ops, stacks, &crate_mover);
    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&' ').clone())
        .collect()
}

fn part1(data: &str, stacks: &mut Stacks) -> Vec<Crate> {
    process_stack_with_crate_mover(data, stacks, &CrateMover::CreateMover9000)
}

fn part2(data: &str, stacks: &mut Stacks) -> Vec<Crate> {
    process_stack_with_crate_mover(data, stacks, &CrateMover::CreateMover9001)
}

#[derive(Debug, PartialEq)]
struct Op {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Debug, PartialEq)]
enum OpParseError {
    MissingMoveKeyWord,
    MissingFromKeyWord,
    MissingToKeyWord,
    MissingFromIndex,
    MissingToIndex,
    MissingCount,
    TooMuchData,
}

// Get the operation from a line of input
// Input should be formatted "move <count> from <from> to <to>"
fn get_op(line: &str) -> Result<Op, OpParseError> {
    let mut parts = line.split(" ");
    let move_keyword = parts.next().ok_or(OpParseError::MissingMoveKeyWord)?;
    if move_keyword != "move" {
        return Err(OpParseError::MissingMoveKeyWord);
    }
    let move_count = parts
        .next()
        .ok_or(OpParseError::MissingCount)?
        .parse::<usize>()
        .map_err(|_| OpParseError::MissingCount)?;

    let from_keyword = parts.next().ok_or(OpParseError::MissingFromKeyWord)?;
    if from_keyword != "from" {
        return Err(OpParseError::MissingFromKeyWord);
    }
    let from_index = parts
        .next()
        .ok_or(OpParseError::MissingFromIndex)?
        .parse::<usize>()
        .map_err(|_| OpParseError::MissingFromIndex)?;

    let to_keyword = parts.next().ok_or(OpParseError::MissingToKeyWord)?;
    if to_keyword != "to" {
        return Err(OpParseError::MissingToKeyWord);
    }
    let to_index = parts
        .next()
        .ok_or(OpParseError::MissingToIndex)?
        .parse::<usize>()
        .map_err(|_| OpParseError::MissingToIndex)?;

    if parts.next().is_some() {
        return Err(OpParseError::TooMuchData);
    }

    Ok(Op {
        from: from_index,
        to: to_index,
        count: move_count,
    })
}

fn get_ops(data: &str) -> Vec<Op> {
    data.lines().filter_map(|line| get_op(line).ok()).collect()
}

#[derive(Debug, PartialEq)]
enum ApplyOpError {
    InvalidFromIndex(usize),
    InvalidToIndex(usize),
    InvalidCount(usize),
}

fn apply_op(op: &Op, stacks: &mut Stacks, crate_mover: &CrateMover) -> Option<ApplyOpError> {
    if op.from == op.to {
        return None;
    }

    if op.from < 1 || op.from > stacks.len() {
        return Some(ApplyOpError::InvalidFromIndex(op.from));
    }

    if op.to < 1 || op.to > stacks.len() {
        return Some(ApplyOpError::InvalidToIndex(op.to));
    }

    let max_idx = std::cmp::max(op.from, op.to);
    let (stack_left, stack_right) = stacks.split_at_mut(max_idx - 1);

    let (from_stack, to_stack) = {
        if op.from < op.to {
            let from_stack = &mut stack_left[op.from - 1];
            let to_stack = &mut stack_right[0];

            (from_stack, to_stack)
        } else {
            let to_stack = &mut stack_left[op.to - 1];
            let from_stack = &mut stack_right[0];

            (from_stack, to_stack)
        }
    };

    if op.count > from_stack.len() {
        return Some(ApplyOpError::InvalidCount(op.count));
    }

    let from_stack_ref = &from_stack[from_stack.len() - op.count..];
    match crate_mover {
        CrateMover::CreateMover9000 => to_stack.extend(from_stack_ref.iter().rev()),
        CrateMover::CreateMover9001 => to_stack.extend(from_stack_ref.iter()),
    }
    from_stack.resize(from_stack.len() - op.count, ' ');

    None
}

fn apply_ops(
    ops: &Vec<Op>,
    stacks: &mut Stacks,
    crate_mover: &CrateMover,
) -> Vec<Option<ApplyOpError>> {
    ops.iter()
        .map(|op| apply_op(op, stacks, &crate_mover))
        .collect()
}

mod tests {
    use super::*;

    static _TEST_DATA: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_get_op() {
        assert_eq!(
            get_op("move 1 from 2 to 1").unwrap(),
            Op {
                from: 2,
                to: 1,
                count: 1
            }
        );
        assert_eq!(
            get_op("move 3 from 1 to 3").unwrap(),
            Op {
                from: 1,
                to: 3,
                count: 3
            }
        );
        assert_eq!(
            get_op("move 2 from 5 to 1").unwrap(),
            Op {
                from: 5,
                to: 1,
                count: 2
            }
        );
        assert_eq!(
            get_op("move 1 from 1 to 2").unwrap(),
            Op {
                from: 1,
                to: 2,
                count: 1
            }
        );
        // Test missing move keyword
        assert_eq!(
            get_op("1 from 2 to 1").unwrap_err(),
            OpParseError::MissingMoveKeyWord
        );

        // Test missing from keyword
        assert_eq!(
            get_op("move 1 2 to 1").unwrap_err(),
            OpParseError::MissingFromKeyWord
        );

        // Test missing to keyword
        assert_eq!(
            get_op("move 1 from 2 1").unwrap_err(),
            OpParseError::MissingToKeyWord
        );

        // Test missing from index
        assert_eq!(
            get_op("move 1 from to 1").unwrap_err(),
            OpParseError::MissingFromIndex
        );

        // Test missing to index
        assert_eq!(
            get_op("move 1 from 2 to").unwrap_err(),
            OpParseError::MissingToIndex
        );

        // Test missing count
        assert_eq!(
            get_op("move from 2 to 1").unwrap_err(),
            OpParseError::MissingCount
        );

        // test error on empty string
        assert_eq!(get_op("").unwrap_err(), OpParseError::MissingMoveKeyWord);

        // test erroneous string
        assert_eq!(
            get_op("move 1 from 2 to 1 move 3 from 1 to 3").unwrap_err(),
            OpParseError::TooMuchData
        );
    }

    fn _get_tests_stacks() -> Stacks {
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
    }

    #[test]
    fn test_apply_op() {
        let mut stacks = _get_tests_stacks();
        assert_eq!(
            apply_op(
                &Op {
                    count: 1,
                    from: 2,
                    to: 1,
                },
                &mut stacks,
                &CrateMover::CreateMover9000
            ),
            None
        );

        assert_eq!(stacks, vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]);

        assert_eq!(
            apply_op(
                &Op {
                    count: 3,
                    from: 1,
                    to: 3,
                },
                &mut stacks,
                &CrateMover::CreateMover9000
            ),
            None
        );

        assert_eq!(
            stacks,
            vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(_TEST_DATA, &mut _get_tests_stacks()),
            vec!['C', 'M', 'Z']
        )
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(_TEST_DATA, &mut _get_tests_stacks()),
            vec!['M', 'C', 'D']
        )
    }
}
