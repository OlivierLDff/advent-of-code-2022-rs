use anyhow::{self, Context};
use ndarray::Array2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 : {}", part1(input));
    println!("Part 2 : {}", part2(input));
}

fn part1(data: &str) -> i32 {
    let lines = parse_input(data).unwrap();
    let mut cave = create_cave_with_rocks(&lines).unwrap();

    // Count the number of time the sand is flowing
    let mut stopped_count = 0;

    // The sand starts at 500,0
    let sand_point = Point { x: 500, y: 0 };

    loop {
        match simulate_sand(&sand_point, &mut cave) {
            SandSimulationResult::Stopped => {
                stopped_count += 1;
            }
            _ => {
                break;
            }
        }
    }
    cave.dump_cave(Some(sand_point));
    stopped_count
}

fn part2(data: &str) -> i32 {
    let lines = parse_input(data).unwrap();
    let mut cave = create_cave_with_rocks(&lines).unwrap();
    cave.add_floor();

    // Count the number of time the sand is flowing
    let mut stopped_count = 0;

    // The sand starts at 500,0
    let sand_point = Point { x: 500, y: 0 };

    loop {
        match simulate_sand(&sand_point, &mut cave) {
            SandSimulationResult::Stopped => {
                stopped_count += 1;
            }
            SandSimulationResult::Flowing(point) => {
                cave.resize_to_fit_point(&point);
                cave.draw_floor();
            }
            SandSimulationResult::Overflow => {
                break;
            }
        }
    }
    cave.dump_cave(Some(sand_point));
    stopped_count
}

// A cave is a 2D array of CaveMaterial
// CaveMaterial can be Air, Rock or Sand
// Air is the default value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaveMaterial {
    Air,
    Rock,
    Sand,
}

// Steps
// Parse input
// Find the min and max x and y
// and create a 2D array representing the grid
// Draw rocks line
// Simulate the sand falling from 500,0
// Have a stopping condition to check if the sand is outside the grid

// A point in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

// Parse the input
// Each line is a list of points
// Each point is a pair of x and y
// The points are connected by a line
// The lines are connected by a ->
// The lines are separated by a newline
//
// # Arguments
//
// * `data` - The input data
//
// # Returns
//
// A vector of vector of points
fn parse_input(data: &str) -> anyhow::Result<Vec<Vec<Point>>> {
    data.lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").context("no comma")?;
                    Ok(Point {
                        x: x.parse()?,
                        y: y.parse()?,
                    })
                })
                .collect()
        })
        .collect()
}

// Find the min and max x and y
// of all the points in the lines
//
// # Arguments
//
// * `lines` - The lines
//
// # Returns
//
// A tuple of the min and max points
fn find_min_max_point(lines: &[Vec<Point>]) -> Option<(Point, Point)> {
    let mut min = None;
    let mut max = None;
    for line in lines {
        for point in line {
            if min.is_none() || max.is_none() {
                min = Some(*point);
                max = Some(*point);
                continue;
            }

            let min = min.as_mut().unwrap();
            let max = max.as_mut().unwrap();

            if point.x < min.x {
                min.x = point.x;
            }
            if point.y < min.y {
                min.y = point.y;
            }
            if point.x > max.x {
                max.x = point.x;
            }
            if point.y > max.y {
                max.y = point.y;
            }
        }
    }

    if min.is_none() || max.is_none() {
        None
    } else {
        Some((min.unwrap(), max.unwrap()))
    }
}

struct Cave {
    grid: Array2<CaveMaterial>,
    // The min and max points of the cave
    // The min point is the top left corner
    // The max point is the bottom right corner
    // They are used to transform absolute points to relative points which can be used as grid index
    min: Point,
    max: Point,
}

impl Cave {
    fn new(min: &Point, max: &Point) -> Cave {
        let min = Point { x: min.x, y: 0 };
        let width = max.x - min.x + 1;
        let height = max.y - min.y + 1;

        Cave {
            grid: Array2::from_elem((width as usize, height as usize), CaveMaterial::Air),
            min,
            max: *max,
        }
    }

