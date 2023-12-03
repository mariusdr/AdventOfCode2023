use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::io;
use std::iter::Peekable;

struct DigitGroup<Iter: Iterator<Item = (usize, u32)>> {
    iter: Peekable<Iter>
}

impl<Iter: Iterator<Item = (usize, u32)>> DigitGroup<Iter> {
    fn new(iter: Iter) -> Self {
        Self { iter: iter.peekable() }
    }
}

impl<Iter: Iterator<Item = (usize, u32)>> Iterator for DigitGroup<Iter> {
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut digits: Vec<u32> = Vec::new();
        let (mut ipos, dig) = self.iter.next()?;
        let start_pos = ipos;
        digits.push(dig);

        while let Some((pos, dig)) = self.iter.peek() {
            if *pos == ipos + 1 {
                ipos = *pos;
                digits.push(*dig);
                self.iter.next()?;
            } else {
                break;
            }
        }

        let n = (digits.len() - 1) as u32;
        let value = digits.iter().enumerate().map(|(i, d)| {
            d * 10u32.pow(n - i as u32)
        }).sum();

        Some((start_pos, ipos + 1, value))
    }
}

fn halo(coord: (usize, usize), max: (usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = coord;
    let (w, h) = max;
    let mut halo = Vec::new();
    if i > 0 && j > 0 { halo.push((i - 1, j - 1)); }
    if i > 0 { halo.push((i - 1, j)); }
    if j > 0 { halo.push((i, j - 1)); }
    if i < w - 1 && j < h - 1 { halo.push((i + 1, j + 1)); }
    if i < w - 1 { halo.push((i + 1, j)); }
    if j < h - 1 { halo.push((i, j + 1)); }
    if i > 0 && j < h - 1 { halo.push((i - 1, j + 1)); }
    if i < w - 1 && j > 0 { halo.push((i + 1, j - 1)); }
    halo
}


fn main() -> io::Result<()> {
    const WIDTH: usize = 255;
    const HEIGHT: usize = 255;
    const DIM: (usize, usize) = (WIDTH, HEIGHT);

    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file);

    let mut values: Vec<u32> = Vec::new();
    values.push(0);

    let mut grid = vec![vec![0usize; WIDTH]; HEIGHT];
    let mut symbols: Vec<(usize, usize)> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?.clone();

        let it = line  
            .char_indices()
            .filter(|(_, c)| c.is_digit(10))
            .map(|(j, c)| (j, c.to_digit(10).unwrap()));
        
        let it = DigitGroup::new(it);
        for group in it {
            let (start, end, value) = group;
            values.push(value);
            let idx = values.len() - 1;
            for j in start..end {
                grid[i][j] = idx;
            }
        }

        let it = line
            .char_indices()
            .filter(|(_, c)| !c.is_digit(10) && *c == '*')
            .map(|(j, _)| (i, j));
        let mut syms = it.collect();
        symbols.append(&mut syms);
    }

    let mut sum = 0u32;
    for (i, j) in symbols {
        let mut vals = halo((i, j), DIM).iter().map(|(i, j)| {
            let idx = grid[*i][*j];
            values[idx]
        }).collect::<Vec<u32>>();
        vals.sort();
        vals.dedup(); 
        if vals.len() == 3 {
            sum += vals[1] * vals[2]
        }
    }
    println!("solution is {}", sum);

    Ok(())
}
