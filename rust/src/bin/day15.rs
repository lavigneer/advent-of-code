use rayon::prelude::*;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct ParseSensorBeaconError;

#[derive(Debug)]
struct SensorBeacon {
    pub sensor: Point,
    pub beacon: Point,
    pub beacon_distance: isize,
}

impl SensorBeacon {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        let x_beacon_diff = (beacon.x - sensor.x).abs();
        let y_beacon_diff = (beacon.y - sensor.y).abs();
        let beacon_distance = x_beacon_diff + y_beacon_diff;
        Self {
            sensor,
            beacon,
            beacon_distance,
        }
    }
    pub fn check_beacon_impossible(&self, point: &Point) -> bool {
        if self.beacon.eq(&point) {
            return false;
        }
        let x_diff = (point.x - self.sensor.x).abs();
        let y_diff = (point.y - self.sensor.y).abs();
        let point_distance = x_diff + y_diff;

        return point_distance <= self.beacon_distance;
    }
}

impl FromStr for SensorBeacon {
    type Err = ParseSensorBeaconError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut segments = s.split_whitespace();
        // Sensor
        segments.next();
        // at
        segments.next();

        let sensor_x = segments
            .next()
            .unwrap()
            .trim_end_matches(',')
            .split_once("=")
            .unwrap()
            .1
            .parse::<isize>()
            .unwrap();
        let sensor_y = segments
            .next()
            .unwrap()
            .trim_end_matches(':')
            .split_once("=")
            .unwrap()
            .1
            .parse::<isize>()
            .unwrap();

        // closest
        segments.next();
        // beacon
        segments.next();
        // is
        segments.next();
        // at
        segments.next();

        let beacon_x = segments
            .next()
            .unwrap()
            .trim_end_matches(',')
            .split_once("=")
            .unwrap()
            .1
            .parse::<isize>()
            .unwrap();
        let beacon_y = segments
            .next()
            .unwrap()
            .trim_end_matches(':')
            .split_once("=")
            .unwrap()
            .1
            .parse::<isize>()
            .unwrap();

        Ok(SensorBeacon::new(
            Point {
                x: sensor_x,
                y: sensor_y,
            },
            Point {
                x: beacon_x,
                y: beacon_y,
            },
        ))
    }
}

fn main() -> Result<()> {
    let sensors = include_str!("./day15.prod")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<SensorBeacon>>();
    let x_start = sensors
        .iter()
        .flat_map(|s| [s.sensor.x, s.sensor.x - s.beacon_distance])
        .min()
        .unwrap();
    let x_end = sensors
        .iter()
        .flat_map(|s| [s.sensor.x, s.sensor.x + s.beacon_distance])
        .max()
        .unwrap();

    let y_line = 2000000;
    let mut count = 0;
    for i in x_start..=x_end {
        let point = Point { x: i, y: y_line };
        if sensors.iter().any(|s| s.check_beacon_impossible(&point)) {
            count = count + 1;
        }
    }
    println!("Part 1: {}", count);

    let mut result = 0;
    'outer: for x in 0..4000000 {
        for y in 0..4000000 {
            let point = Point { x, y };
            if sensors
                .par_iter()
                .all(|s| s.check_beacon_impossible(&point))
            {
                result = (x * 4000000) + y;
                break 'outer;
            }
        }
    }
    println!("Part 2: {}", result);

    Ok(())
}
