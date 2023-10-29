use std::collections::HashMap;

use anyhow::Result;
use petgraph::algo::dijkstra;
use petgraph::graph::Graph;

static DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn get_char_value(ch: char) -> u32 {
    match ch {
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        _ => ch as u32,
    }
}

fn main() -> Result<()> {
    let graph: Vec<&str> = include_str!("./day12.prod").lines().collect();
    let mut graph_map = HashMap::new();
    let mut built_graph = Graph::new();

    let mut starting_point = None;
    let mut starting_points = vec![];
    let mut ending_point = None;
    for (i, row) in graph.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            let p = Point { x: i, y: j };
            let n = built_graph.add_node(());
            match ch {
                'S' => {
                    starting_point = Some(n);
                    starting_points.push(n);
                }
                'E' => {
                    ending_point = Some(n);
                }
                'a' => {
                    starting_points.push(n);
                }
                _ => {}
            }
            graph_map.insert(p, n);
        }
    }
    for (i, row) in graph.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            let ch_value = get_char_value(ch);
            for (x, y) in DIRECTIONS {
                let new_x = i as i32 + x;
                let new_y = j as i32 + y;
                if new_x < 0
                    || new_x > (graph.len() - 1) as i32
                    || new_y < 0
                    || new_y > (row.len() - 1) as i32
                {
                    continue;
                }
                let other_ch =
                    get_char_value(graph[new_x as usize].chars().nth(new_y as usize).unwrap());
                if (other_ch as i64) <= (ch_value as i64 + 1) {
                    if let Some(node) = graph_map.get(&Point { x: i, y: j }) {
                        if let Some(other_node) = graph_map.get(&Point {
                            x: new_x as usize,
                            y: new_y as usize,
                        }) {
                            built_graph.update_edge(*node, *other_node, 1);
                        }
                    }
                }
            }
        }
    }
    // println!(
    //     "{:?}",
    //     petgraph::dot::Dot::with_config(&built_graph, &[petgraph::dot::Config::GraphContentOnly])
    // );
    let starting_point = starting_point.unwrap();
    let ending_point = ending_point.unwrap();

    let d = dijkstra(&built_graph, starting_point, Some(ending_point), |_| 1);
    println!("Part 1: {:?}", d.get(&ending_point));

    let min_start = starting_points
        .iter()
        .map(|s| {
            let mut d = dijkstra(&built_graph, *s, Some(ending_point), |_| 1);
            d.remove(&ending_point).unwrap_or(i32::MAX)
        })
        .min();
    println!("Part 2: {:?}", min_start);

    Ok(())
}
