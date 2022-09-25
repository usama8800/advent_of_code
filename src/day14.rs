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
    pairs: HashMap<Pair, u64>,
    insertions: HashMap<Pair, char>,
    start: char,
    end: char,
}

impl Polymer {
    fn iteration(&mut self) {
        let mut inserts: Vec<(Pair, i64)> = Vec::new();

        for (pair, count) in &self.pairs {
            if let Some(x) = self.insertions.get(pair) {
                let new_pair1 = Pair(pair.0, *x);
                let new_pair2 = Pair(*x, pair.1);
                // println!(
                //     "{} {}{} to {}{}{}",
                //     count, pair.0, pair.1, pair.0, x, pair.1
                // );
                inserts.push((*pair, -(*count as i64)));
                inserts.push((new_pair1, *count as i64));
                inserts.push((new_pair2, *count as i64));
            }
        }

        for (pair, count) in inserts {
            let curr = self.pairs.get(&pair).map_or(0, |v| *v);
            self.pairs.insert(pair, (curr as i64 + count) as u64);
        }
    }

    fn counts(&self) -> HashMap<char, u64> {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            let curr = counts.get(&pair.0).map_or(0, |f| *f);
            counts.insert(pair.0, curr + *count as u64);
            let curr = counts.get(&pair.1).map_or(0, |f| *f);
            counts.insert(pair.1, curr + *count as u64);
        }
        for (ch, count) in counts.iter_mut() {
            *count = *count / 2u64;
            if self.start == *ch || self.end == *ch {
                *count += 1;
            }
        }
        counts
    }

    fn max_diff(&self) -> u64 {
        let counts = self.counts();
        let max = counts.values().max().unwrap();
        let min = counts.values().min().unwrap();
        // dbg!(max);
        // dbg!(min);
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

    let mut pairs: HashMap<Pair, u64> = HashMap::new();
    let line = lines.next().unwrap();
    let start = line.chars().nth(0).unwrap();
    let end = line.chars().rev().nth(0).unwrap();
    for chars in line.chars().zip(line.chars().skip(1)) {
        let pair = Pair(chars.0, chars.1);
        let prev = pairs.get(&pair).unwrap_or(&0);
        pairs.insert(pair, prev + 1);
    }

    let mut insertions: HashMap<Pair, char> = HashMap::new();
    lines.for_each(|line| {
        let splits = line.split(" -> ").collect::<Vec<&str>>();
        let chars = splits[0].chars().collect::<Vec<char>>();
        insertions.insert(Pair(chars[0], chars[1]), splits[1].chars().next().unwrap());
    });

    Ok(Polymer {
        pairs,
        insertions,
        start,
        end,
    })
}

fn solve_p1() -> Result<()> {
    let mut polymer = get_input()?;
    // println!("{}", polymer);
    for i in 0..10 {
        polymer.iteration();
        // println!("{}", polymer);
    }
    let counts = polymer.counts();
    // dbg!(&counts);
    let sum: u64 = counts.values().sum();
    // dbg!(sum);
    dbg!(polymer.max_diff());

    Ok(())
}

fn solve_p2() -> Result<()> {
    let mut polymer = get_input()?;
    for i in 0..40 {
        polymer.iteration();
    }
    let counts = polymer.counts();
    // dbg!(&counts);
    let sum: u64 = counts.values().sum();
    // dbg!(sum);
    dbg!(polymer.max_diff());

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
