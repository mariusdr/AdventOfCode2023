use std::io::{BufReader, prelude::*};
use std::io;
use std::fs::File;
use std::fmt::Write;

//Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red

fn mkerr(txt: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, txt)
}

fn parse_game_no(xs: &str) -> io::Result<(u32, &str)> {
    if !xs[0..5].contains("Game ") {
        return Err(mkerr("missing Game tag"));
    }
    let xs = &xs[5..];

    let ds = xs.chars().take_while(|c| c.is_digit(10)).collect::<String>();
    let dig = ds.parse::<u32>().map_err(|_| mkerr("game no is not a digit"))?;
    Ok((dig, &xs[2 + ds.len()..]))
    // let dig = xs.chars().next().ok_or(mkerr("not a digit"))?.to_digit(10).ok_or(mkerr("not a digi"))?;
    // Ok((dig, &xs[3..]))
}

#[derive(Debug)]
enum Color {
    Red, Green, Blue
}

fn parse_ball(xs: &str) -> io::Result<(Color, u32, &str)> {
    
    let ds = xs.chars().take_while(|c| c.is_digit(10)).collect::<String>();
    let count = ds.parse::<u32>().map_err(|_| mkerr("not a digit"))?;
    let xs = &xs[ds.len() + 1..];
    let c = xs.chars().next().ok_or(mkerr("missing color"))?;
    match c {
        'g' => Ok((Color::Green, count, &xs[5..])),
        'r' => Ok((Color::Red, count, &xs[3..])),
        'b' => Ok((Color::Blue, count, &xs[4..])),
        _ => Err(mkerr("invalid color")),
    }
}

fn parse_sep(xs: &str) -> io::Result<(bool, &str)> {
    if let Some(c) = xs.chars().next() {
        return match c {
            ',' => Ok((false, &xs[2..])),
            ';' => Ok((true, &xs[2..])),
            _ => Err(mkerr("unexpected char")),
        }
    }
    Ok((true, ""))
}

fn parse_set(xs: &str) -> io::Result<(Vec<(Color, u32)>, &str)> {
    let mut game_set: Vec<(Color, u32)> = Vec::new();
    let mut cur = xs;
    loop {
        let (col, count, rest) = parse_ball(cur)?;
        game_set.push((col, count));
        cur = rest;
        let (end_of_set, rest) = parse_sep(cur)?;
        cur = rest;
        if end_of_set {
            break;
        }
    }
    Ok((game_set, cur))
}

#[derive(Debug)]
struct Game {
    no: u32, 
    sets: Vec<Vec<(Color, u32)>>,
}

fn parse_game(xs: &str) -> io::Result<Game> {
    let (no, xs) = parse_game_no(xs)?;
    let mut sets: Vec<Vec<(Color, u32)>> = Vec::new();
    let mut cur = xs;
    while cur.len() > 0 {
        let (set, rest) = parse_set(cur)?;
        sets.push(set);
        cur = rest;
    }
    Ok(Game{no, sets})
}

impl Game {
    fn possible(&self) -> bool {
        let mut res = true;
        for set in &self.sets {
            for (col, cnt) in set {
                match col {
                    Color::Blue => res &= *cnt <= 14,
                    Color::Red => res &= *cnt <= 12,
                    Color::Green => res &= *cnt <= 13,
                }
            }
        }
        res
    }

    fn possible_set(&self) -> (u32, u32, u32) {
        let mut max_red = 0u32;
        let mut max_green = 0u32;
        let mut max_blue = 0u32;
        for set in &self.sets {
            for (col, cnt) in set {
                match col {
                   Color::Red => max_red = std::cmp::max(max_red, *cnt),
                   Color::Green => max_green = std::cmp::max(max_green, *cnt),
                   Color::Blue => max_blue = std::cmp::max(max_blue, *cnt),
                }
            }
        }
        (max_red, max_green, max_blue)
    }
}

fn main() -> io::Result<()> {
    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file); 

    let mut sum = 0u32;
    for line in reader.lines() {
        let line = line?.clone();
        let game = parse_game(&line)?;
        let (r, g, b) = game.possible_set();
        sum += r * g * b;
    }
    println!("sum is {}", sum);

    Ok(())
}
