use std::collections::HashMap;
use std::io::{prelude::*, self, Error};
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum SpringState {
    Broken = b'#',
    Functional = b'.',
    Unknown = b'?',
}

impl TryInto<SpringState> for char {
    type Error = std::io::Error;
    fn try_into(self) -> Result<SpringState, Self::Error> {
        let c = self as u8;
        match c {
            b'#' => Ok(SpringState::Broken),
            b'.' => Ok(SpringState::Functional),
            b'?' => Ok(SpringState::Unknown),
            _ => Err(Error::new(io::ErrorKind::InvalidInput, String::from(c as char)))
        }
    }
}

impl Into<char> for SpringState {
    fn into(self) -> char {
        self as u8 as char
    }
}

fn count_functionals(xs: &[SpringState]) -> usize {
    xs.iter().filter(|s| **s == SpringState::Functional).count()
}

fn count_brokens(xs: &[SpringState]) -> usize {
    xs.iter().filter(|s| **s == SpringState::Broken).count()
}

fn valid_grouping(xs: &[SpringState], n: u32) -> bool {
    let n = n as usize;
    if n > xs.len() {
        false
    } else if n == xs.len() {
        count_functionals(&xs[0..n]) == 0
    } else { // n < xs.len()
        (count_functionals(&xs[0..n]) == 0) && xs[n] != SpringState::Broken
    }
}

fn cache_key(xs: &[SpringState], ns: &[u32]) -> (String, String) {
    let xk: String = xs.iter().map(|&s| -> char {s.into()}).collect();
    let mut nk = String::new();
    for n in ns {
        let tmp = n.to_string();
        nk.extend(tmp.chars());
    }
    (xk, nk)
}

fn solve(xs: &[SpringState], ns: &[u32], mut cache: &mut HashMap<(String, String), usize>) -> usize {
    if xs.len() == 0 {
        if ns.len() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if ns.len() == 0 {
        if count_brokens(xs) == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    let ckey = cache_key(xs, ns);
    if cache.contains_key(&ckey) {
        return *cache.get(&ckey).unwrap();
    }

    let mut count = 0;

    // handle ? as .
    if xs[0] == SpringState::Functional || xs[0] == SpringState::Unknown {
        count += solve(&xs[1..], ns, &mut cache);
    }

    // handle ? as #
    if xs[0] == SpringState::Broken || xs[0] == SpringState::Unknown {
        let n = ns[0];
        if valid_grouping(xs, n) {
            if (n as usize) == xs.len() { 
                count += solve(&xs[n as usize..], &ns[1..], &mut cache);
            } else { 
                // we skip the next symbol after the grouping because 
                // it has to be '.' or '?' which is mapped to '.' in that case
                count += solve(&xs[n as usize + 1..], &ns[1..], &mut cache);
            }
        }
    }
    cache.insert(ckey, count);
    count
}

fn unfold_springs(xs: &[SpringState]) -> Vec<SpringState> {
    let mut res = Vec::with_capacity(5 * xs.len());
    for i in 0..5 {
        if i > 0 {
            res.push(SpringState::Unknown);
        }
        res.extend(xs.iter().map(|s| *s));
    } 
    res
}

fn unfold_numbers(ns: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(5 * ns.len());
    for _ in 0..5 {
        res.extend(ns.iter());
    }
    res
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;

    let mut cnt = 0;
    for line in fbuf.lines() {
        let xs: Vec<SpringState> = line
            .chars()
            .take_while(|c| !c.is_whitespace())
            .map(|c| c.try_into())
            .collect::<io::Result<Vec<SpringState>>>()?;

        let nstr: &str = &line[xs.len() + 1..];
        let ns: Vec<u32> = nstr
            .split(',')
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let xs = unfold_springs(&xs);
        let ns = unfold_numbers(&ns);

        let mut cache: HashMap<(String, String), usize> = HashMap::new();
        cnt += solve(&xs, &ns, &mut cache);
        println!("cur cnt = {}", cnt);
    }
    println!("count is {}", cnt);
    Ok(())
}
