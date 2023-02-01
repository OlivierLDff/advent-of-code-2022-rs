use anyhow::{self, Context};
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input, 2000000));
    println!("Part 2: {}", part2(input, 4000000).unwrap());
}

fn part1(input: &str, y: i64) -> usize {
    let sensors = parse_input(input).unwrap();
    let mut intersections = get_possible_intersections(&sensors, y);
    let merged_intersections = get_merged_intersections(&mut intersections);
    get_intersection_sum(&merged_intersections) as usize - get_beacons_count_with_y(y, &sensors)
}

fn part2(input: &str, max: i64) -> Option<usize> {
    let sensors = parse_input(input).unwrap();
    for y in 0..max {
        let mut intersections = get_possible_intersections(&sensors, y);
        let merged_intersections = get_merged_intersections(&mut intersections);

        for (_, x2) in &merged_intersections {
            if *x2 >= 0 && *x2 <= max {
                let x = x2 + 1;
                return Some(x as usize * 4000000 + y as usize);
            }
        }
    }
    None
}

fn get_beacons_count_with_y(y: i64, sensors: &[Sensor]) -> usize {
    sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.y == y)
        .map(|sensor| sensor.closest_beacon.y)
        .unique()
        .count()
}

// Get the possible intersections between the sensors and the line y = y
fn get_possible_intersections(sensors: &[Sensor], y: i64) -> Vec<(i64, i64)> {
    sensors
        .iter()
        .filter_map(|sensor| {
            let distance_to_closest_beacon = sensor.sensor_to_beacon_distance();
            get_intersection_point_between_circle_and_line(
                y,
                &sensor.point,
                distance_to_closest_beacon,
            )
        })
        .collect()
}

/// Merge the intersections to get the total area of the intersections
/// For example, if we have the following intersections:
/// [(1, 3), (2, 4), (5, 6), (7, 8)]
/// We want to merge them to get:
/// [(1, 8)]
///
/// # Return
///
/// The merged intersections
/// The intersections are sorted by the x coordinate of the first point
/// The first point of the intersection is the leftmost point of the intersection
/// The second point of the intersection is the rightmost point of the intersection
fn get_merged_intersections(intersections: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    intersections.sort_by_key(|(x, _)| *x);
    let intersections: &[(i64, i64)] = intersections;
    let mut results = Vec::new();

    for (x1, x2) in intersections {
        if results.is_empty() {
            results.push((*x1, *x2));
        } else {
            let last_intersection = results.last_mut().unwrap();
            if last_intersection.1 + 1 == *x1 {
                last_intersection.1 = *x2;
            } else if *x1 > last_intersection.1 {
                results.push((*x1, *x2));
            } else if *x2 > last_intersection.1 {
                last_intersection.1 = *x2;
            }
        }
    }
    results
}

fn get_intersection_sum(intersections: &[(i64, i64)]) -> i64 {
    intersections.iter().map(|(x1, x2)| x2 - x1 + 1).sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    point: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn new(point: Point, closest_beacon: Point) -> Self {
        Sensor {
            point,
            closest_beacon,
        }
    }

    fn sensor_to_beacon_distance(&self) -> i64 {
        manhattan_distance(&self.point, &self.closest_beacon)
    }
}

