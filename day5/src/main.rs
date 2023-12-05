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

fn parse_seed_list<B: BufRead>(it: &mut std::io::Lines<B>) -> io::Result<Vec<u32>> {
    let line = it.next().ok_or(io::Error::new(io::ErrorKind::InvalidData, "seed line empty"))??; //?
    if !line.starts_with("seeds: ") {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "failed to match pattern 'seed: '");
        return Err(err);
    }
    let mut nums = Vec::new();
    let mut xs = &line[7..];
    while let Ok((num, ys)) = parse_number(xs) {
        if xs.len() == 0 {
            break;
        }
        nums.push(num);
        xs = ys;
    }
    Ok(nums)
}

fn parse_range_list<B: BufRead>(it: &mut std::io::Lines<B>) -> io::Result<Vec<(u32, u32, u32)>> {
    let mut res = Vec::new(); 
    while let Some(line) = it.next() {
        let xs = line?.clone();
        if xs.len() == 0 {
            break;
        }
        let (fst, xs) = parse_number(&xs)?;
        let (snd, xs) = parse_number(&xs)?;
        let (thd, _) = parse_number(&xs)?;
        let val = (fst, snd, thd);
        res.push(val);
    }
    Ok(res)
}

type Ranges = Vec<(u32, u32, u32)>;

fn parse_game(file: &File) -> io::Result<(Vec<u32>, Ranges, Ranges, Ranges, Ranges, Ranges, Ranges, Ranges)> {
    let reader = BufReader::new(file);
    let mut it = reader.lines();
    let seeds = parse_seed_list(&mut it)?;
    it.next(); 
    it.next();
    let stos = parse_range_list(&mut it)?;
    it.next(); 
    let stof = parse_range_list(&mut it)?;
    it.next(); 
    let ftow = parse_range_list(&mut it)?;
    it.next(); 
    let wtol = parse_range_list(&mut it)?;
    it.next(); 
    let ltot = parse_range_list(&mut it)?;
    it.next(); 
    let ttoh = parse_range_list(&mut it)?;
    it.next(); 
    let htol = parse_range_list(&mut it)?;
    Ok((seeds, stos, stof, ftow, wtol, ltot, ttoh, htol))
}

fn map_over_ranges(inp: usize, ranges: &Ranges) -> usize {
    for (dest, src, len) in ranges {
        let dest = *dest as usize;
        let src = *src as usize;
        let len = *len as usize;
        if src <= inp && inp < src + len {
            let delta = inp - src;
            return dest + delta;
        }
    }
    inp
}

fn max_src_end_of_range(ranges: &Ranges) -> usize {
    ranges.iter().map(|(_, src, len)| {
        *src as usize + *len as usize 
    }).max().unwrap()
}

fn max_dest_end_of_range(ranges: &Ranges) -> usize {
    ranges.iter().map(|(dest, _, len)| {
        *dest as usize + *len as usize
    }).max().unwrap()
}

fn main() -> io::Result<()> {
    let file = File::open("src/data.txt")?;
    let (seeds, seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humid, humid_to_loc) = parse_game(&file)?;
    // println!("seeds {:?}", seeds);
    // println!("seed to soil {:?}", seed_to_soil);
    // println!("soil to fert {:?}", soil_to_fert);
    // println!("fert to water {:?}", fert_to_water);
    // println!("water to light {:?}", water_to_light);
    // println!("light to temp {:?}", light_to_temp);
    // println!("temp to humid {:?}", temp_to_humid);
    // println!("humid to location {:?}", humid_to_loc);


    let seed_to_soil_max   = max_src_end_of_range(&seed_to_soil);
    let soil_to_fert_max   = max_src_end_of_range(&soil_to_fert);
    let fert_to_water_max  = max_src_end_of_range(&fert_to_water);
    let water_to_light_max = max_src_end_of_range(&water_to_light);
    let light_to_temp_max  = max_src_end_of_range(&light_to_temp);
    let temp_to_humid_max  = max_src_end_of_range(&temp_to_humid);
    let humid_to_loc_max   = max_src_end_of_range(&humid_to_loc);
    
    let seed_to_soil_max_dest   = max_dest_end_of_range(&seed_to_soil);
    let soil_to_fert_max_dest   = max_dest_end_of_range(&soil_to_fert);
    let fert_to_water_max_dest  = max_dest_end_of_range(&fert_to_water);
    let water_to_light_max_dest = max_dest_end_of_range(&water_to_light);
    let light_to_temp_max_dest  = max_dest_end_of_range(&light_to_temp);
    let temp_to_humid_max_dest  = max_dest_end_of_range(&temp_to_humid);
    let humid_to_loc_max_dest   = max_dest_end_of_range(&humid_to_loc);


    let mut candidates = seeds.chunks(2).map(|seed| {
        let start = seed[0];
        let len = seed[1];
        let end = start + len;
        (start as usize, end as usize)
    }).collect::<Vec<(usize, usize)>>();

    candidates.sort_by(|(a, _), (b, _)| a.cmp(&b));

    let min = candidates.iter().map(|(start, end)| {
        let mut min = humid_to_loc_max_dest;
        
        for seed in *start..*end {
            let x = seed as usize;
            if x > seed_to_soil_max {
                break;
            }
            let x = map_over_ranges(x, &seed_to_soil);
            if x > soil_to_fert_max {
                break;
            }
            let x = map_over_ranges(x, &soil_to_fert);
            if x > fert_to_water_max {
                break;
            }
            let x = map_over_ranges(x, &fert_to_water);
            if x > water_to_light_max {
                break;
            }
            let x = map_over_ranges(x, &water_to_light);
            if x > light_to_temp_max {
                break;
            }
            let x = map_over_ranges(x, &light_to_temp);
            if x > temp_to_humid_max {
                break;
            }
            let x = map_over_ranges(x, &temp_to_humid);
            if x > humid_to_loc_max {
                break;
            }
            let x = map_over_ranges(x, &humid_to_loc);
            min = std::cmp::min(min, x);
        }
        println!("inner min {}", min);
        min
    }).min();

    println!("min is {:?}", min);

    Ok(())
}
