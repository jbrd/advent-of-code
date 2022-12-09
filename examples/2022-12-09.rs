use std::collections::HashSet;
use std::fs::*;
use std::io::*;

fn simulate_rope(num_knots: usize) {
    let input = File::open("data/2022-12-09-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    // Initialise knots
    let mut knots = Vec::<(i32, i32)>::new();
    knots.resize(num_knots, (0, 0));

    // Initialise hash set to keep track of unique tail positions
    let mut tail_positions = HashSet::<(i32, i32)>::new();
    tail_positions.insert((0, 0));

    // Simulate
    while let Some(Ok(line)) = lines.next() {
        let mut tokens = line.trim().split_whitespace();
        let direction = tokens.next().unwrap();
        let count = tokens.next().unwrap().parse::<i32>().unwrap();

        // Move Head
        for _i in 0..count {
            match direction {
                "U" => { knots[0].1 += 1; },
                "D" => { knots[0].1 -= 1; },
                "L" => { knots[0].0 -= 1; },
                "R" => { knots[0].0 += 1; },
                _ => { }
            }

            // Update other knots
            for j in 1..num_knots {
                let between = (knots[j - 1].0 - knots[j].0, knots[j - 1].1 - knots[j].1);
                let distance_sq = between.0 * between.0 + between.1 * between.1;
                if distance_sq >= 4 {
                    knots[j].0 += i32::signum(between.0);
                    knots[j].1 += i32::signum(between.1);
                }
            }
            
            // Add tail position to hash set
            tail_positions.insert(*knots.last().unwrap());
        }
    }
    println!("Number Of Tail Positions: {0}", tail_positions.len());
}

fn main() {
    // Part 1
    simulate_rope(2);
    // Part 2
    simulate_rope(10);
}