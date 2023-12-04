#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
    cell: char,
}

#[derive(Debug)]
struct NumberBox {
    start_point: Point,
    end_point: Point,
    value: usize,
}

impl NumberBox {
    fn is_adjacent(&self, other: &Point) -> bool {
        let x = self.start_point.x;
        let start_y = self.start_point.y;
        let end_y = self.end_point.y;
        (start_y..=end_y).any(|y| other.x.abs_diff(x) <= 1 && other.y.abs_diff(y) <= 1)
    }
}

fn main() {
    let points = include_str!("./day3.prod")
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(move |(y, cell)| Point { x, y, cell })
        });

    let mut number_boxes = vec![];
    let mut special_points = vec![];
    let mut current_box: Vec<Point> = vec![];
    for point in points {
        if point.y == 0 && !current_box.is_empty() {
            let value = current_box
                .iter()
                .map(|p| p.cell)
                .fold("".to_string(), |mut acc, ch| {
                    acc.push(ch);
                    acc
                })
                .parse::<usize>()
                .unwrap();
            number_boxes.push(NumberBox {
                start_point: *current_box.first().unwrap(),
                end_point: *current_box.last().unwrap(),
                value,
            });
            current_box.clear();
        }
        match point.cell {
            '0'..='9' => {
                current_box.push(point);
            }
            _ => {
                if !current_box.is_empty() {
                    let value = current_box
                        .iter()
                        .map(|p| p.cell)
                        .fold("".to_string(), |mut acc, ch| {
                            acc.push(ch);
                            acc
                        })
                        .parse::<usize>()
                        .unwrap();
                    number_boxes.push(NumberBox {
                        start_point: *current_box.first().unwrap(),
                        end_point: *current_box.last().unwrap(),
                        value,
                    });
                    current_box.clear();
                }
                if point.cell != '.' {
                    special_points.push(point);
                }
            }
        }
    }
    if !current_box.is_empty() {
        let value = current_box
            .iter()
            .map(|p| p.cell)
            .fold("".to_string(), |mut acc, ch| {
                acc.push(ch);
                acc
            })
            .parse::<usize>()
            .unwrap();
        number_boxes.push(NumberBox {
            start_point: *current_box.first().unwrap(),
            end_point: *current_box.last().unwrap(),
            value,
        });
        current_box.clear();
    }

    let result: usize = number_boxes
        .iter()
        .filter(|number_box| special_points.iter().any(|p| number_box.is_adjacent(p)))
        .map(|n| n.value)
        .sum();
    println!("Part 1: {}", result);

    let result: usize = special_points
        .into_iter()
        .filter_map(|p| {
            if p.cell == '*' {
                let matches = number_boxes
                    .iter()
                    .filter(|number_box| number_box.is_adjacent(&p))
                    .collect::<Vec<&NumberBox>>();
                if matches.len() == 2 {
                    return Some(matches.iter().map(|n| n.value).product::<usize>());
                }
            }
            None
        })
        .sum();
    println!("Part 2: {}", result);
}
