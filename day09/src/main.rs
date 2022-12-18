fn main() {
    let input = include_str!("../input.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(data: &str) -> usize {
    let instructions = parse_instructions(data);
    let mut rope = Rope {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
    };
    // Unique points visited by the rope
    let mut visited_points = std::collections::HashSet::new();
    visited_points.insert(rope.tail);

    for instruction in instructions {
        for _ in 0..instruction.count {
            move_rope(&mut rope, &instruction.direction);
            visited_points.insert(rope.tail);
        }
    }

    visited_points.len()
}

fn part2(data: &str) -> usize {
    let instructions = parse_instructions(data);
    let mut visited_points = std::collections::HashSet::new();
    let mut rope_points = (0..10).map(|_| Point { x: 0, y: 0 }).collect::<Vec<_>>();
    visited_points.insert(rope_points[rope_points.len() - 1]);

    for instruction in instructions {
        // println!(" Instruction: {:?}", instruction);
        for _ in 0..instruction.count {
            move_point(&mut rope_points[0], &instruction.direction);

            for i in 0..(rope_points.len() - 1) {
                let (head, tail) = rope_points.split_at_mut(i + 1);
                let head = &head[head.len() - 1];
                let tail = &mut tail[0];
                make_point_follow_point(head, tail);
            }

            visited_points.insert(rope_points[rope_points.len() - 1]);
            // print_points_in_grid(rope_points.as_slice(), 0, 6, 0, 6);
        }
    }

    visited_points.len()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Intruction {
    direction: Direction,
    count: usize,
}

fn parse_instruction(s: &str) -> Intruction {
    let mut splitted_str = s.split(" ");
    let direction = splitted_str.next().unwrap();
    let count = splitted_str.next().unwrap();
    let direction = match direction {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction"),
    };
    let count = count.parse().unwrap();
    Intruction { direction, count }
}
fn parse_instructions(data: &str) -> Vec<Intruction> {
    data.lines().map(|line| parse_instruction(line)).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Rope {
    head: Point,
    tail: Point,
}

fn move_point(p: &mut Point, dir: &Direction) {
    match dir {
        Direction::Up => p.y += 1,
        Direction::Down => p.y -= 1,
        Direction::Left => p.x -= 1,
        Direction::Right => p.x += 1,
    }
}

fn make_point_follow_point(head: &Point, tail: &mut Point) {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    if x_diff.abs() >= 2 && y_diff.abs() >= 2 {
        tail.x = head.x - x_diff.signum();
        tail.y = head.y - y_diff.signum();
    } else if x_diff.abs() >= 2 {
        tail.x = head.x - x_diff.signum();
        tail.y = head.y;
    } else if y_diff.abs() >= 2 {
        tail.x = head.x;
        tail.y = head.y - y_diff.signum();
    }
}

fn move_rope(rope: &mut Rope, dir: &Direction) {
    move_point(&mut rope.head, dir);
    make_point_follow_point(&rope.head, &mut rope.tail);
}

#[allow(dead_code)]
fn print_points_in_grid(points: &[Point], min_y: i32, max_y: i32, min_x: i32, max_x: i32) {
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let point = Point { x, y };
            match points.iter().position(|&p| p == point) {
                Some(point_index) => {
                    print!(
                        "{}",
                        if point_index == 0 {
                            "H".to_string()
                        } else {
                            point_index.to_string()
                        }
                    );
                }
                None => {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            parse_instructions(_TEST_INPUT),
            vec![
                Intruction {
                    direction: Direction::Right,
                    count: 4
                },
                Intruction {
                    direction: Direction::Up,
                    count: 4
                },
                Intruction {
                    direction: Direction::Left,
                    count: 3
                },
                Intruction {
                    direction: Direction::Down,
                    count: 1
                },
                Intruction {
                    direction: Direction::Right,
                    count: 4
                },
                Intruction {
                    direction: Direction::Down,
                    count: 1
                },
                Intruction {
                    direction: Direction::Left,
                    count: 5
                },
                Intruction {
                    direction: Direction::Right,
                    count: 2
                },
            ]
        );
    }

    #[test]
    fn test_move_rope_right() {
        let mut rope = Rope {
            head: Point { x: 2, y: 1 },
            tail: Point { x: 1, y: 1 },
        };
        move_rope(&mut rope, &Direction::Right);
        assert_eq!(
            rope,
            Rope {
                head: Point { x: 3, y: 1 },
                tail: Point { x: 2, y: 1 },
            }
        )
    }

    #[test]
    fn test_move_rope_down() {
        let mut rope = Rope {
            head: Point { x: 1, y: 2 },
            tail: Point { x: 1, y: 3 },
        };
        move_rope(&mut rope, &Direction::Down);

        assert_eq!(
            rope,
            Rope {
                head: Point { x: 1, y: 1 },
                tail: Point { x: 1, y: 2 },
            }
        )
    }

    #[test]
    fn test_move_rope_up_diag_tail() {
        let mut rope = Rope {
            head: Point { x: 3, y: 3 },
            tail: Point { x: 2, y: 2 },
        };
        move_rope(&mut rope, &Direction::Up);

        assert_eq!(
            rope,
            Rope {
                head: Point { x: 3, y: 4 },
                tail: Point { x: 3, y: 3 },
            }
        )
    }

    #[test]
    fn test_move_rope_up_diag_left() {
        let mut rope = Rope {
            head: Point { x: 4, y: 0 },
            tail: Point { x: 3, y: 0 },
        };
        move_rope(&mut rope, &Direction::Up);

        assert_eq!(
            rope,
            Rope {
                head: Point { x: 4, y: 1 },
                tail: Point { x: 3, y: 0 },
            }
        )
    }

    #[test]
    fn test_make_point_follow() {
        let mut point = Point { x: 1, y: 1 };
        make_point_follow_point(&Point { x: 2, y: 1 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });

        let mut point = Point { x: 1, y: 1 };
        make_point_follow_point(&Point { x: 1, y: 2 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });

        let mut point = Point { x: 1, y: 1 };
        make_point_follow_point(&Point { x: 0, y: 1 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });

        let mut point = Point { x: 1, y: 1 };
        make_point_follow_point(&Point { x: 1, y: 0 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });

        let mut point = Point { x: 1, y: 0 };
        make_point_follow_point(&Point { x: 2, y: 1 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 0 });

        let mut point = Point { x: 1, y: 0 };
        make_point_follow_point(&Point { x: 2, y: 2 }, &mut point);
        assert_eq!(point, Point { x: 2, y: 1 });

        let mut point = Point { x: 0, y: 0 };
        make_point_follow_point(&Point { x: 2, y: 1 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });

        let mut point = Point { x: 0, y: 0 };
        make_point_follow_point(&Point { x: 2, y: 2 }, &mut point);
        assert_eq!(point, Point { x: 1, y: 1 });
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(_TEST_INPUT), 13);
    }

    static _TEST_INPUT_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_part2() {
        assert_eq!(part2(_TEST_INPUT), 1);
        assert_eq!(part2(_TEST_INPUT_2), 36);
    }
}
