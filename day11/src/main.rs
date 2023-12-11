use std::io::{prelude::*, self};
use std::fs::File;

fn main() -> io::Result<()> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;

    let mut data: Vec<(i64, i64)> = fbuf.lines().enumerate().flat_map(|(i, line)| {
        let i = i;
        line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map( move |(j, _)| (i as i64, j as i64))
    }).collect();

    let x_max = data.iter().map(|p| p.0).max().unwrap();
    let y_max = data.iter().map(|p| p.1).max().unwrap();

    let expansion_factor = 1000000i64 - 1; // set this to 1 for the solution to task 1

    let mut empty_rows = vec![expansion_factor; x_max as usize + 1];
    let mut empty_cols = vec![expansion_factor; y_max as usize + 1];
    for (x, y) in data.iter() {
        empty_rows[*x as usize] = 0;
        empty_cols[*y as usize] = 0;
    } 

    for i in 0..empty_rows.len() - 1 {
        empty_rows[i + 1] += empty_rows[i];
    }
    
    for i in 0..empty_cols.len() - 1 {
        empty_cols[i + 1] += empty_cols[i];
    }
    
    for i in 0..data.len() {
        data[i].0 += empty_rows[data[i].0 as usize];
        data[i].1 += empty_cols[data[i].1 as usize];
    } 

    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            let v = data[i];
            let w = data[j];
            let d = (v.0 - w.0).abs() + (v.1 - w.1).abs();
            sum += d;
        }
    }

    println!("sum is {}", sum / 2);
    Ok(())
}
