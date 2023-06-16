use std::collections::HashSet;

use anyhow::Result;

fn is_visible(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    if col == 0 || row == 0 || row == grid.len() - 1 || col == grid[row].len() - 1 {
        return true;
    }
    let cell_value = grid[row][col];

    for check_col in 0..col {
        if grid[row][check_col] >= cell_value {
            break;
        }

        if check_col == col - 1 {
            return true;
        }
    }

    for check_col in col + 1..grid[row].len() {
        if grid[row][check_col] >= cell_value {
            break;
        }

        if check_col == grid[row].len() - 1 {
            return true;
        }
    }

    for check_row in 0..row {
        if grid[check_row][col] >= cell_value {
            break;
        }

        if check_row == row - 1 {
            return true;
        }
    }

    for check_row in row + 1..grid.len() {
        if grid[check_row][col] >= cell_value {
            break;
        }

        if check_row == grid.len() - 1 {
            return true;
        }
    }
    return false;
}

fn get_scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    if col == 0 || row == 0 || row == grid.len() - 1 || col == grid[row].len() - 1 {
        return 0;
    }
    let mut left = 0;
    let mut right = 0;
    let mut down = 0;
    let mut up = 0;

    let cell_value = grid[row][col];

    for check_col in (0..col).rev() {
        left += 1;
        if grid[row][check_col] >= cell_value {
            break;
        }
    }

    for check_col in col + 1..grid[row].len() {
        right += 1;
        if grid[row][check_col] >= cell_value {
            break;
        }
    }

    for check_row in (0..row).rev() {
        up += 1;
        if grid[check_row][col] >= cell_value {
            break;
        }
    }

    for check_row in row + 1..grid.len() {
        down += 1;
        if grid[check_row][col] >= cell_value {
            break;
        }
    }

    return left * right * up * down;
}

fn main() -> Result<()> {
    let grid = include_str!("./day8.prod")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut set = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if is_visible(&grid, row, col) {
                set.insert((row, col));
            }
        }
    }
    println!("Part 1: {}", set.len());

    let max_scenic = grid.iter().enumerate().fold(0, |acc, (row_idx, row)| {
        let score = row.iter().enumerate().fold(0, |acc, (col_idx, _col)| {
            let score = get_scenic_score(&grid, row_idx, col_idx);
            if score > acc {
                return score;
            }
            return acc;
        });
        if score > acc {
            return score;
        }
        return acc;
    });
    println!("Part 2: {}", max_scenic);
    return Ok(());
}
