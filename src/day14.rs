use color_eyre::eyre::{eyre, Result};
use std::{collections::HashMap, fs, hash::Hash};

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
        for (pair, ch) in self.insertions.iter() {
            let curr = self.pairs.get(pair).map_or(0, |v| *v);
            if curr > 0 {
                // println!("pair   {:?} (curr   {}) - inserting {}", pair, curr, ch);
                inserts.push((*pair, -1));
                // println!("pair   {:?} (change {})", pair, curr - 1);
                let new_pair1 = Pair(pair.0, *ch);
                let new_pair2 = Pair(*ch, pair.1);
                let curr = self.pairs.get(&new_pair1).map_or(0, |v| *v);
                // println!("pair 1 {:?} (change {})", new_pair1, curr + 1);
                inserts.push((new_pair1, 1));
                let curr = self.pairs.get(&new_pair2).map_or(0, |v| *v);
                inserts.push((new_pair2, 1));
                // println!("pair 2 {:?} (change {})", new_pair2, curr + 1);
            }
        }
        for (pair, count) in inserts {
            let curr = self.pairs.get(&pair).map_or(0, |v| *v);
            self.pairs.insert(pair, (curr as i32 + count) as u32);
        }
    }

    fn max_diff(&self) -> u64 {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for (pair, count) in self.pairs.iter() {
            println!("{:?} {}", pair, count);
            let curr = counts.get(&pair.0).map_or(0, |f| *f);
            counts.insert(pair.0, curr + *count as u64);
            println!(
                "{} = {} + {} = {}",
                pair.0,
                count,
                curr,
                curr + *count as u64
            );
            let curr = counts.get(&pair.1).map_or(0, |f| *f);
            counts.insert(pair.1, curr + *count as u64);
            println!(
                "{} = {} + {} = {}",
                pair.1,
                count,
                curr,
                curr + *count as u64
            );
        }
        let counts = counts
            .values()
            .map(|v| if v % 2 == 0 { v / 2 } else { v / 2 + 1 });
        let max = counts.clone().max().unwrap();
        let min = counts.clone().min().unwrap();
        max - min
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
    for _ in 0..10 {
        polymer.iteration();
    }
    // dbg!(&polymer.pairs); //2937
    dbg!(polymer.max_diff()); //2937

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
