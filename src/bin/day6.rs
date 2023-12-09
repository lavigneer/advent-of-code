#[derive(Debug)]
struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn find_times(&self) -> usize {
        let mut count = 0;
        for i in 1..self.time {
            let distance = i * (self.time - i);
            if distance > self.record_distance {
                count += 1;
            }
        }
        return count;
    }
}

fn main() {
    let (times, distances) = include_str!("./day6.prod").split_once("\n").unwrap();
    let times = times
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let distances = distances
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race {
            time,
            record_distance: distance,
        })
        .collect::<Vec<Race>>();
    let result: usize = races.iter().map(|r| r.find_times()).product();
    println!("Part 1: {}", result);

    let (time, distance) = include_str!("./day6.prod").split_once("\n").unwrap();
    let (_, time) = time.split_once(":").unwrap();
    let time = time.trim().replace(" ", "").parse::<usize>().unwrap();
    let (_, distance) = distance.split_once(":").unwrap();
    let distance = distance.trim().replace(" ", "").parse::<usize>().unwrap();
    let race = Race {
        time,
        record_distance: distance,
    };
    let result = race.find_times();
    println!("Part 2: {}", result);
}
