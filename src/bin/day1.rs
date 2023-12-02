use std::collections::HashMap;

fn main() {
    let result = include_str!("./day1.prod")
        .lines()
        .filter_map(|line| {
            let line = line.trim_matches(|c: char| !c.is_numeric());
            if !line.is_empty() {
                let first = line.chars().next().unwrap();
                let last = line.chars().next_back().unwrap();
                return format!("{}{}", first, last).parse::<i64>().ok();
            }
            None
        })
        .fold(0, |acc, n| acc + n);
    println!("Part 1: {}", result);

    let mut name_replacement = HashMap::new();
    name_replacement.insert("one", "1");
    name_replacement.insert("two", "2");
    name_replacement.insert("three", "3");
    name_replacement.insert("four", "4");
    name_replacement.insert("five", "5");
    name_replacement.insert("six", "6");
    name_replacement.insert("seven", "7");
    name_replacement.insert("eight", "8");
    name_replacement.insert("nine", "9");

    let result = include_str!("./day1.prod")
        .lines()
        .filter_map(|line| {
            let mut line = line.to_string();
            for (key, val) in name_replacement.iter() {
                line = line.replace(key, &format!("{}{}{}", key, val, key));
            }
            let line = line.trim_matches(|c: char| !c.is_numeric());
            if !line.is_empty() {
                let first = line.chars().next().unwrap();
                let last = line.chars().next_back().unwrap();
                return format!("{}{}", first, last).parse::<i64>().ok();
            }
            None
        })
        .fold(0, |acc, n| acc + n);
    println!("Part 2: {}", result);
}
