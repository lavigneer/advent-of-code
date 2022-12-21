use std::collections::HashSet;

use anyhow::Result;

fn find_start_of_marker(s: &str, distinct_char: usize) -> usize {
    let mut result = s.char_indices().skip_while(|(idx, _chr)| {
        let next_chars = s
            .chars()
            .skip(*idx)
            .take(distinct_char)
            .collect::<HashSet<char>>();
        return next_chars.len() != distinct_char;
    });
    return result.next().unwrap().0 + distinct_char;
}

fn main() -> Result<()> {
    println!("Part 1: {}", find_start_of_marker(include_str!("./day6.prod"), 4));
    println!(
        "Part 2: {}",
        find_start_of_marker(include_str!("./day6.prod"), 14)
    );

    return Ok(());
}
