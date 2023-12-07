use std::io::{prelude::*, BufReader};
use std::fs::File;

struct CardCnt {
    buckets: [u32; 13], // count occurs of cards
    bytepatt: u32, // count face value of hand
}

fn symidx(sym: char) -> u32 {
    match sym {
        'A' => 12, 'K' => 11, 'Q' => 10, 'T' => 9,
        '9' => 8, '8' => 7, '7' => 6, '6' => 5, '5' => 4,
        '4' => 3, '3' => 2, '2' => 1, 'J' => 0,
        _ => panic!("symbol does not describe a valid card!")
    }
}

impl CardCnt {
    fn new(hand: &str) -> Self {
        let mut s = Self { buckets: [0; 13], bytepatt: 0 };
        for c in hand.chars() {
            let v = symidx(c);
            s.buckets[v as usize] += 1;
            s.bytepatt |= v;
            s.bytepatt <<= 4;
        }
        s
    }

    fn count_jokers(&self) -> u32 {
        self.buckets[0]
    }

    fn is_five_of_a_kind(&self) -> bool {
        let j = self.count_jokers();
        let fives = self.buckets[0..].iter().filter(|&&b| b + j == 5).count();
        fives > 0
    }

    fn is_four_of_a_kind(&self) -> bool {
        let j = self.count_jokers();
        let fours = self.buckets[1..].iter().filter(|&&b| b + j == 4).count();
        fours > 0
    }

    fn is_full_house(&self) -> bool {
        let j = self.count_jokers();
        let threes = self.buckets[1..].iter().filter(|&&b| b + j == 3).count();
        let twos = self.buckets[1..].iter().filter(|&&b| b == 2).count();

        (threes == 1) && (twos == 1) && (j == 0) || (threes == 2) && (j == 1)
    }

    fn is_three_of_a_kind(&self) -> bool {
        let j = self.count_jokers();
        let threes = self.buckets[1..].iter().filter(|&&b| b + j == 3).count();
        threes > 0
    }
    
    // x, y, z, w, J
    fn is_two_pairs(&self) -> bool {
        let j = self.count_jokers();
        let twos = self.buckets[1..].iter().filter(|&&b| b == 2).count();
        // other cases are irrelevant, with two of a kind and one joker a three of a kind or 
        // a full house can be made and with two jokers a four of a kind can be made, all of 
        // them are better choices
        (twos == 2) && (j == 0)
    }

    fn is_one_pair(&self) -> bool {
        let j = self.count_jokers();
        let twos = self.buckets[1..].iter().filter(|&&b| b + j == 2).count();
        // again, all other cases yield better hands
        (twos == 1) && (j == 0) || (twos == 4) && (j == 1)
    }

    fn value(&self) -> u32 {
        let mut value = 0;
        if self.is_five_of_a_kind() {
            value += 0x6000000;
        } else if self.is_four_of_a_kind() {
            value += 0x5000000;
        } else if self.is_full_house() {
            value += 0x4000000;
        } else if self.is_three_of_a_kind() {
            value += 0x3000000;
        } else if self.is_two_pairs() {
            value += 0x2000000;
        } else if self.is_one_pair() {
            value += 0x1000000;
        }
    
        // max value for bytepatt is ccccc0 = 13421760
        value + self.bytepatt
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file);

    let mut store: Vec<(u32, u32)> = reader
        .lines()
        .map(|line| -> Result<(u32, u32), Box<dyn std::error::Error>> {
            let line = line?;
            let parts: Vec<&str> = line.split(' ').collect();
            let hand = parts[0];
            let bet = parts[1].parse::<u32>()?;
            let value = CardCnt::new(hand).value();
            Ok((value, bet))
        }).collect::<Result<Vec<(u32, u32)>, Box<dyn std::error::Error>>>()?;
        
    store.sort_unstable_by(|x, y| {
        let (xv, _) = x;
        let (yv, _) = y;
        xv.cmp(yv)
    });

    let score = store.iter().enumerate().map(|(i, x)| {
        let (_, bet) = x;
        (i as u32 + 1) * bet
    }).sum::<u32>();

    println!("score is {}", score);
    
    Ok(())
}