    fn resize(&mut self, min: &Point, max: &Point) {
        let mut new_cave = Cave::new(min, max);

        let min = Point {
            x: min.x.max(self.min.x),
            y: min.y.max(self.min.y),
        };
        let max = Point {
            x: max.x.min(self.max.x),
            y: max.y.min(self.max.y),
        };

        // Copy the cave to the new cave
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let point = Point { x, y };
                let material = self.get_from_absolute_point(&point).unwrap();
                *new_cave.get_from_absolute_point_mut(&point).unwrap() = *material;
            }
        }
        *self = new_cave
    }

    fn resize_to_fit_point(&mut self, point: &Point) {
        let mut min = self.min;
        let mut max = self.max;

        if point.x < min.x {
            min.x = point.x;
        }
        if point.y < min.y {
            min.y = point.y;
        }
        if point.x > max.x {
            max.x = point.x;
        }
        if point.y > max.y {
            max.y = point.y;
        }

        self.resize(&min, &max);
    }

    fn add_floor(&mut self) {
        self.resize(
            &self.min.clone(),
            &Point {
                x: self.max.x,
                y: self.max.y + 2,
            },
        );

        self.draw_floor();
    }

    fn draw_floor(&mut self) {
        self.draw_rock_line(
            &Point {
                x: self.min.x,
                y: self.max.y,
            },
            &Point {
                x: self.max.x,
                y: self.max.y,
            },
        )
        .unwrap();
    }

    fn get_from_absolute_point_mut(&mut self, point: &Point) -> Option<&mut CaveMaterial> {
        if point.x < self.min.x
            || point.x > self.max.x
            || point.y < self.min.y
            || point.y > self.max.y
        {
            None
        } else {
            Some(
                &mut self.grid[[
                    (point.x - self.min.x) as usize,
                    (point.y - self.min.y) as usize,
                ]],
            )
        }
    }

    fn get_from_absolute_point(&self, point: &Point) -> Option<&CaveMaterial> {
        if point.x < self.min.x
            || point.x > self.max.x
            || point.y < self.min.y
            || point.y > self.max.y
        {
            None
        } else {
            Some(
                &self.grid[[
                    (point.x - self.min.x) as usize,
                    (point.y - self.min.y) as usize,
                ]],
            )
        }
    }

    fn draw_rock_line(&mut self, p1: &Point, p2: &Point) -> anyhow::Result<()> {
        if p1.x == p2.x {
            // Vertical line
            let (y1, y2) = (p1.y.min(p2.y), p1.y.max(p2.y));
            for y in y1..=y2 {
                *self
                    .get_from_absolute_point_mut(&Point { x: p1.x, y })
                    .unwrap() = CaveMaterial::Rock;
            }
            Ok(())
        } else if p1.y == p2.y {
            // Horizontal line
            let (x1, x2) = (p1.x.min(p2.x), p1.x.max(p2.x));
            for x in x1..=x2 {
                *self
                    .get_from_absolute_point_mut(&Point { x, y: p1.y })
                    .unwrap() = CaveMaterial::Rock;
            }
            Ok(())
        } else {
            anyhow::bail!("Invalid line: {:?} -> {:?}, x or y doesn't match", p1, p2);
        }
    }

    fn dump_cave(&self, start: Option<Point>) {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                if let Some(start) = start {
                    if start.x == x && start.y == y {
                        print!("+");
                        continue;
                    }
                }
                let material = self.get_from_absolute_point(&Point { x, y }).unwrap();
                match material {
                    CaveMaterial::Air => print!("."),
                    CaveMaterial::Rock => print!("#"),
                    CaveMaterial::Sand => print!("o"),
                }
            }
            println!();
        }
    }
}

