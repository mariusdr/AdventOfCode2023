use std::collections::HashMap;
use std::io::{prelude::*, self};
use std::fs::File;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
enum Field {
    RoundRock = b'O',
    SquareRock = b'#',
    FreeSpace = b'.',
}

impl Field {
    fn is_square(&self) -> bool {
        *self == Field::SquareRock
    }
    
    fn is_round(&self) -> bool {
        *self == Field::RoundRock
    }

    fn is_free(&self) -> bool {
        *self == Field::FreeSpace
    }
}

impl TryFrom<char> for Field {
    type Error = std::io::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value as u8 {
            b'O' => Ok(Field::RoundRock),
            b'#' => Ok(Field::SquareRock),
            b'.' => Ok(Field::FreeSpace),
            _ => Err(std::io::Error::new(io::ErrorKind::InvalidInput, "invalid field"))
        }
    }
}

fn tilt_col_north(j: usize, pattern: &mut Vec<Vec<Field>>) {
    let mut seg = 0; 
    for i in 0..pattern.len() {
        if pattern[i][j].is_square() {
            seg = i + 1;
        } else if pattern[i][j].is_round() {
            let mut ix = i;
            for i0 in (seg..i).rev() {
                if !pattern[i0][j].is_free() {
                    break;
                }
                ix = i0;
            }
            let tmp = pattern[i][j];
            pattern[i][j] = pattern[ix][j];
            pattern[ix][j] = tmp;
        }
    }
}

fn tilt_row_west(i: usize, pattern: &mut Vec<Vec<Field>>) {
    let m = pattern[0].len();
    let mut seg = 0;
    for j in 0..m {
        if pattern[i][j].is_square() {
            seg = j + 1;
        } else if pattern[i][j].is_round() {
            let mut jx = j;
            for j0 in (seg..j).rev() {
                if !pattern[i][j0].is_free() {
                    break;
                }
                jx = j0;
            }
            let tmp = pattern[i][j];
            pattern[i][j] = pattern[i][jx];
            pattern[i][jx] = tmp;
        }
    } 
}

fn tilt_col_south(j: usize, pattern: &mut Vec<Vec<Field>>) {
    for i in (0..pattern.len()).rev() {
        if pattern[i][j].is_round() {
            let mut ix = i;
            for i0 in i+1..pattern.len() {
                if !pattern[i0][j].is_free() {
                    break;
                }
                ix = i0;
            }
            let tmp = pattern[i][j];
            pattern[i][j] = pattern[ix][j];
            pattern[ix][j] = tmp;
        }
    }
}

fn tilt_row_east(i: usize, pattern: &mut Vec<Vec<Field>>) {
    let m = pattern[0].len();
    for j in (0..m).rev() {
        if pattern[i][j].is_round() {
            let mut jx = j;
            for j0 in j+1..m {
                if !pattern[i][j0].is_free() {
                    break;
                }
                jx = j0;
            }
            let tmp = pattern[i][j];
            pattern[i][j] = pattern[i][jx];
            pattern[i][jx] = tmp;
        }
    }
}

fn dbg(pattern: &Vec<Vec<Field>>) {
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let c = pattern[i][j] as u8 as char;
            print!("{}", c);
        }
        print!("\n");
    }
    println!("================");
}

fn tilt_cycle(pattern: &mut Vec<Vec<Field>>) {
    let n = pattern.len();
    let m = pattern[0].len();

    for j in 0..m {
        tilt_col_north(j, pattern);
    }
    // dbg(pattern);
    // println!("============");

    for i in 0..n {
        tilt_row_west(i, pattern);
    }
    // dbg(pattern);
    // println!("============");

    for j in 0..m {
        tilt_col_south(j, pattern);
    }
    // dbg(pattern);
    // println!("============");

    for i in 0..n {
        tilt_row_east(i, pattern);
    }
    // dbg(pattern);
    // println!("============");
}


fn calc_load(pattern: &Vec<Vec<Field>>) -> usize {
    let n = pattern.len();
    let m = pattern[0].len();

    let mut sum = 0;
    for j in 0..m {
        for i in 0..n {
            if pattern[i][j].is_round() {
                sum += n - i;
            }
        }
    }
    sum
}

fn stringify(pattern: &Vec<Vec<Field>>) -> String {
    let mut s = String::new();
    for row in pattern {
        for x in row {
            s.push(*x as u8 as char);
        }
    }
    s
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;

    let mut pattern: Vec<Vec<Field>> = Vec::new();
    for line in fbuf.lines() {
        let row = line
            .chars()
            .map(|c| Field::try_from(c))
            .collect::<io::Result<Vec<Field>>>()?;
        pattern.push(row);
    }

    let mut cycle_start = 0;
    let mut cycle_length = 0;
    let mut map: HashMap<String, usize> = HashMap::new();
    for round in 0..1000000000 {
        println!("round {}", round);
        tilt_cycle(&mut pattern);
        let s = stringify(&pattern);
        let prev = *map.entry(s).or_insert(round); 
        if prev < round {
            cycle_start = prev;
            cycle_length = round - prev;
            println!("found cycle {} --> {} ({}; {})", prev, round, cycle_start, cycle_length);
            break;
        }
    }

    let mut stop = (1000000000 - cycle_start) / cycle_length;
    for round in (cycle_start..1000000000).step_by(cycle_length) {
        stop = round;
    }
    println!("stopped at {}", stop);

    for round in (stop + 1)..1000000000 {
        println!("round {}", round);
        tilt_cycle(&mut pattern);
    }
    let ld = calc_load(&pattern);
    println!("load is {}", ld);
    Ok(())
}
