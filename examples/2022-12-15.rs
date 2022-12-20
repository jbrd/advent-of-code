use std::fs::*;
use std::io::*;
use std::iter::*;
use std::cmp::*;

type Vec2 = (i32, i32);

fn calc_min(a: &Vec2, b: &Vec2) -> Vec2 {
    (a.0.min(b.0), a.1.min(b.1))
}

fn calc_max(a: &Vec2, b: &Vec2) -> Vec2 {
    (a.0.max(b.0), a.1.max(b.1))
}

fn dist(a: &Vec2, b: &Vec2) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn read_sensors_and_beacons() -> Vec<(Vec2, Vec2)> {
    let input = File::open("data/2022-12-15-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut result = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let numbers = line.replace("Sensor at x=", "")
            .replace(", y=", ",")
            .replace(": closest beacon is at x=", ",")
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec::<i32>>();
        result.push(((numbers[0], numbers[1]), (numbers[2], numbers[3])));
    }
    result
}

fn main() {
    let sensors = read_sensors_and_beacons();
    let (mut min, mut max) = ((i32::MAX, i32::MAX), (i32::MIN, i32::MIN));
    for (a, b) in &sensors {
        min = calc_min(&min, &calc_min(a, b));
        max = calc_max(&max, &calc_max(a, b));
    }

    for y in min.1..=max.1 {
        for (sensor, beacon) in &sensors {
            let sensor_to_beacon = dist(&sensor, &beacon);
            min = calc_min(&min, &(sensor.0 - sensor_to_beacon, y));
            max = calc_max(&max, &(sensor.0 + sensor_to_beacon, y));
        }
    }

    // Part 1
    const Y: i32 = 2000000;
    let mut count = 0;
    for x in min.0..=max.0 {
        let cell = (x, Y);
        let mut occupied = false;
        for (sensor, beacon) in &sensors {
            let sensor_to_beacon = dist(&sensor, &beacon);
            let sensor_to_cell = dist(&sensor, &cell);
            if cell != *beacon && sensor_to_cell <= sensor_to_beacon {
                occupied = true;
                break;
            }
        }
        if occupied {
            count += 1;
        } 
    }
    println!("Part 1: {0} positions cannot contain a beacon", count);

    // Part 2
    for y in 0..4000000 {
        let mut intervals = Vec::<(i32, i32)>::new();
        for (sensor, beacon) in &sensors {
            let radius = dist(&sensor, &beacon) - (y - sensor.1).abs();
            if radius >= 0 {
                let min = sensor.0 - radius;
                let max = sensor.0 + radius;
                if intervals.is_empty() {
                    intervals.push((min, max));
                }
                else {
                    let mut max_index: i32 = -1;
                    for i in 0..intervals.len() {
                        if max >= intervals[i].0 {
                            max_index += 1;
                        }
                        else {
                            break;
                        }
                    }

                    if max_index < 0 || max > intervals[max_index as usize].1 {
                        intervals.insert((max_index + 1) as usize, (min, max));
                        max_index += 1;
                    }
                    else {
                        intervals[max_index as usize].0 = min.min(intervals[max_index as usize].0);
                    }

                    let mut min_index: i32 = max_index;
                    while min_index > 0 && intervals[min_index as usize].0 <= intervals[min_index as usize - 1].1 + 1 {
                        intervals[min_index as usize].0 = min.min(intervals[min_index as usize - 1].0);
                        intervals.remove(min_index as usize - 1);
                        min_index -= 1;
                    }
                }
            }
        }
        if intervals.len() == 2 {
            let x = intervals[0].1 + 1;
            assert!(x == intervals[1].0 - 1);
            let frequency: u64 = x as u64 * 4000000 + y as u64;
            println!("Part 2: Beacon Found at {0:?} with frequency {1}", (x,y), frequency);
            break;
        }
    }
}