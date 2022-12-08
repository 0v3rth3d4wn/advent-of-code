use grid::*;
use std::fs;

fn main() {
    let data = "data.txt";
    let file = fs::read_to_string(data).unwrap();
    let tree_lines: Vec<&str> = file.trim_end().split("\n").collect();
    let mut tree_grid: Grid<u8> = grid![];
    let mut tree_scores: Vec<u32> = vec![];
    let mut sum = 0;

    for line in &tree_lines {
        tree_grid.push_row(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );
    }
    // println!("tree_grid: {:#?}", tree_grid);

    for row in 1..tree_lines.len() - 1 {
        for col in 1..tree_lines.get(row).unwrap().len() - 1 {
            let tree = tree_grid.get(row, col).unwrap();

            let mut top: bool = false;
            for upper_row in 0..=row - 1 {
                if tree <= tree_grid.get(upper_row, col).unwrap() {
                    top = false;
                    break;
                }

                top = true;
            }

            let mut bottom: bool = false;
            for bottom_row in row + 1..tree_grid.cols() {
                if tree <= tree_grid.get(bottom_row, col).unwrap() {
                    bottom = false;
                    break;
                }

                bottom = true;
            }

            let mut left: bool = false;
            for left_col in 0..=col - 1 {
                if tree <= tree_grid.get(row, left_col).unwrap() {
                    left = false;
                    break;
                }

                left = true;
            }

            let mut right: bool = false;
            for right_col in col + 1..tree_grid.rows() {
                if tree <= tree_grid.get(row, right_col).unwrap() {
                    right = false;
                    break;
                }

                right = true;
            }

            if left || right || top || bottom {
                sum += 1;
            }
        }
    }

    sum += tree_lines.first().unwrap().len() * 2 + tree_lines.last().unwrap().len() * 2 - 4;
    println!("Part 1 sum: {}", sum);

    for row in 1..tree_lines.len() - 1 {
        for col in 1..tree_lines.get(row).unwrap().len() - 1 {
            let tree = tree_grid.get(row, col).unwrap();

            let mut top: u32 = 0;
            let mut top_distance: Vec<u8> = vec![];
            for upper_row in 0..=row - 1 {
                top_distance.push(*tree_grid.get(upper_row, col).unwrap());
            }
            top_distance.reverse();
            for comp in &top_distance {
                top += 1;
                if comp >= tree {
                    break;
                }
            }

            let mut bottom: u32 = 0;
            let mut bottom_distance: Vec<u8> = vec![];
            for bottom_row in row + 1..tree_grid.cols() {
                bottom_distance.push(*tree_grid.get(bottom_row, col).unwrap());
            }
            for comp in &bottom_distance {
                bottom += 1;
                if comp >= tree {
                    break;
                }
            }

            let mut left: u32 = 0;
            let mut left_distance: Vec<u8> = vec![];
            for left_col in 0..=col - 1 {
                left_distance.push(*tree_grid.get(row, left_col).unwrap());
            }
            left_distance.reverse();
            for comp in &left_distance {
                left += 1;
                if comp >= tree {
                    break;
                }
            }

            let mut right: u32 = 0;
            let mut right_distance: Vec<u8> = vec![];
            for right_col in col + 1..tree_grid.rows() {
                right_distance.push(*tree_grid.get(row, right_col).unwrap());
            }
            for comp in &right_distance {
                right += 1;
                if comp >= tree {
                    break;
                }
            }

            tree_scores.push(top * bottom * left * right);
        }
    }

    tree_scores.sort();
    println!("Part 2: {:?}", tree_scores.last());
}
