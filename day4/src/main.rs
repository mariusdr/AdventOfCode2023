use std::collections::HashSet;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::io;

fn skip_ws(xs: &str) -> io::Result<&str> {
    let mut xs = xs;
    while xs.starts_with(' ') {
        xs = &xs[1..];
    }
    Ok(xs)
}

fn parse_number(xs: &str) -> io::Result<(u32, &str)> {
    let xs = skip_ws(xs)?;
    let digits: Vec<u32> = xs
        .chars()
        .take_while(|x| x.is_digit(10))
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let ln = digits.len() as u32;
    let value: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, x)| x * 10u32.pow(ln - 1 - i as u32))
        .sum();
    
    Ok((value, &xs[ln as usize..]))
}

fn parse_game_tag(xs: &str) -> io::Result<(u32, &str)> {
    if !xs.starts_with("Card ") {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "missing Card tag");
        return Err(err);
    }
    let xs = &xs[5..];

    let (value, xs) = parse_number(xs)?;
    let xs = &xs[2..];
    Ok((value, xs))
}

fn parse_sep(xs: &str) -> io::Result<&str> {
    if !xs.starts_with('|') {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "failed to parse seperator");
        return Err(err);
    }
    Ok(&xs[1..])
}

fn parse_winning_numbers(xs: &str) -> io::Result<(Vec<u32>, &str)> {
    let mut ns = Vec::new();
    let mut xs = xs;
    while parse_sep(xs).is_err() {
        let (n, ys) = parse_number(xs)?;
        ns.push(n);
        xs = &ys[1..];
    }
    Ok((ns, &xs[2..]))
}

fn parse_dealt_numbers(xs: &str) -> io::Result<(Vec<u32>, &str)> {
    let mut ns = Vec::new();
    let mut xs = xs;
    while let Ok((n, ys)) = parse_number(xs) {
        ns.push(n);
        if ys.len() > 0 {
            xs = &ys[1..];
        } else {
            xs = ys;
            break;
        }
    }
    Ok((ns, xs))
}

fn parse(xs: &str) -> io::Result<(u32, Vec<u32>, Vec<u32>, &str)> {
    let (tag, xs) = parse_game_tag(xs)?;
    let (ws, xs) = parse_winning_numbers(xs)?;
    let (ds, xs) = parse_dealt_numbers(xs)?;
    Ok((tag, ws, ds, xs))
}

// solves 4.1
fn compute_game_score(winners: Vec<u32>, candidates: Vec<u32>) -> usize {
    let hs: HashSet<u32> = winners.iter().map(|x| *x).collect();
    let mut value = 0;    
    for c in candidates {
        if hs.contains(&c) {
            if value == 0 {
                value = 1;
            } else {
                value *= 2;
            }
        }
    }
    value
}

fn compute_winners(winners: Vec<u32>, candidates: Vec<u32>) -> usize { 
    let hs: HashSet<u32> = winners.iter().map(|x| *x).collect();
    candidates.iter().filter(|c| hs.contains(&c)).count()
}

fn spawn(og: &Vec<usize>, idx: usize) -> usize {
    if idx > og.len() {
        panic!("index out of bounds");
    }
    let m = og[idx] as usize;
    let mut total = m;
    for i in 1..=m {
        total += spawn(og, idx + i);
    }
    total
}

fn main() -> io::Result<()> {
    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file);

    let mut og: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let (card, winners, candidates, _) = parse(&line?)?;
        og.push(0);
        og[card as usize - 1] = compute_winners(winners, candidates);
    }

    let mut total = og.len();
    for card in 0..og.len() {
        total += spawn(&og, card);
    }

    println!("solution is {}", total);
    Ok(())
}
