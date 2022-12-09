/*

*/
use std::io::*;
use std::fs::*;

fn read_trees() -> (usize, usize, Vec::<Vec::<i32>>) {
    let input = File::open("data/2022-12-08-input.txt").unwrap();
    let reader = BufReader::new(input);
    let mut lines = reader.lines();
    let mut rows = Vec::<Vec::<i32>>::new();
    let mut num_cols = 0;

    while let Some(result) = lines.next() {
        let mut row = Vec::<i32>::new();
        if let Ok(line) = result {
            let cols = line.trim();
            num_cols = cols.len();
            for c in cols.chars() {
                row.push(c.to_string().parse::<i32>().unwrap());
            }
        }
        rows.push(row);
    }
    return (rows.len(), num_cols, rows);
}

fn calculate_visibility(num_rows: usize, num_cols: usize, trees: &Vec::<Vec::<i32>>) -> Vec::<Vec::<bool>> {
    // Prepare visibility look-up
    let mut visibility: Vec<Vec<bool>> = Vec::<Vec::<bool>>::new();
    let mut row_template = Vec::<bool>::new();
    row_template.resize(num_cols, false);
    visibility.resize(num_rows, row_template);

    // North
    for j in 0..num_cols {
        let mut current_height = -1;
        for i in (0..num_rows).rev() {
            if trees[i][j] > current_height {
                visibility[i][j] = true;
                current_height = trees[i][j];
            }
        }
    }

    // South
    for j in 0..num_cols {
        let mut current_height = -1;
        for i in 0..num_rows {
            if trees[i][j] > current_height {
                visibility[i][j] = true;
                current_height = trees[i][j];
            }
        }
    }
    
    // West
    for i in 0..num_rows {
        let mut current_height = -1;
        for j in (0..num_cols).rev() {
            if trees[i][j] > current_height {
                visibility[i][j] = true;
                current_height = trees[i][j];
            }
        }
    }

    // East
    for i in 0..num_rows {
        let mut current_height = -1;
        for j in 0..num_cols {
            if trees[i][j] > current_height {
                visibility[i][j] = true;
                current_height = trees[i][j];
            }
        }
    }

    return visibility;
}

fn part_1() {
    // Read trees
    let (num_rows, num_cols, trees) = read_trees();

    // Calculate visibility
    let visibility = calculate_visibility(num_rows, num_cols, &trees);

    // Count how many visible
    let mut total = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if visibility[i][j] {
                total += 1;
            }
        }
    }
    println!("Total Visible Trees: {0}", total);
}

/*

*/

fn calculate_scenic_scores(num_rows: usize, num_cols: usize, trees: &Vec::<Vec::<i32>>) -> (Vec::<Vec::<i32>>, i32) {
    let mut row_template = Vec::<i32>::new();
    row_template.resize(num_cols, 0);
    let mut scores: Vec<Vec<i32>> = Vec::<Vec::<i32>>::new();
    scores.resize(num_rows, row_template);

    let mut max_score = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            // North
            let mut num_north_trees = 0;
            for k in (0..i).rev() {
                num_north_trees += 1;
                if trees[k][j] >= trees[i][j] {
                    break;
                }
            }
            // South
            let mut num_south_trees = 0;
            for k in (i+1)..num_rows {
                num_south_trees += 1;
                if trees[k][j] >= trees[i][j] {
                    break;
                }
            }
            // West
            let mut num_west_trees = 0;
            for k in (0..j).rev() {
                num_west_trees += 1;
                if trees[i][k] >= trees[i][j] {
                    break;
                }
            }
            // East
            let mut num_east_trees = 0;
            for k in (j+1)..num_cols {
                num_east_trees += 1;
                if trees[i][k] >= trees[i][j] {
                    break;
                }
            }

            scores[i][j] = num_north_trees * num_south_trees * num_west_trees * num_east_trees;
            if scores[i][j] > max_score {
                max_score = scores[i][j];
            }
        }
    }
    return (scores, max_score);
}

fn part_2() {
    // Read trees
    let (num_rows, num_cols, trees) = read_trees();

    // Calculate scenic scores
    let visibility = calculate_visibility(num_rows, num_cols, &trees);
    let (_, max_score) = calculate_scenic_scores(num_rows, num_cols, &trees);

    // Find largest score
    println!("Maximum Score: {0}", max_score);
}

fn main() {
    part_1();
    part_2();
}