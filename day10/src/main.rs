use std::io::{prelude::*, BufReader, self};
use std::fs::File;
use std::os::unix::raw::uid_t;
use std::path::PrefixComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum TileType {
    VerticalSegment = b'|',
    HorizontalSegment = b'-',
    NorthEastBendSegment = b'L',
    NorthWestBendSegment = b'J',
    SouthWestBendSegment = b'7',
    SouthEastBendSegment = b'F',
    StartSegment = b'S',
    Ground = b'.',
}

impl Into<char> for TileType {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl TryInto<TileType> for char {
    type Error = std::io::Error;
    fn try_into(self) -> Result<TileType, Self::Error> {
        match self as u8 {
            b'|' => Ok(TileType::VerticalSegment),
            b'-' => Ok(TileType::HorizontalSegment),
            b'L' => Ok(TileType::NorthEastBendSegment),
            b'J' => Ok(TileType::NorthWestBendSegment),
            b'7' => Ok(TileType::SouthWestBendSegment),
            b'F' => Ok(TileType::SouthEastBendSegment),
            b'S' => Ok(TileType::StartSegment),
            b'.' => Ok(TileType::Ground),
            _ => Err(Self::Error::new(io::ErrorKind::InvalidInput, "char is not a tile type")),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<TileType>>,
    start: (usize, usize),
}

impl Map {
    fn parse(buf: &str) -> Result<Self, std::io::Error> {
        let mut map: Vec<Vec<TileType>> = Vec::new();
        for line in buf.lines() {
            let tiles = line
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<TileType>, std::io::Error>>()?;
            map.push(tiles);
        }
        let mut start: Option<(usize, usize)> = None;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == TileType::StartSegment {
                    start = Some((i, j));
                }
            }
        }
        let start = start
            .ok_or(std::io::Error::new(io::ErrorKind::InvalidData, "no start tile found"))?;
        Ok(Self { tiles: map, start })
    }

    fn expand_horizontal(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        
        if j > 0 { 
            let t = self.tiles[i][j - 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthEastBendSegment || t == TileType::SouthEastBendSegment {
                res.push((i, j - 1)); 
            }
        }
        if j < self.tiles[0].len() - 1 { 
            let t = self.tiles[i][j + 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthWestBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i, j + 1)); 
            }
        }
        res
    }
    
    fn expand_vertical(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        if i > 0 { 
            let t = self.tiles[i - 1][j];
            if t == TileType::VerticalSegment || t == TileType::SouthEastBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i - 1, j)); 
            }
        } 
        if i < self.tiles.len() - 1 { 
            let t = self.tiles[i + 1][j];
            if t == TileType::VerticalSegment || t == TileType::NorthEastBendSegment || t == TileType::NorthWestBendSegment {
                res.push((i + 1, j)); 
            }
        }
        res
    }

    fn expand_northeast(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        if i > 0 { 
            let t = self.tiles[i - 1][j];
            if t == TileType::VerticalSegment || t == TileType::SouthEastBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i - 1, j)); 
            }
        } 
        if j < self.tiles[0].len() - 1 { 
            let t = self.tiles[i][j + 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthWestBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i, j + 1)); 
            }
        }
        res
    }
    
    fn expand_northwest(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        if i > 0 { 
            let t = self.tiles[i - 1][j];
            if t == TileType::VerticalSegment || t == TileType::SouthEastBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i - 1, j)); 
            }
        } 
        if j > 0 { 
            let t = self.tiles[i][j - 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthEastBendSegment || t == TileType::SouthEastBendSegment {
                res.push((i, j - 1))
            }
        };
        res
    }

    fn expand_southeast(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        if i < self.tiles.len() - 1 { 
            let t = self.tiles[i + 1][j];
            if t == TileType::VerticalSegment || t == TileType::NorthEastBendSegment || t == TileType::NorthWestBendSegment {
                res.push((i + 1, j)); 
            }
        } 
        if j < self.tiles[0].len() - 1 { 
            let t = self.tiles[i][j + 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthWestBendSegment || t == TileType::SouthWestBendSegment {
                res.push((i, j + 1)); 
            }
        }
        res
    }
    
    fn expand_southhwest(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let mut res = Vec::new();
        if i < self.tiles.len() - 1 { 
            let t = self.tiles[i + 1][j];
            if t == TileType::VerticalSegment || t == TileType::NorthEastBendSegment || t == TileType::NorthWestBendSegment {
                res.push((i + 1, j)); 
            }
        } 
        if j > 0 { 
            let t = self.tiles[i][j - 1];
            if t == TileType::HorizontalSegment || t == TileType::NorthEastBendSegment || t == TileType::SouthEastBendSegment {
                res.push((i, j - 1))
            }
        };
        res
    }

    fn expand_start(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut h = self.expand_horizontal(pos);
        let mut v = self.expand_vertical(pos);
        let mut ne = self.expand_northeast(pos);
        let mut se = self.expand_southeast(pos);
        let mut nw = self.expand_northwest(pos);
        let mut sw = self.expand_southhwest(pos);
        h.append(&mut v);
        h.append(&mut ne);
        h.append(&mut se);
        h.append(&mut nw);
        h.append(&mut sw);
        h
    }

    fn expand(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = pos;
        let tile = self.tiles[i][j];
        let frontier = match tile {
            TileType::Ground => Vec::new(),
            TileType::HorizontalSegment => self.expand_horizontal(pos),
            TileType::NorthEastBendSegment => self.expand_northeast(pos),
            TileType::NorthWestBendSegment => self.expand_northwest(pos),
            TileType::SouthEastBendSegment => self.expand_southeast(pos),
            TileType::SouthWestBendSegment => self.expand_southhwest(pos),
            TileType::StartSegment => self.expand_start(pos),
            TileType::VerticalSegment => self.expand_vertical(pos)
        };
        frontier
    }

    fn init_mask(&self) -> Vec<Vec<bool>> {
        let mut mask = Vec::new();
        for row in &self.tiles {
            mask.push(vec![false; row.len()]);
        } 
        let (i, j) = self.start;
        mask[i][j] = true;
        mask
    }

    fn bfs(&self) -> Vec<Vec<bool>> {
        let mut visited = self.init_mask();
        let mut frontier: Vec<(usize, usize)> = vec![self.start];
        let mut iterations = 0;
        loop {
            let mut expanse: Vec<(usize, usize)> = Vec::new();
            for pos in frontier {
                let candidates = self.expand(pos);
                for pos in candidates {
                    if visited[pos.0][pos.1] == false {
                        expanse.push(pos);
                        visited[pos.0][pos.1] = true;
                    }
                }
            }
            if expanse.len() == 0 {
                break;
            }
            iterations += 1;
            frontier = expanse;
        } 
        // self.print_dist(&visited);
        // println!("solution (1) is {}", iterations);
        visited
    }
}

