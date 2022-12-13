use std::fs::*;
use std::io::*;

fn char_to_height(c: char) -> i32 {
    c as i32 - 'a' as i32
}

fn read_heightmap() -> (Vec::<Vec::<i32>>, (usize, usize), (usize, usize)) {
    let input = File::open("data/2022-12-12-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut y = 0;
    let mut rows = Vec::<Vec::<i32>>::new();
    while let Some(Ok(line)) = lines.next() {
        let mut x = 0;
        let mut row = Vec::<i32>::new();
        for c in line.chars() {
            match c {
                'S' => {
                    start_pos = (x, y);
                    row.push(char_to_height('a'));
                },
                'E' => {
                    end_pos = (x, y);
                    row.push(char_to_height('z'));
                },
                _ => {
                    row.push(char_to_height(c));
                }
            }
            x += 1;
        }
        rows.push(row);
        y += 1;
    }
    return (rows, start_pos, end_pos);
}

fn push_neighbours_to_working_set(rows: &Vec::<Vec::<i32>>, current_pos: (usize, usize), row_scores: &mut Vec::<Vec::<i32>>, working_set: &mut Vec::<(usize, usize)>) {
    let (x, y) = current_pos;
    let height = rows[y][x];
    let current_score = row_scores[y][x];
    if x < rows[0].len() - 1 && rows[y][x + 1] <= height + 1 {
        if row_scores[y][x + 1] == -1 || row_scores[y][x + 1] > current_score + 1 {
            row_scores[y][x + 1] = current_score + 1;
            working_set.push((x + 1, y));
        }
    }
    if x > 0 && rows[y][x - 1] <= height + 1 {
        if row_scores[y][x - 1] == -1 || row_scores[y][x - 1] > current_score + 1 {
            row_scores[y][x - 1] = current_score + 1;
            working_set.push((x - 1, y));
        }
    }
    if y < rows.len() - 1 && rows[y + 1][x] <= height + 1 {
        if row_scores[y + 1][x] == -1 || row_scores[y + 1][x] > current_score + 1 {
            row_scores[y + 1][x] = current_score + 1;
            working_set.push((x, y + 1));
        }
    }
    if y > 0 && rows[y - 1][x] <= height + 1 {
        if row_scores[y - 1][x] == -1 || row_scores[y - 1][x] > current_score + 1 {
            row_scores[y - 1][x] = current_score + 1;
            working_set.push((x, y - 1));    
        }
    }
}

fn find_lowest_score<CanStartFn:Fn(usize, usize) -> bool>(rows: &Vec::<Vec::<i32>>, end_pos:(usize, usize), can_start_here: &CanStartFn) -> i32 {
    let mut row_scores = Vec::<Vec::<i32>>::new();
    let mut row_template = Vec::<i32>::new();
    row_template.resize(rows[0].len(), -1);
    row_scores.resize(rows.len(), row_template);

    for row in 0..rows.len() {
        for col in 0..rows[row].len() {
            if can_start_here(col, row) {
                row_scores[row][col] = 0;
                let mut working_set = Vec::<(usize, usize)>::new();
                push_neighbours_to_working_set(&rows, (col, row), &mut row_scores, &mut working_set);
                while !working_set.is_empty() {
                    let next_pos = working_set.pop().unwrap();
                    push_neighbours_to_working_set(&rows, next_pos, &mut row_scores, &mut working_set);
                }
            }
        }
    }
    return row_scores[end_pos.1][end_pos.0];
}

fn main() {
    let (rows, start_pos, end_pos) = read_heightmap();
    let minimum_steps_from_s = find_lowest_score(&rows, end_pos, &|x, y| (x, y) == start_pos);
    println!("Minimum Steps From S: {0}", minimum_steps_from_s);

    let minimum_steps_from_any = find_lowest_score(&rows, end_pos, &|x, y| rows[y][x] == 0);
    println!("Minimum Steps From Any Lowest Position: {0}", minimum_steps_from_any);
}