use anyhow::Result;

fn main() -> Result<()> {
    let mut vals = include_str!("./p1-1.prod").split("\n\n").map(|elf| 
        elf.split("\n").flat_map(|cal| cal.parse::<usize>()).sum::<usize>()
    ).collect::<Vec::<usize>>();

    vals.sort_by(|a, b| b.cmp(a));

    println!("{:?}", vals.iter().take(3).sum::<usize>());
    return Ok(());
}
