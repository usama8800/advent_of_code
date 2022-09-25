use color_eyre::eyre::{eyre, Result};
use std::collections::HashMap;
use std::fs;

fn get_input() -> Result<Vec<u64>> {
    let contents = fs::read_to_string("inputs/day6.txt").expect("");
    let mut lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });
    let line = lines.next().ok_or_else(|| eyre!(""))?.split(',');
    Ok(line.filter_map(|v| v.parse().ok()).collect())
}

fn iteration(fishmap: &mut HashMap<u64, u64>) {
    let mut new6: u64 = *fishmap.get(&0).map_or(&0, |v| v);
    for key in 1..=8u64 {
        let val = *fishmap.get(&key).map_or(&0, |v| v);
        fishmap.insert(key - 1, val);
    }
    let old6: u64 = *fishmap.get(&6).map_or(&0, |v| v);
    fishmap.insert(6, new6 + old6);
    fishmap.insert(8, new6);
}

fn solve_p1() -> Result<()> {
    let input = get_input()?;
    let mut fishmap: HashMap<u64, u64> = HashMap::new();
    for num in input {
        let curr = fishmap.get(&num).map_or(0, |v| *v);
        fishmap.insert(num, curr + 1);
    }

    // println!("{:?}", &fishmap);
    // for _ in 0..80 {
    for _ in 0..256 {
        iteration(&mut fishmap);
        // println!("{:?}", &fishmap);
    }
    dbg!(fishmap.values().sum::<u64>());

    Ok(())
}

fn solve_p2() -> Result<()> {
    let input = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    // solve_p2();

    Ok(())
}
