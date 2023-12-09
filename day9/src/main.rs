use std::io::{prelude::*, BufReader, self};
use std::fs::File;
use std::error::Error;

fn skip_non_number(xs: &str) -> Result<&str, Box<dyn Error>> {
    let s = xs.chars().take_while(|c| !c.is_digit(10) && *c != '-').count();
    Ok(&xs[s..])
}

fn parse_number(xs: &str) -> Result<(i64, &str), Box<dyn Error>> {
    let mut xs = skip_non_number(xs)?;

    let sign = if xs.starts_with('-') {
        xs = &xs[1..];
        -1 as i64
    } else {
        1 as i64
    };

    let digits: Vec<i64> = xs
        .chars()
        .take_while(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect();

    let ln = digits.len();
    let value: i64 = sign * digits
        .iter()
        .enumerate()
        .map(|(i, x)| x * 10i64.pow(ln as u32 - 1 - i as u32))
        .sum::<i64>();
    
    Ok((value, &xs[ln as usize..]))
}

fn parse_line(xs: &str) -> Result<(Vec<i64>, &str), Box<dyn Error>> {
    let mut ys = xs;
    let mut ns = Vec::new();
    while ys.len() > 0 {
        let (n, xs) = parse_number(ys)?;
        ns.push(n);
        ys = xs;
    }
    Ok((ns, ys))
}

fn parse_inp(xs: &str) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    xs.lines().map(|line| -> Result<Vec<i64>, Box<dyn Error>> {
       let (xs, _) = parse_line(line)?;
        Ok(xs)
    }).collect::<Result<Vec<Vec<i64>>, Box<dyn Error>>>()
}

#[derive(Debug)]
struct Diffs {
    diffs: Vec<Vec<i64>>,
    last_row: usize,
    // row_sums: Vec<i64>,
}

impl Diffs {
    fn calc(row0: &Vec<i64>) -> Self {
        let n = row0.len();
        let mut diffs = vec![vec![0; n]; n + 1];

        let mut last_row = 0;

        for j in 0..n {
            diffs[0][j] = row0[j];
            if diffs[0][j] != 0 {
                last_row = 1;
            }
        }

        for i in 1..n {
            for j in (i - 1)..(n - 1) {
                let d_next = diffs[i - 1][j + 1];
                let d_prev = diffs[i - 1][j];
                diffs[i][j + 1] = d_next - d_prev;
                if diffs[i][j + 1] != 0 {
                    last_row = i;
                }
            }
        }
        Self { diffs, last_row }
    }

    fn solve_row_forward(&self, i: usize, diff: i64) -> i64 {
        let j = self.diffs[0].len() - 1;
        let d_prev = self.diffs[i][j];
        let d_next = diff + d_prev; 
        d_next
    }

    fn solve_forward(&self) -> i64 {
        let istart = self.last_row;
        let mut value = 0;
        for i in (0..=istart).rev() {
            // println!("value in row {} is {}", i+1, value);
            value = self.solve_row_forward(i, value);
        }
        // println!("value in row 0 is {}", value);
        value 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("src/data.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let data = parse_inp(&buf)?;

    let mut sum = 0;
    for row in data {
        let row: Vec<i64> = row.iter().rev().map(|x| *x).collect(); // to solve task 1, delete this line.
        let diffs = Diffs::calc(&row);

        for dr in &diffs.diffs {
            println!("{:?}", dr);
        }
        let val = diffs.solve_forward();
        sum += val;
    }
    println!("sum is {}", sum);
    Ok(())
}
