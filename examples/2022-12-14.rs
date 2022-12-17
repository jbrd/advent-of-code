use std::collections::hash_set::*;
use std::fs::*;
use std::io::*;
use std::iter::*;
use std::cmp::*;

fn update_min_max(coord: (i32, i32), min: &mut (i32, i32), max: &mut (i32, i32)) {
    min.0 = min.0.min(coord.0);
    min.1 = min.1.min(coord.1);
    max.0 = max.0.max(coord.0);
    max.1 = max.1.max(coord.1);
}

fn read_rocks() -> (HashSet::<(i32, i32)>, (i32, i32), (i32, i32)) {
    let input = File::open("data/2022-12-14-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut rocks = HashSet::<(i32, i32)>::new();
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    while let Some(Ok(line)) = lines.next() {
        let coords: Vec<(i32, i32)> = line.replace(" -> ", "#").split("#").map(|x| {
            let mut elements = x.split(",").map(|x| x.parse::<i32>().unwrap());
            (elements.next().unwrap(), elements.next().unwrap())
        }).collect();

        let (mut x, mut y) = coords[0];
        rocks.insert((x,y));
        update_min_max((x, y), &mut min, &mut max);

        for i in 1..coords.len() {
            let (dx, dy) = ((coords[i].0 - x).signum(), (coords[i].1 - y).signum()); 
            while (x,y) != coords[i] {
                x += dx;
                y += dy;
                rocks.insert((x,y));
                update_min_max((x, y), &mut min, &mut max);
            }
        }
    }
    return (rocks, min, max);
}

fn simulate_sand<T:Fn((i32, i32)) -> bool>(collision_fn: &T, max: &(i32, i32), initial_pos: &(i32, i32)) -> HashSet::<(i32, i32)> {
    let mut sand = HashSet::<(i32, i32)>::new();
    let mut at_rest = true;
    while at_rest && !sand.contains(initial_pos) {
        let (mut x, mut y) = initial_pos;
        at_rest = false;
        while y <= max.1 {
            if !collision_fn((x, y + 1)) && !sand.contains(&(x, y + 1)) {
                y += 1;
            }
            else if !collision_fn((x - 1, y + 1)) && !sand.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            }
            else if !collision_fn((x + 1, y + 1)) && !sand.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            }
            else {
                sand.insert((x, y));
                at_rest = true;
                break;
            }
        }
    }
    return sand;
}

fn main() {
    let (rocks, _min, max) = read_rocks();
    const INITIAL_POS: (i32, i32) = (500, 0);

    // Part 1
    let sand = simulate_sand(&|coord| rocks.contains(&coord), &max, &INITIAL_POS);
    println!("Part 1 - Total Units Of Sand At Rest: {0}", sand.len());

    // Part 2
    let sand2 = simulate_sand(&|(x,y)| rocks.contains(&(x,y)) || y == max.1 + 2, &(max.0, max.1 + 2), &INITIAL_POS);
    println!("Part 2 - Total Units Of Sand At Rest: {0}", sand2.len());
}