use anyhow::Result;

fn main() -> Result<()> {
    let mut vals = include_str!("./day1.prod")
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .flat_map(|cal| cal.parse::<usize>())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    println!("Part 1 {:?}", vals.iter().max());

    vals.sort_by(|a, b| b.cmp(a));

    println!("Part 2 {:?}", vals.iter().take(3).sum::<usize>());

    Ok(())
}
