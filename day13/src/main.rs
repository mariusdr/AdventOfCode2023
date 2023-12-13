use std::io::{prelude::*, self, Error};
use std::fs::File;

fn calc_horizontal_reflection(pattern: &Vec<Vec<bool>>, axis: usize) -> usize {
    let n = pattern.len();
    let m = pattern[0].len();
    let mut refwidth = 0;
    for j in axis..m {
        if axis < (j - axis) + 1 {
            continue;
        }
        let rj = axis - (j - axis) - 1;
        for i in 0..n {
            if pattern[i][j] != pattern[i][rj] {
                return refwidth;
            } 
        }
        refwidth += 1
    }
    refwidth
}

fn calc_vertical_reflection(pattern: &Vec<Vec<bool>>, axis: usize) -> usize {
    let n = pattern.len();
    let m = pattern[0].len();
    let mut refwidth = 0;
    for i in axis..n {
        if axis < (i - axis) + 1 {
            continue;
        }
        let ri = axis - (i - axis) - 1;
        for j in 0..m {
            if pattern[i][j] != pattern[ri][j] {
                return refwidth;
            } 
        }
        refwidth += 1;
    }
    refwidth
}

fn solve(pattern: &Vec<Vec<bool>>, base_axis: usize) -> usize {
    let n = pattern.len();
    let m = pattern[0].len();

    for axis in 1..m {
        let h = calc_horizontal_reflection(pattern, axis);
        if h == 0 || axis == base_axis {
            continue;
        }
        if axis + h == m || axis - h == 0 {
            return axis;
        }
    }

    for axis in 1..n {
        let v = calc_vertical_reflection(pattern, axis);
        if v == 0 || 100 * axis == base_axis {
            continue;
        }
        // println!("AXIS = {}; V = {}", axis, v);
        if axis + v == n || axis - v == 0 {
            return 100 * axis;
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;

    let mut sum = 0;
    let mut pattern: Vec<Vec<bool>> = Vec::new(); 
    for line in fbuf.lines() {
        if line.is_empty() {
            // sum += solve(&pattern); // solution to (1)

            let base_axis = solve(&pattern, 0);
            for i in 0..pattern.len() {
                let mut found_smudge = false;
                for j in 0..pattern[0].len() {
                    pattern[i][j] = !pattern[i][j];
                    let axis = solve(&pattern, base_axis);
                    if axis > 0 {
                        sum += axis;
                        found_smudge = true;
                        break;
                    }
                    pattern[i][j] = !pattern[i][j];
                }
                if found_smudge {
                    break;
                }
            }
            pattern.clear();
            continue;
        }
        pattern.push(line.chars().map(|c| c != '.').collect());
    }
    println!("sum is {}", sum);

    Ok(())
}
