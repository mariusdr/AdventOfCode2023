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

// t = time button is held
// L = time limit of the race
// K = record distance
// 
// the distance traveled is D(t) = t * (L - t)
// 
// Find all t in 1..L where D(t) - K > 0, so basically solve for t:
// 
// 0 = t * (L - t) - K = -t^2 + tL - K 
// 0 = t^2 - tL + K
//
// ==> the basic quadratic equation (abc-formel!!)

fn closed_form(time_limit: usize, record_dist: usize) -> (usize, usize) {
    let l = time_limit as f64;
    let k = record_dist as f64;
    let t1 = (l + (l*l - 4.0 * k).sqrt()) * 0.5;
    let t2 = (l - (l*l - 4.0 * k).sqrt()) * 0.5;
    (t1 as usize, t2 as usize)
}

fn main() -> io::Result<()>{
    let mut file = File::open("src/data.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let parts = buf.split('\n').collect::<Vec<&str>>();
    let (ts, _) = parse_line(parts[0])?;
    let (ds, _) = parse_line(parts[1])?;

    for (&tl, &rd) in ts.iter().zip(ds.iter()) {
        let tl = tl as usize;
        let rd = rd as usize;

        let (t0, t1) = closed_form(tl, rd);

        let delta = std::cmp::max(t0, t1) - std::cmp::min(t0, t1);
        println!("solution is {}", delta);
    }

    Ok(())
}
