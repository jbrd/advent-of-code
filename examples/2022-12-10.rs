use std::fs::*;
use std::io::*;

fn execute_cycles<CallbackFn:FnMut(i32, i32) -> ()>(callback: &mut CallbackFn) {
    let input = File::open("data/2022-12-10-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut cycle = 0;
    let mut x = 1;

    while let Some(Ok(line)) = lines.next() {
        let mut tokens = line.trim().split_whitespace();
        let instruction = tokens.next().unwrap();
        match instruction {
            "addx" => {
                let operand = tokens.next().unwrap().parse::<i32>().unwrap();
                cycle += 1;
                callback(cycle, x);
                cycle += 1;
                callback(cycle, x);
                x += operand;
            },
            "noop" => {
                cycle += 1;
                callback(cycle, x);
            }
            _ => {}
        }
    }
}

fn main() {
    // Part 1
    let mut total_signal_strength = 0;
    execute_cycles(&mut |cycle, x| {
        if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
            total_signal_strength += cycle * x;
        }
    });
    println!("Total Signal Strength: {0}", total_signal_strength);

    // Part 2
    let mut current_line = String::with_capacity(40);
    execute_cycles(&mut |_cycle, x| {
        if current_line.len() as i32 >= x - 1 && current_line.len() as i32 <= x + 1 {
            current_line.push('#');
        }
        else {
            current_line.push('.');
        }
        if current_line.len() == 40 {
            println!("{0}", current_line);
            current_line.clear();
        }
    });
    println!("{0}", current_line);
}