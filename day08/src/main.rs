use ndarray::{arr2, Array2};

fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// 1. Parse string to get 2d matrix
// 2. For each row col not on edge compute visibility
fn part1(input: &str) -> usize {
    let forest = parse_forest_to_matrix(input);
    compute_visible_tree_count(&forest)
}

fn parse_forest_to_matrix(input: &str) -> ndarray::Array2<i32> {
    let nested_matrix = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as i32)
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    if nested_matrix.is_empty() {
        return Array2::zeros((0, 0));
    }

    let mut matrix = Array2::zeros((nested_matrix.len(), nested_matrix[0].len()));
    for (i, row) in nested_matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            matrix[[i, j]] = *col;
        }
    }

    matrix
}

fn is_tree_visible(forest: &ndarray::Array2<i32>, row: usize, col: usize) -> bool {
    let tree_height = forest[[row, col]];
    let forest_width = forest.shape()[1];
    let forest_height = forest.shape()[0];

    // Check is edge
    if row == 0 || row == forest_width - 1 || col == 0 || col == forest_height - 1 {
        return true;
    }

    // Check right direction
    for col_id in (col + 1)..forest_width {
        let height = forest[[row, col_id]];
        if height >= tree_height {
            break;
        }

        if col_id == forest_width - 1 {
            return true;
        }
    }

    // Check left direction
    for col_id in (0..col).rev() {
        let height = forest[[row, col_id]];
        if height >= tree_height {
            break;
        }

        if col_id == 0 {
            return true;
        }
    }

    // Check up direction
    for row_id in (0..row).rev() {
        let height = forest[[row_id, col]];
        if height >= tree_height {
            break;
        }

        if row_id == 0 {
            return true;
        }
    }

    // Check down direction
    for row_id in (row + 1)..forest_height {
        let height = forest[[row_id, col]];
        if height >= tree_height {
            break;
        }

        if row_id == forest_height - 1 {
            return true;
        }
    }

    false
}

fn compute_visible_tree_count(forest: &ndarray::Array2<i32>) -> usize {
    let mut count = 0;
    for row in 0..forest.shape()[0] {
        for col in 0..forest.shape()[1] {
            if is_tree_visible(forest, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn compute_scenic_score_of_tree(forest: &ndarray::Array2<i32>, row: usize, col: usize) -> usize {
    let tree_height = forest[[row, col]];
    let forest_width = forest.shape()[1];
    let forest_height = forest.shape()[0];

    // Check right direction
    let mut right_score = 0;
    for col_id in (col + 1)..forest_width {
        right_score += 1;
        let height = forest[[row, col_id]];
        if height >= tree_height {
            break;
        }
    }

    // Check left direction
    let mut left_score = 0;
    for col_id in (0..col).rev() {
        let height = forest[[row, col_id]];
        left_score += 1;
        if height >= tree_height {
            break;
        }
    }

    // Check up direction
    let mut up_score = 0;
    for row_id in (0..row).rev() {
        let height = forest[[row_id, col]];
        up_score += 1;
        if height >= tree_height {
            break;
        }
    }

    // Check down direction
    let mut down_score = 0;
    for row_id in (row + 1)..forest_height {
        let height = forest[[row_id, col]];
        down_score += 1;
        if height >= tree_height {
            break;
        }
    }

    left_score * right_score * up_score * down_score
}

fn find_best_scenic_score(forest: &ndarray::Array2<i32>) -> usize {
    let mut best_score = 0;
    for row in 0..forest.shape()[0] {
        for col in 0..forest.shape()[1] {
            let score = compute_scenic_score_of_tree(forest, row, col);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

fn part2(input: &str) -> usize {
    let forest = parse_forest_to_matrix(input);
    find_best_scenic_score(&forest)
}

#[cfg(test)]
mod tests {
    use super::*;

    static _EXAMPLE_INPUT: &str = r#"
30373
25512
65332
33549
35390"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(_EXAMPLE_INPUT), 21);
    }

    fn create_forest_example_data() -> Array2<i32> {
        arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ])
    }

    #[test]
    fn test_parse_forest_to_matrix() {
        assert_eq!(
            parse_forest_to_matrix(_EXAMPLE_INPUT),
            create_forest_example_data()
        );
    }

    #[test]
    fn test_is_tree_visible() {
        let forest = create_forest_example_data();
        assert!(is_tree_visible(&forest, 0, 0));
        assert!(is_tree_visible(&forest, 0, 1));

        // Top left
        assert!(is_tree_visible(&forest, 1, 1));
        // Top middle
        assert!(is_tree_visible(&forest, 1, 2));
        // Top right
        assert!(!is_tree_visible(&forest, 1, 3));

        // Left middle
        assert!(is_tree_visible(&forest, 2, 1));
        // Center 3
        assert!(!is_tree_visible(&forest, 2, 2));
        // Right middle
        assert!(is_tree_visible(&forest, 2, 3));

        // Bottom left
        assert!(!is_tree_visible(&forest, 3, 1));
        // Bottom middle
        assert!(is_tree_visible(&forest, 3, 2));
        // Bottom right
        assert!(!is_tree_visible(&forest, 3, 3));
    }

    #[test]
    fn test_compute_visible_tree_count() {
        let forest = create_forest_example_data();
        assert_eq!(compute_visible_tree_count(&forest), 21);
    }

    #[test]
    fn test_get_scenic_score() {
        let forest = create_forest_example_data();
        assert_eq!(compute_scenic_score_of_tree(&forest, 1, 2), 4);
        assert_eq!(compute_scenic_score_of_tree(&forest, 3, 2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(_EXAMPLE_INPUT), 8);
    }
}