fn scale_up(pipe_map: &Vec<Vec<bool>>, tilemap: &Map) -> Vec<Vec<bool>> {
    let n = pipe_map.len();
    let m = pipe_map[0].len();
    let mut scaled_map = vec![vec![false; 2 * m]; 2 * n];
    for i in 0..n {
        for j in 0..m {
            if !pipe_map[i][j] {
                continue;
            }
            let si = 2 * i;
            let sj = 2 * j;
            
            scaled_map[si][sj] = true;
            let offs = tilemap.expand((i, j));
            for (ni, nj) in offs {
                let di: i64 = ni as i64 - i as i64;
                let dj: i64 = nj as i64 - j as i64;
                scaled_map[(si as i64 + di) as usize][(sj as i64 + dj) as usize] = true;
            }
        }
    }
    scaled_map
}

fn flood_fill(scaled_map: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
    let n = scaled_map.len();
    let m = scaled_map[0].len();

    let mut frontier = vec![pos];
    loop {
        let mut expanse = Vec::new();     

        for (i, j) in &frontier {
            let i = *i;
            let j = *j;
            scaled_map[i][j] = true;
            if i > 0 && !scaled_map[i - 1][j] {
                scaled_map[i - 1][j] = true;
                expanse.push((i - 1, j));
            }
            if i < n - 1 && !scaled_map[i + 1][j] {
                scaled_map[i + 1][j] = true;
                expanse.push((i + 1, j));
            }
            if j > 0 && !scaled_map[i][j - 1] {
                scaled_map[i][j - 1] = true;
                expanse.push((i, j - 1));
            }
            if j < m - 1 && !scaled_map[i][j + 1] {
                scaled_map[i][j + 1] = true;
                expanse.push((i, j + 1));
            }
        }
        if expanse.len() == 0 {
            break;
        }
        frontier = expanse;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;

    let map = Map::parse(&fbuf)?;
    let pipe_map = map.bfs();

    let mut scaled_map = scale_up(&pipe_map, &map);
    let mut flood_start: Vec<(usize, usize)> = Vec::new();
    for i in 0..scaled_map.len() {
        if !scaled_map[i][0] {
            flood_start.push((i, 0));
        }
        let j = scaled_map[0].len() - 1;
        if !scaled_map[i][j] {
            flood_start.push((i, j));
        }
    }
    for j in 0..scaled_map[0].len() {
        if !scaled_map[0][j] {
            flood_start.push((0, j));
        }
        let i = scaled_map.len() - 1;
        if !scaled_map[i][j] {
            flood_start.push((i, j));
        }
    }

    for pos in flood_start {
        if !scaled_map[pos.0][pos.1] {
            println!("start flood fill from {:?}", pos);
            flood_fill(&mut scaled_map, pos);
        }
    }

    // evry point that is marked in scaled_map after the flood is 
    // guaranteed to be outside of the loop, to "scale down" just 
    // check for any point in the normal sized map if its pendant 
    // in the scaled map is outside of the loop. What's left is 
    // inside.
      
    let mut cnt = 0;
    for i in 0..pipe_map.len() {
        for j in 0..pipe_map[0].len() {
            if !scaled_map[2 * i][2 * j] {
                cnt += 1;
            }
        }
    }
    println!("cnt is {}", cnt);

    Ok(())
}
