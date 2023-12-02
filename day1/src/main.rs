use std::io::{BufReader, prelude::*};
use std::io;
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file); 

    let sum = reader.lines().map(|line| -> io::Result<u32> {
        let xs = line?.clone();

        // need to consider the case where two numbers are "interleaved", 
        // for example replacing twone with 2ne would be wrong! Correct is 21
        let xs = xs
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "th3ree")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "s6ix")
            .replace("seven", "se7ven")
            .replace("eight", "eig8ht")
            .replace("nine", "ni9ne");
        
        // solution 1..
        let ds: Vec<u32> = xs
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        Ok(ds[0] * 10 + ds[ds.len() - 1])
    }).try_fold(0u32, |acc, x| -> io::Result<u32> {
        Ok(acc + x?)
    })?;
    println!("solution is {}", sum);
    Ok(())
}
