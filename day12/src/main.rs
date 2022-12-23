use std::collections::HashMap;

use ndarray::Array2;
use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let height_map = parse_input_to_height_map(input);
    climb(&height_map.map, &height_map.start, &height_map.end).unwrap() - 1
}

fn part2(input: &str) -> i32 {
    let height_map = parse_input_to_height_map(input);
    let mut possible_start_points = Vec::new();
    for y in 0..height_map.map.shape()[0] {
        for x in 0..height_map.map.shape()[1] {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if height_map.map[[y, x]] == 0
                && (x == 0
                    || x == height_map.map.shape()[1] - 1
                    || y == 0
                    || y == height_map.map.shape()[0] - 1)
            {
                possible_start_points.push(point);
            }
        }
    }

    possible_start_points
        .iter()
        .filter_map(|start| climb(&height_map.map, &start, &height_map.end))
        .min()
        .unwrap()
        - 1
}

fn convert_char_to_height(c: char) -> i32 {
    if c == 'S' {
        return convert_char_to_height('a');
    }
    if c == 'E' {
        return convert_char_to_height('z');
    }
    c as i32 - 'a' as i32
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

type HeightMap = Array2<i32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HeightMapInput {
    map: HeightMap,
    start: Point,
    end: Point,
}

fn parse_input_to_height_map(input: &str) -> HeightMapInput {
    let mut height_map = Array2::zeros((
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
    ));
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            height_map[[y, x]] = convert_char_to_height(c);

            if c == 'S' {
                start = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
            } else if c == 'E' {
                end = Some(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    HeightMapInput {
        map: height_map,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn is_valid_point(height_map: &HeightMap, point: &Point) -> bool {
    point.x >= 0
        && point.x < height_map.shape()[1] as i32
        && point.y >= 0
        && point.y < height_map.shape()[0] as i32
}

fn is_elevation_ok(height_map: &HeightMap, from: &Point, to: &Point) -> bool {
    let from_height = height_map[[from.y as usize, from.x as usize]];
    let to_height = height_map[[to.y as usize, to.x as usize]];
    to_height - from_height <= 1
}

// See dijkstra's algorithm
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// priority_queue is a max-heap priority queue, so min distance is the max value
fn climb(height_map: &HeightMap, from: &Point, to: &Point) -> Option<i32> {
    let mut dist = HashMap::<Point, i32>::new();
    let mut prev = HashMap::<Point, Option<Point>>::new();
    dist.insert(from.clone(), std::i32::MAX);
    let mut pq = PriorityQueue::<Point, i32>::new();

    for y in 0..height_map.shape()[0] {
        for x in 0..height_map.shape()[1] {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if point != *from {
                dist.insert(point.clone(), 0);
            }

            prev.insert(point.clone(), None);
            pq.push(point.clone(), dist[&point]);
        }
    }

    while let Some((current, _)) = pq.pop() {
        let neighbors = vec![
            Point {
                x: current.x - 1,
                y: current.y,
            },
            Point {
                x: current.x + 1,
                y: current.y,
            },
            Point {
                x: current.x,
                y: current.y - 1,
            },
            Point {
                x: current.x,
                y: current.y + 1,
            },
        ];
        let neighbors = neighbors.iter().filter(|&neighbor| {
            is_valid_point(height_map, neighbor) && is_elevation_ok(height_map, &current, neighbor)
        });

        for neighbor in neighbors {
            let alt = dist[&current] - 1;
            if alt > dist[neighbor] {
                dist.insert(neighbor.clone(), alt);
                prev.insert(neighbor.clone(), Some(current.clone()));
                pq.change_priority(neighbor, alt);
            }

            if neighbor == to {
                pq.clear();
                break;
            }
        }
    }

    let mut path = Vec::new();
    let mut u = Some(to.clone());
    while let Some(point) = u {
        path.push(point.clone());
        u = prev[&point].clone();
    }
    if let Some(point) = path.last() {
        if point != from {
            return None;
        }
    }
    Some(path.len() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_char_to_height() {
        assert_eq!(convert_char_to_height('S'), 0);
        assert_eq!(convert_char_to_height('E'), 25);
        assert_eq!(convert_char_to_height('a'), 0);
        assert_eq!(convert_char_to_height('z'), 25);
        assert_eq!(convert_char_to_height('b'), 1);
    }

    #[test]
    fn test_parse_input_to_height_map() {
        let input = "aSbc
                     defg
                     hijE
                     lmnp";
        let height_map = parse_input_to_height_map(input);
        assert_eq!(
            height_map.map,
            Array2::from_shape_vec(
                (4, 4),
                vec![0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 25, 11, 12, 13, 15]
            )
            .unwrap()
        );
        assert_eq!(height_map.start, Point { x: 1, y: 0 });
        assert_eq!(height_map.end, Point { x: 3, y: 2 });
    }

    #[test]
    fn test_is_point_valid() {
        assert!(is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: 0, y: 0 }
        ));
        assert!(is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: 1, y: 1 }
        ));
        assert!(!is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: -1, y: 0 }
        ));
        assert!(!is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: 0, y: -1 }
        ));
        assert!(!is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: 3, y: 0 }
        ));
        assert!(!is_valid_point(
            &Array2::zeros((3, 3)),
            &Point { x: 0, y: 3 }
        ));
    }

    #[test]
    fn test_is_elevation_ok() {
        let map = Array2::from_shape_vec((3, 3), vec![0, 0, 1, 2, 3, 4, 5, 6, 7]).unwrap();
        assert!(is_elevation_ok(
            &map,
            &Point { x: 0, y: 0 },
            &Point { x: 1, y: 0 }
        ));
        assert!(!is_elevation_ok(
            &map,
            &Point { x: 0, y: 0 },
            &Point { x: 0, y: 1 }
        ));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 31);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 29);
    }
}