// Create a grid that will contain the cave
// with all the CaveMaterial set to Air
// The grid is the size of the min and max points on x
// and the max point on y.
//
// Then draw the rocks on the grid using the lines, the rocks are
// represented by CaveMaterial::Rock
//
// # Arguments
//
// * `lines` - The lines
//
// # Returns
//
// A 2D array of CaveMaterial
// Err if the min and max points cannot be found (no lines or not points in lines)
// Err if the lines are invalid (ie not horizontal or vertical)
fn create_cave_with_rocks(lines: &[Vec<Point>]) -> anyhow::Result<Cave> {
    let (min, max) =
        find_min_max_point(lines).context("Fail to find min/max point, due to empty lines")?;

    let mut cave = Cave::new(&min, &max);

    for line in lines {
        // line is a list of 2 points, so we want to use windows(2)
        for points in line.windows(2) {
            let (p1, p2) = (points[0], points[1]);
            cave.draw_rock_line(&p1, &p2)?;
        }
    }
    Ok(cave)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SandSimulationResult {
    // The sand stopped because it hit a rock or got block
    Stopped,
    // The sand got outside the grid
    // This can happen if the grid is too small
    // Point hold the last point of the sand where it flowed
    Flowing(Point),
    // The start point if already with stand
    Overflow,
}

// Simulate the sand falling from the start point
// The sand will fall down until it hits a rock or the bottom of the grid.
// If the sand hit a rock it will spread to the left and right if possible.
// If the sand hit other sand it will spread to the bottom left if possible then bottom right
// or the edge of the grid.
//
// # Arguments
//
// * `start` - The start point
// * `cave` - The cave grid
//
// # Returns
//
// SandSimulationResult::Stopped if the sand stopped because it hit a rock
// SandSimulationResult::Outside if the sand stopped because it reached the bottom of the grid
fn simulate_sand(start: &Point, cave: &mut Cave) -> SandSimulationResult {
    match cave.get_from_absolute_point(start) {
        Some(CaveMaterial::Rock | CaveMaterial::Sand) => return SandSimulationResult::Overflow,
        _ => (),
    }
    let mut current = *start;

    loop {
        let bottom = Point {
            x: current.x,
            y: current.y + 1,
        };
        match cave.get_from_absolute_point(&bottom) {
            None => {
                // We reached the bottom of the grid or we are outside the grid
                return SandSimulationResult::Flowing(bottom);
            }
            Some(CaveMaterial::Air) => {
                // We can fall down
                current = bottom;
            }
            Some(CaveMaterial::Sand | CaveMaterial::Rock) => {
                // We reached the bottom of the sand, left try if we can go left or right
                let bottom_left = Point {
                    x: current.x - 1,
                    y: current.y + 1,
                };
                let bottom_right = Point {
                    x: current.x + 1,
                    y: current.y + 1,
                };

                if !(match cave.get_from_absolute_point(&bottom_left) {
                    None => return SandSimulationResult::Flowing(bottom_left),
                    Some(CaveMaterial::Air) => {
                        // We can flow on the left
                        current = bottom_left;
                        true
                    }
                    Some(_) => {
                        // We can't flow on the left
                        false
                    }
                } || match cave.get_from_absolute_point(&bottom_right) {
                    None => return SandSimulationResult::Flowing(bottom_right),
                    Some(CaveMaterial::Air) => {
                        // We can flow on the right
                        current = bottom_right;
                        true
                    }
                    Some(_) => {
                        // We can't flow on the right
                        false
                    }
                }) {
                    // We can't flow left or right, we are stuck
                    *cave.get_from_absolute_point_mut(&current).unwrap() = CaveMaterial::Sand;
                    return SandSimulationResult::Stopped;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    static EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    fn get_example_data() -> Vec<Vec<Point>> {
        vec![
            vec![
                Point { x: 498, y: 4 },
                Point { x: 498, y: 6 },
                Point { x: 496, y: 6 },
            ],
            vec![
                Point { x: 503, y: 4 },
                Point { x: 502, y: 4 },
                Point { x: 502, y: 9 },
                Point { x: 494, y: 9 },
            ],
        ]
    }

    #[test]
    fn test_parse_input() {
        let lines = parse_input(EXAMPLE).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines, get_example_data());
    }

    #[test]
    fn test_parse_input_malformatted() {
        assert!(parse_input("498,4 -> 498").is_err());
        assert!(parse_input("498,4 -> 498,6 -> 496,6 -> 496").is_err());
        assert!(parse_input("498,4 -> 498,6 -> 496,6 -> 496,6a").is_err());
        assert!(parse_input("498").is_err());
        assert!(parse_input("498,4 -> 498,6 > 12,3").is_err());
        assert!(parse_input("aze,a").is_err());
        assert!(parse_input("aze,4 -> 498,6 -> 496,6").is_err());
        assert!(parse_input("13,a").is_err());
    }

    #[test]
    fn test_parse_input_fuzz() {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let mut s = String::new();
            for _ in 0..rng.gen_range(1..10) {
                s.push_str(&format!(
                    "{},{} -> ",
                    rng.gen_range(0..1000),
                    rng.gen_range(0..1000)
                ));
            }
            s.push_str(&format!(
                "{},{}",
                rng.gen_range(0..1000),
                rng.gen_range(0..1000)
            ));
            parse_input(&s).unwrap();
        }
    }

    #[test]
    fn test_get_min_max() {
        let lines = get_example_data();
        let (min, max) = find_min_max_point(&lines).unwrap();

        assert_eq!(min, Point { x: 494, y: 4 });
        assert_eq!(max, Point { x: 503, y: 9 });
    }

    #[test]
    fn test_empty_create_cave() {
        assert!(create_cave_with_rocks(&vec![]).is_err());
    }

    #[test]
    fn test_cave_out_of_range() {
        let lines = get_example_data();
        let cave = create_cave_with_rocks(&lines).unwrap();
        assert_eq!(cave.grid.shape(), &[10, 10]);

        assert_eq!(cave.min, Point { x: 494, y: 0 });
        assert_eq!(cave.max, Point { x: 503, y: 9 });

        assert_eq!(cave.get_from_absolute_point(&Point { x: 0, y: 0 }), None);
        assert_eq!(cave.get_from_absolute_point(&Point { x: 10, y: 10 }), None);
        assert_eq!(cave.get_from_absolute_point(&Point { x: 493, y: 0 }), None);
        assert_eq!(cave.get_from_absolute_point(&Point { x: 504, y: 9 }), None);
        assert_eq!(cave.get_from_absolute_point(&Point { x: 494, y: 10 }), None);
        assert_eq!(cave.get_from_absolute_point(&Point { x: 503, y: 10 }), None);
    }

    #[test]
    fn test_create_cave_with_rocks() {
        let lines = get_example_data();
        let Cave {
            grid,
            min: _,
            max: _,
        } = create_cave_with_rocks(&lines).unwrap();

        assert_eq!(grid.shape(), &[10, 10]);

        // Wall 0
        assert_eq!(grid[[4, 4]], CaveMaterial::Rock);
        assert_eq!(grid[[4, 5]], CaveMaterial::Rock);
        assert_eq!(grid[[4, 6]], CaveMaterial::Rock);
        assert_eq!(grid[[3, 6]], CaveMaterial::Rock);
        assert_eq!(grid[[2, 6]], CaveMaterial::Rock);

        // Wall 1
        assert_eq!(grid[[9, 4]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 4]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 5]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 6]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 7]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 8]], CaveMaterial::Rock);
        assert_eq!(grid[[8, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[7, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[6, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[5, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[4, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[3, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[2, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[1, 9]], CaveMaterial::Rock);
        assert_eq!(grid[[0, 9]], CaveMaterial::Rock);

        // Air
        assert_eq!(grid[[0, 0]], CaveMaterial::Air);
        assert_eq!(grid[[0, 1]], CaveMaterial::Air);
    }

    #[test]
    fn test_simulate_sand() {
        let mut cave = create_cave_with_rocks(&get_example_data()).unwrap();

        // Step 1
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Stopped
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 500, y: 8 })
                .unwrap(),
            CaveMaterial::Sand
        );

        // Step 2
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Stopped
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 499, y: 8 })
                .unwrap(),
            CaveMaterial::Sand
        );

        // Step 3
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Stopped
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 501, y: 8 })
                .unwrap(),
            CaveMaterial::Sand
        );

        // Step 4
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Stopped
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 500, y: 7 })
                .unwrap(),
            CaveMaterial::Sand
        );

        // Step 5
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Stopped
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 498, y: 8 })
                .unwrap(),
            CaveMaterial::Sand
        );

        // Step 6-22
        for _ in 0..17 {
            assert_eq!(
                simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
                SandSimulationResult::Stopped
            );
        }

        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 500, y: 1 })
                .unwrap(),
            CaveMaterial::Air
        );
        assert_eq!(
            *cave
                .get_from_absolute_point(&Point { x: 500, y: 2 })
                .unwrap(),
            CaveMaterial::Sand
        );
        for i in 0..=2 {
            assert_eq!(
                *cave
                    .get_from_absolute_point(&Point { x: 499 + i, y: 3 })
                    .unwrap(),
                CaveMaterial::Sand
            );
        }

        // Step 23 and 24 should be stopped
        for _ in 0..2 {
            assert_eq!(
                simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
                SandSimulationResult::Stopped
            );
        }

        // Step 25 should be flowing
        assert_eq!(
            simulate_sand(&Point { x: 500, y: 0 }, &mut cave),
            SandSimulationResult::Flowing(Point { x: 493, y: 9 })
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 24);
    }

    #[test]
    fn test_add_floor() {
        let mut cave = create_cave_with_rocks(&get_example_data()).unwrap();
        cave.add_floor();
        cave.dump_cave(None);

        assert_eq!(cave.grid.shape(), &[10, 12]);

        for i in 0..10 {
            assert_eq!(
                *cave
                    .get_from_absolute_point(&Point { x: 494 + i, y: 11 })
                    .unwrap(),
                CaveMaterial::Rock
            );
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 93);
    }
}
