use std::collections::{HashMap, HashSet};
use std::hash;
use std::io::{prelude::*, BufReader, self};
use std::fs::File;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right
}

fn parse_directions(xs: &str) -> Result<Vec<Direction>, Box<dyn Error>> {
    let ds = xs.chars().map(|c| {
        match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a direction")))
        }
    }).collect::<Result<Vec<Direction>, Box<std::io::Error>>>()?;
    Ok(ds)
}

fn skip_ws(xs: &str) -> Result<&str, Box<dyn Error>> {
    let mut xs = xs;
    while xs.starts_with(' ') {
        xs = &xs[1..];
    }
    Ok(xs)
}

fn expect<'a>(xs: &'a str, pat: char) -> Result<&'a str, Box<dyn Error>> {
    if !xs.starts_with(pat) {
        println!("{:?}", xs);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "no match")));
    }
    Ok(&xs[1..])
}

fn parse_id(xs: &str) -> Result<(String, &str), Box<dyn Error>> {
    let r: String = xs
        .chars()
        .take_while(|c| !c.is_whitespace() && *c != ',' && *c != ')')
        .collect();
    Ok((r.clone(), &xs[r.len()..]))
}

fn parse_branch(xs: &str) -> Result<(String, String, String), Box<dyn Error>> {
    let (start, xs) = parse_id(xs)?;
    let xs = skip_ws(xs)?;
    let xs = expect(xs, '=')?;
    let xs = skip_ws(xs)?;
    let xs = expect(xs, '(')?;
    let (left, xs) = parse_id(xs)?;
    let xs = expect(xs, ',')?;
    let xs = skip_ws(xs)?;
    let (right, _) = parse_id(xs)?;
    Ok((start, left, right))
}

#[derive(Debug)]
struct GameInfo {
    node_map: HashMap<String, u32>,
    branch_map: HashMap<u32, (u32, u32)>,
    directions: Vec<Direction>,
    start_nodes: Vec<u32>,
    end_nodes: HashSet<u32>,

    aaa_id: u32,
    zzz_id: u32,
}

fn get_id(node_map: &mut HashMap<String, u32>, next_node_id: &mut u32, label: String) -> u32 {
    let id = node_map.entry(label.clone()).or_insert(*next_node_id);
    if *id == *next_node_id {
        *next_node_id += 1;
    }
    *id
}

fn is_start_node(label: &str) -> bool {
    if let Some(c) = label.chars().nth(2) {
        return c == 'A';
    }
    false
}

fn is_end_node(label: &str) -> bool {
    if let Some(c) = label.chars().nth(2) {
        return c == 'Z';
    }
    false
}

fn parse_game(fbuf: &String) -> Result<GameInfo, Box<dyn Error>> {
    let mut lit = fbuf.lines();
    let directions = parse_directions(lit.next().ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid input")))?)?;
    lit.next().ok_or(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid input")))?;
    
    let mut next_node_id = 0u32;
    let mut node_map: HashMap<String, u32> = HashMap::new();
    let mut branch_map: HashMap<u32, (u32, u32)> = HashMap::new();

    while let Some(line) = lit.next() {
        let (start, left, right) = parse_branch(line)?;
        let sid = get_id(&mut node_map, &mut next_node_id, start.clone());
        let lid = get_id(&mut node_map, &mut next_node_id, left);
        let rid = get_id(&mut node_map, &mut next_node_id, right);
        let rc = branch_map.insert(sid, (lid, rid));
        if rc != None {
            let err = std::io::Error::new(std::io::ErrorKind::InvalidInput, "duplicate key in branch map");
            return Err(Box::new(err));
        }
    }

    let mut start_nodes: Vec<u32> = Vec::new();
    let mut end_nodes: HashSet<u32> = HashSet::new();

    for (label, id) in node_map.iter() {
        if is_start_node(label) {
            start_nodes.push(*id);
        } else if is_end_node(label) {
            end_nodes.insert(*id);
        }
    }

    let aaa_id = *node_map.get(&String::from_str("AAA").unwrap()).unwrap();
    let zzz_id = *node_map.get(&String::from_str("ZZZ").unwrap()).unwrap();

    let g = GameInfo { node_map, branch_map, directions, start_nodes, end_nodes, aaa_id, zzz_id };
    Ok(g)
}

struct GameState<'a> {
    next_dir: usize,
    game: &'a GameInfo,
}

impl<'a> GameState<'a> {
    fn new(game: &'a GameInfo) -> Self {
        GameState { 
            next_dir: 0, 
            game
        }
    }

    fn incr_dir(&mut self) {
        self.next_dir = (self.next_dir + 1) % self.game.directions.len();
    }

    // fn is_done(&self) -> bool {
    //     self.state.iter().all(|id| self.game.end_nodes.contains(&id))
    // } 
    
    fn step(&mut self, cur: u32) -> u32 {
        let (left, right) = self.game.branch_map.get(&cur).unwrap();
        let dir = self.game.directions[self.next_dir];
        self.incr_dir();
        match dir {
            Direction::Left => *left,
            Direction::Right => *right,
        }
    }

    fn find_first_reachable_end_node(&mut self, sid: u32) -> (u32, usize) {
        self.next_dir = 0;
        let mut s = 0;
        let mut cur = sid;
        loop {
            cur = self.step(cur);
            s += 1;
            if self.game.end_nodes.contains(&cur) {
                break;
            }
        }
        (cur, s)
    }

    fn solve_for(&mut self, sid: u32, eid: u32) -> usize {
        self.next_dir = 0;
        let mut s = 0;
        let mut cur = sid;
        loop {
            cur = self.step(cur);
            s += 1;
            if cur == eid {
                break;
            }
        }
        s
    }
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

fn lcm(x: usize, y: usize) -> usize {
    let g = gcd(x, y);
    x * (y / g)
}

// This is a strange one, basically for task 2 there is an invariant where the
// cycle between end-node X to X has the same length as the length of the
// path of the (one) starting node Y that reaches X.
//
// .. maybe I don't get that this is implied in the task description but I found 
// it through (a lot of) trial and error..
//
// So, if the dist between Y and X is n, the cycle between X and X has also
// dist n.
//
// If X(i) is the first reachable end node for starting node Y(i) and n(i)
// the distance then the least amount of steps required to reach a state
// where all end nodes are reached at the same time is the least common 
// divisor of n(1)..n(k) for k starting nodes.
//
fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("src/data.txt")?;
    let mut fbuf = String::new();
    file.read_to_string(&mut fbuf)?;
    
    let gdescr = parse_game(&fbuf)?;

    let mut steps: Vec<usize> = Vec::new();
    let mut gstate = GameState::new(&gdescr);
    for sid in gstate.game.start_nodes.iter() {
        let (eid, s) = gstate.find_first_reachable_end_node(*sid);
        println!("first reachable end node for {} is {} after {} steps", sid, eid, s);
        steps.push(s);
    }

    let lcm = steps.iter().map(|x| *x).reduce(|acc, x| lcm(acc, x)).unwrap();
    println!("LCM = {:?}", lcm);

    for s in steps {
        println!("{} / {} = {}", lcm, s, lcm / s);
    }

    // for sid in gstate.game.end_nodes.iter() {
    //     let steps = gstate.solve_for(*sid, *sid);
    //     println!("cycle of {} is {} steps long", sid, steps);
    // }

    Ok(())
}
