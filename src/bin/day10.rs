use std::collections::HashSet;

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Point {
    row: i32,
    col: i32,
    ch: char,
}

fn is_valid_move(curr: &Point, next: &Point) -> bool {
    match (next.row - curr.row, next.col - curr.col, next.ch) {
        (_, 0, '|') => true,
        (0, _, '-') => true,
        (1, 0, 'L') | (0, -1, 'L') => true,
        (1, 0, 'J') | (0, 1, 'J') => true,
        (-1, 0, '7') | (0, 1, '7') => true,
        (-1, 0, 'F') | (0, -1, 'F') => true,
        (_, _, 'S') => true,
        _ => false,
    }
}

fn walk<'a>(
    grid: &'a Vec<Vec<Point>>,
    curr: &'a Point,
    target: &Point,
    seen: &mut HashSet<Point>,
    path: &mut Vec<&'a Point>,
) -> bool {
    if curr.eq(target) && path.len() > 2 {
        path.push(curr);
        return true;
    } else if seen.contains(&curr) {
        return false;
    }
    seen.insert(curr.clone());
    let height = grid.len();
    let width = grid[0].len();

    path.push(curr);
    for (row_adjustment, col_adjustment) in DIRECTIONS {
        let new_row = curr.row + row_adjustment;
        let new_col = curr.col + col_adjustment;
        if new_row < 0 || new_row >= height as i32 || new_col < 0 || new_col >= width as i32 {
            continue;
        }
        let point = &grid[new_row as usize][new_col as usize];
        if is_valid_move(curr, point) && walk(grid, point, target, seen, path) {
            return true;
        }
    }
    path.pop();
    return false;
}

fn solve(grid: &Vec<Vec<Point>>, starting_point: &Point) -> usize {
    let mut seen = HashSet::new();
    let mut path = Vec::new();
    walk(grid, starting_point, starting_point, &mut seen, &mut path);
    path.len()
}

fn solve_contained_pipes(grid: &Vec<Vec<Point>>, starting_point: &Point) -> usize {
    let mut seen = HashSet::new();
    let mut path = Vec::new();
    walk(grid, starting_point, starting_point, &mut seen, &mut path);

    let mut inside_pipes = HashSet::new();
    for row in grid {
        let mut inside = false;
        for point in row {
            if path.contains(&point) {
                if point.ch != '-' {
                    inside = !inside;
                }
            } else if inside {
                inside_pipes.insert(point);
            }
        }
    }

    for col in 0..grid[0].len() {
        for row in grid {
            let cell = &row[col];
            if path.contains(&&cell) {
                break;
            }
            if inside_pipes.contains(&cell) {
                inside_pipes.remove(&cell);
            }
        }

        for row in grid.iter().rev() {
            let cell = &row[col];
            if path.contains(&&cell) {
                break;
            }
            if inside_pipes.contains(&cell) {
                inside_pipes.remove(&cell);
            }
        }
    }

    for row in grid {
        for point in row {
            if path.contains(&point) {
                print!("{}", point.ch);
            } else if inside_pipes.contains(&point) {
                print!("I");
            } else {
                print!("0");
            }
        }
        println!();
    }
    inside_pipes.len()
}

fn main() {
    let grid = include_str!("./day10.prod")
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, ch)| Point {
                    row: row.try_into().unwrap(),
                    col: col.try_into().unwrap(),
                    ch,
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut starting_point = None;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col].ch == 'S' {
                starting_point = Some(&grid[row][col]);
            }
        }
    }
    let starting_point = starting_point.unwrap();
    let count = solve(&grid, starting_point);
    println!("Part 1: {}", count / 2);

    let inside_pipes = solve_contained_pipes(&grid, starting_point);
    println!("Part 2: {}", inside_pipes);
}
