use color_eyre::eyre::{eyre, Result};
use std::{collections::HashMap, fmt::Display, fs, hash::Hash};

static mut DEBUG: bool = false;

fn debug(s: String) {
    unsafe {
        if DEBUG {
            println!("{}", s);
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pair(char, char);

#[derive(Debug)]
struct Polymer {
    pairs: HashMap<Pair, u32>,
    insertions: HashMap<Pair, char>,
}

impl Polymer {
    fn iteration(&mut self) {
        let mut inserts: Vec<(Pair, i32)> = Vec::new();

        debug("".to_owned());
        for (pair, count) in inserts {
            let curr = self.pairs.get(&pair).map_or(0, |v| *v);
            debug(format!("{:?} (new count  {})", pair, (curr as i32 + count)));
            self.pairs.insert(pair, (curr as i32 + count) as u32);
        }
    }

    fn counts(&self) -> HashMap<char, u64> {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            // println!("{:?} {}", pair, count);
            let curr = counts.get(&pair.0).map_or(0, |f| *f);
            counts.insert(pair.0, curr + *count as u64);
            // println!(
            //     "{} = {} + {} = {}",
            //     pair.0,
            //     count,
            //     curr,
            //     curr + *count as u64
            // );
            let curr = counts.get(&pair.1).map_or(0, |f| *f);
            counts.insert(pair.1, curr + *count as u64);
            // println!(
            //     "{} = {} + {} = {}",
            //     pair.1,
            //     count,
            //     curr,
            //     curr + *count as u64
            // );
        }
        for (ch, count) in counts.iter_mut() {
            *count = if *count % 2 == 0 {
                *count / 2u64
            } else {
                *count / 2u64 + 1
            };
        }
        counts
    }

    fn max_diff(&self) -> u64 {
        let counts = self.counts();
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        max - min
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pair, count) in self.pairs.iter() {
            if *count == 0 {
                continue;
            }
            writeln!(f, "{:?} = {}", pair, count);
        }
        Ok(())
    }
}

fn get_input() -> Result<Polymer> {
    let contents = fs::read_to_string("inputs/day14.txt").expect("");
    let mut lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let mut pairs: HashMap<Pair, u32> = HashMap::new();
    let line = lines.next().unwrap();
    for chars in line.chars().zip(line.chars().skip(1)) {
        pairs.insert(Pair(chars.0, chars.1), 1);
    }

    let mut insertions: HashMap<Pair, char> = HashMap::new();
    lines.for_each(|line| {
        let splits = line.split(" -> ").collect::<Vec<&str>>();
        let chars = splits[0].chars().collect::<Vec<char>>();
        insertions.insert(Pair(chars[0], chars[1]), splits[1].chars().next().unwrap());
    });

    Ok(Polymer { pairs, insertions })
}

fn solve_p1() -> Result<()> {
    let mut polymer = get_input()?;
    for i in 0..3 {
        if i == 2 {
            dbg!(polymer.counts());
            println!("{}", polymer);
            unsafe {
                DEBUG = true;
            }
        }
        polymer.iteration();
    }
    dbg!(polymer.counts());
    dbg!(polymer.max_diff());

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let mut polymer = get_input()?;
    for _ in 0..40 {
        polymer.iteration();
    }
    dbg!(polymer.max_diff());

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    // solve_p2();

    Ok(())
}
