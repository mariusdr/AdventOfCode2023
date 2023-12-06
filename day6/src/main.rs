use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::io;

fn skip_non_number(xs: &str) -> io::Result<&str> {
    let s = xs.chars().take_while(|c| !c.is_digit(10)).count();
    Ok(&xs[s..])
}

fn parse_number(xs: &str) -> io::Result<(usize, &str)> {
    let xs = skip_non_number(xs)?;
    let digits: Vec<usize> = xs
        .chars()
        .take_while(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let ln = digits.len();
    let value: usize = digits
        .iter()
        .enumerate()
        .map(|(i, x)| x * 10usize.pow(ln as u32 - 1 - i as u32))
        .sum();
    
    Ok((value, &xs[ln as usize..]))
}

fn parse_line(xs: &str) -> io::Result<(Vec<usize>, &str)> {
    let mut ys = xs;
    let mut ns = Vec::new();
    while ys.len() > 0 {
        let (n, xs) = parse_number(ys)?;
        ns.push(n);
        ys = xs;
    }
    Ok((ns, ys))
}

// travel distance = time held * (time limit - time held)

#[inline]
fn calc_dist(time_held: usize, time_limit: usize) -> usize {
    time_held * (time_limit - time_held)
}

fn main() -> io::Result<()>{
    let mut file = File::open("src/data.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let parts = buf.split('\n').collect::<Vec<&str>>();
    let (ts, _) = parse_line(parts[0])?;
    let (ds, _) = parse_line(parts[1])?;

    let mut prod = 1;
    for (&tl, &rd) in ts.iter().zip(ds.iter()) {
        let tl = tl as usize;
        let rd = rd as usize;

        let mut ways = 0; 
        for t in 1..=tl {
            if calc_dist(t, tl) > rd {
                ways += 1;
            }
        }
        prod *= ways;
    }
    println!("solution is {}", prod);

    Ok(())
}
