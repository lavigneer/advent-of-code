use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    row: usize,
    col: usize,
    ch: char,
}

fn part_one() -> usize {
    let grid = include_str!("./day11.prod")
        .lines()
        .flat_map(|l| {
            let res = l.chars().collect::<Vec<char>>();
            if l.chars().all(|c| c == '.') {
                vec![res.clone(), res]
            } else {
                vec![res]
            }
        })
        .collect::<Vec<Vec<char>>>();
    let mut empty_columns_set = HashSet::<usize>::from_iter(0..grid[0].len());
    for row in &grid {
        for i in 0..row.len() {
            if row[i] == '#' {
                empty_columns_set.remove(&i);
            }
        }
    }
    let grid = grid
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .flat_map(|(col, ch)| {
                    if empty_columns_set.contains(&col) {
                        return vec![ch, ch];
                    }
                    return vec![ch];
                })
                .enumerate()
                .map(move |(col, ch)| Point { row, col, ch })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let galaxies = grid
        .iter()
        .flat_map(|l| l.iter().filter(|p| p.ch == '#'))
        .collect::<Vec<&Point>>();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        let current_galaxy = galaxies[i];
        for j in i + 1..galaxies.len() {
            let compare_galaxy = galaxies[j];
            sum += current_galaxy.row.abs_diff(compare_galaxy.row);
            sum += current_galaxy.col.abs_diff(compare_galaxy.col);
        }
    }

    sum
}

fn part_two() -> usize {
    let grid = include_str!("./day11.prod")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut empty_columns_set = HashSet::<usize>::from_iter(0..grid[0].len());
    let mut empty_rows_set = HashSet::<usize>::from_iter(0..grid[0].len());
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                empty_columns_set.remove(&col);
            }
        }
        if grid[row].iter().any(|c| c == &'#') {
            empty_rows_set.remove(&row);
        }
    }

    let grid = grid
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.into_iter()
                .enumerate()
                .map(move |(col, ch)| Point { row, col, ch })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let galaxies = grid
        .iter()
        .flat_map(|l| l.iter().filter(|p| p.ch == '#'))
        .collect::<Vec<&Point>>();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        let current_galaxy = galaxies[i];
        for j in i + 1..galaxies.len() {
            let compare_galaxy = galaxies[j];
            let min_row = current_galaxy.row.min(compare_galaxy.row);
            let mut max_row = current_galaxy.row.max(compare_galaxy.row);
            let fill_rows = empty_rows_set
                .iter()
                .filter(|r| r > &&min_row && r < &&max_row)
                .count();

            max_row += fill_rows * 999999;

            let min_col = current_galaxy.col.min(compare_galaxy.col);
            let mut max_col = current_galaxy.col.max(compare_galaxy.col);
            let fill_cols = empty_columns_set
                .iter()
                .filter(|c| c > &&min_col && c < &&max_col)
                .count();
            max_col += fill_cols * 999999;

            sum += max_row - min_row;
            sum += max_col - min_col;
        }
    }

    sum
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}