fn parse_line(input: &str) -> anyhow::Result<Sensor> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    // Use regex to parse the input
    let re = Regex::new(r"Sensor at x=(?P<x>-?\d+), y=(?P<y>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();

    let caps = re.captures(input).context(format!(
        "Invalid line {}, that doesn't match regex {}",
        input, re
    ))?;
    let x = caps.name("x").unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.name("y").unwrap().as_str().parse::<i64>().unwrap();
    let bx = caps.name("bx").unwrap().as_str().parse::<i64>().unwrap();
    let by = caps.name("by").unwrap().as_str().parse::<i64>().unwrap();

    Ok(Sensor::new(Point { x, y }, Point { x: bx, y: by }))
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Sensor>> {
    input.lines().map(parse_line).collect()
}

/// In manhattan distance, a circle of `radius` and `origin` is given by the equation |x - origin.x| + |y - origin.y| = radius
/// We want to find the intersection between a line of equation y = line_y and the circle
///
/// # Return
///
/// The intersection point between the circle and the line if there is one as (x1, x2)
/// If there is no intersection, return None
fn get_intersection_point_between_circle_and_line(
    line_y: i64,
    origin: &Point,
    radius: i64,
) -> Option<(i64, i64)> {
    // |x - origin.x| = radius - |y - origin.y|
    // if radius - |y - origin.y| < 0, there is no intersection
    let x_abs = radius - (line_y - origin.y).abs();
    if x_abs < 0 {
        return None;
    }

    // (x - origin.x)^2 - (radius - |y - origin.y|)^2 = 0
    // x^2 - 2*origin.x*x + origin.x^2 - (radius - |y - origin.y|)^2 = 0
    // ax^2 + bx + c = 0

    let a = 1;
    let b = -2 * origin.x;
    let c = origin.x * origin.x - x_abs * x_abs;

    // delta = b^2 - 4ac
    let delta = b * b - 4 * a * c;

    if delta < 0 {
        return None;
    }

    let delta_sqrt = (delta as f64).sqrt() as i64;
    let x1 = (-b - delta_sqrt) / (2 * a);
    let x2 = (-b + delta_sqrt) / (2 * a);

    Some((x1, x2))
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    fn get_example_data() -> Vec<Sensor> {
        vec![
            Sensor::new(Point { x: 2, y: 18 }, Point { x: -2, y: 15 }),
            Sensor::new(Point { x: 9, y: 16 }, Point { x: 10, y: 16 }),
            Sensor::new(Point { x: 13, y: 2 }, Point { x: 15, y: 3 }),
            Sensor::new(Point { x: 12, y: 14 }, Point { x: 10, y: 16 }),
            Sensor::new(Point { x: 10, y: 20 }, Point { x: 10, y: 16 }),
            Sensor::new(Point { x: 14, y: 17 }, Point { x: 10, y: 16 }),
            Sensor::new(Point { x: 8, y: 7 }, Point { x: 2, y: 10 }),
            Sensor::new(Point { x: 2, y: 0 }, Point { x: 2, y: 10 }),
            Sensor::new(Point { x: 0, y: 11 }, Point { x: 2, y: 10 }),
            Sensor::new(Point { x: 20, y: 14 }, Point { x: 25, y: 17 }),
            Sensor::new(Point { x: 17, y: 20 }, Point { x: 21, y: 22 }),
            Sensor::new(Point { x: 16, y: 7 }, Point { x: 15, y: 3 }),
            Sensor::new(Point { x: 14, y: 3 }, Point { x: 15, y: 3 }),
            Sensor::new(Point { x: 20, y: 1 }, Point { x: 15, y: 3 }),
        ]
    }

    #[test]
    fn test_parse_line() {
        let sensor = parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").unwrap();
        assert_eq!(sensor.point, Point { x: 2, y: 18 });
        assert_eq!(sensor.closest_beacon, Point { x: -2, y: 15 });

        let sensor = parse_line("Sensor at x=9, y=16: closest beacon is at x=10, y=16").unwrap();
        assert_eq!(sensor.point, Point { x: 9, y: 16 });
        assert_eq!(sensor.closest_beacon, Point { x: 10, y: 16 });
    }

    #[test]
    fn test_wrong_parse_line() {
        let sensor = parse_line("Sensor at x=2 closest beacon is at x=-2, y=15");
        assert!(sensor.is_err());

        let sensor = parse_line("Sensor at x, y=18: closest beacon is at x=-2, y=15: extra");
        assert!(sensor.is_err());
    }

    #[test]
    fn test_parse_input() {
        let sensors = parse_input(EXAMPLE).unwrap();
        assert_eq!(sensors.len(), 14);
        assert_eq!(sensors, get_example_data());
    }

    #[test]
    fn test_get_intersection_point_between_circle_and_line_circle_simple() {
        let (x1, x2) =
            get_intersection_point_between_circle_and_line(0, &Point { x: 0, y: 0 }, 1).unwrap();
        assert_eq!(x1, -1);
        assert_eq!(x2, 1);
    }

    #[test]
    fn test_get_intersection_point_between_circle_and_line_circle_simple_r2() {
        let (x1, x2) =
            get_intersection_point_between_circle_and_line(0, &Point { x: 0, y: 0 }, 2).unwrap();
        assert_eq!(x1, -2);
        assert_eq!(x2, 2);
    }

    #[test]
    fn test_get_intersection_point_between_circle_and_line_circle_simple_no_intersection() {
        assert!(dbg!(get_intersection_point_between_circle_and_line(
            4,
            &Point { x: 0, y: 0 },
            3
        ))
        .is_none());
    }

    #[test]
    fn test_get_intersection_point_between_circle_and_line_circle_offset_origin() {
        let sensor = Point { x: 8, y: 7 };
        let beacon = Point { x: 2, y: 10 };
        let distance = manhattan_distance(&sensor, &beacon);
        let (x1, x2) =
            get_intersection_point_between_circle_and_line(10, &Point { x: 8, y: 7 }, distance)
                .unwrap();
        println!("x1: {}, x2: {}", x1, x2);
        assert_eq!(x1, 2);
        assert_eq!(x2, 14);
    }

    fn get_intersections() -> Vec<(i64, i64)> {
        vec![(12, 12), (2, 14), (2, 2), (-2, 2), (16, 24), (14, 18)]
    }

    #[test]
    fn test_get_possible_intersection_points() {
        let sensors = get_example_data();
        let intersections = get_possible_intersections(&sensors, 10);
        assert_eq!(intersections, get_intersections());
    }

    #[test]
    fn test_get_merged_intersections() {
        let mut intersections = get_intersections();
        let merged_intersections = get_merged_intersections(&mut intersections);
        assert_eq!(merged_intersections, vec![(-2, 24)]);
    }

    #[test]
    fn test_get_merged_intersections_2() {
        let merged_intersections = get_merged_intersections(&mut vec![
            (1, 2),
            (3, 4),
            (5, 6),
            (6, 6),
            (10, 13),
            (11, 34),
        ]);
        assert_eq!(merged_intersections, vec![(1, 6), (10, 34)]);
    }

    #[test]
    fn test_get_intersection_sum() {
        assert_eq!(get_intersection_sum(&vec![(-2, 24)]), 27);
        assert_eq!(get_intersection_sum(&vec![(1, 6), (10, 34)]), 6 + 25);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE, 20), Some(56000011));
    }
}
