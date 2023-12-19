fn extrapolate_history(history: Vec<i64>) -> i64 {
    if history.iter().all(|v| v == &0) {
        return 0;
    }

    let mut next_history = vec![];
    for i in 1..history.len() {
        let val = history[i];
        let prev_val = history[i - 1];
        next_history.push(val - prev_val);
    }
    let next_value = extrapolate_history(next_history);
    let last_value = history.last().unwrap();
    return *last_value + next_value;
}

fn extrapolate_history_backwards(history: Vec<i64>) -> i64 {
    if history.iter().all(|v| v == &0) {
        return 0;
    }

    let mut next_history = vec![];
    for i in 1..history.len() {
        let val = history[i];
        let prev_val = history[i - 1];
        next_history.push(val - prev_val);
    }
    let next_value = extrapolate_history_backwards(next_history);
    let first_value = history.first().unwrap();
    return *first_value - next_value;
}

fn main() {
    let result: i64 = include_str!("./day9.prod")
        .lines()
        .map(|l| {
            let history = l
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            extrapolate_history(history)
        })
        .sum();
    println!("Part 1: {:?}", result);

    let result: i64 = include_str!("./day9.prod")
        .lines()
        .map(|l| {
            let history = l
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            extrapolate_history_backwards(history)
        })
        .sum();
    println!("Part 2: {:?}", result);
}
