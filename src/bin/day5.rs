use anyhow::Error;
use std::str::FromStr;

struct ConversionRow {
    source: usize,
    destination: usize,
    range: usize,
}

impl FromStr for ConversionRow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split_whitespace();
        let destination = splits.next().unwrap().parse::<usize>().unwrap();
        let source = splits.next().unwrap().parse::<usize>().unwrap();
        let range = splits.next().unwrap().parse::<usize>().unwrap();
        Ok(Self {
            destination,
            source,
            range,
        })
    }
}

impl ConversionRow {
    fn conversion_num(&self, other: usize) -> Option<usize> {
        if other >= self.source && other <= self.source + self.range {
            return other
                .checked_sub(self.source)
                .map(|diff| self.destination + diff);
        }
        None
    }
}

#[derive(Debug)]
struct SeedRange {
    start: usize,
    range: usize,
}

fn convert_to_new_ranges(
    seed_ranges: Vec<SeedRange>,
    section: Vec<ConversionRow>,
) -> Vec<SeedRange> {
    let mut new_seed_ranges = vec![];

    for seed_range in seed_ranges {
        let mut seed_start = seed_range.start;
        let seed_end = seed_start + seed_range.range;
        let mut matching_rows = section
            .iter()
            .filter(|row| {
                let row_start = row.source;
                let row_end = row_start + row.range;
                (row_start >= seed_start && row_start <= seed_end)
                    || (seed_start >= row_start && seed_start <= row_end)
                    || (row_end >= seed_start && row_end <= seed_end)
                    || (seed_end >= row_start && seed_end <= row_end)
            })
            .collect::<Vec<&ConversionRow>>();
        matching_rows.sort_by_key(|r| r.source);
        if matching_rows.is_empty() {
            new_seed_ranges.push(seed_range);
        } else {
            for row in matching_rows {
                let row_start = row.source;
                let row_end = row_start + row.range;
                if seed_start < row_start {
                    new_seed_ranges.push(SeedRange {
                        start: seed_start,
                        range: row_start - seed_start,
                    });
                    seed_start = row_start;
                }
                if seed_end > row_end {
                    new_seed_ranges.push(SeedRange {
                        start: row.conversion_num(seed_start).unwrap(),
                        range: row_end - seed_start,
                    });
                    seed_start = row_end;
                } else {
                    new_seed_ranges.push(SeedRange {
                        start: row.conversion_num(seed_start).unwrap(),
                        range: seed_end - seed_start,
                    });
                }
            }
        }
    }
    new_seed_ranges
}

fn main() {
    let mut sets = include_str!("./day5.prod").split("\n\n");
    let seeds = sets.next().unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let mut seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for section in sets {
        let section_lines = section
            .lines()
            .skip(1)
            .map(|l| l.parse::<ConversionRow>().unwrap());
        for i in 0..seeds.len() {
            let seed = seeds[i];
            let new_value = section_lines
                .clone()
                .find_map(|l| l.conversion_num(seed))
                .or(Some(seed))
                .unwrap();
            seeds[i] = new_value;
        }
    }
    println!("Part 1: {:?}", seeds.iter().min().unwrap());

    let mut sets = include_str!("./day5.prod").split("\n\n");
    let seeds = sets.next().unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split_whitespace()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let seeds = seeds
        .chunks(2)
        .map(|a| SeedRange {
            start: a[0],
            range: a[1],
        })
        .collect::<Vec<SeedRange>>();

    let mut sections = sets.map(|section| {
        section
            .lines()
            .skip(1)
            .map(|l| l.parse::<ConversionRow>().unwrap())
            .collect::<Vec<ConversionRow>>()
    });

    let mut result = seeds;
    while let Some(section) = sections.next() {
        result = convert_to_new_ranges(result, section);
    }
    println!("Part 2: {:?}", result.iter().min_by_key(|s| s.start).unwrap().start);

}
