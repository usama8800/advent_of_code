use color_eyre::eyre::{eyre, Result};
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Cave {
    is_big: bool,
    visited: bool,
    connections: Vec<String>,
}

fn get_input() -> Result<HashMap<String, Cave>> {
    let contents = fs::read_to_string("inputs/day12.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let mut ret = HashMap::new();
    for line in lines {
        let splits: Vec<&str> = line.split('-').collect();
        for i in 0..2 {
            if let Some(curr) = ret.get(splits[1 - i]) {
                curr.
            } else {
                ret.insert(
                    splits[1 - 0].to_owned(),
                    Cave {
                        connections: Vec::from([splits[1 - i].to_owned()]),
                        is_big: splits[1 - i].chars().next().unwrap().is_uppercase(),
                        visited: false,
                    },
                );
            }
        }
    }

    Ok(ret)
}

fn solve_p1() -> Result<()> {
    let input = get_input()?;
    dbg!(input);

    Ok(())
}

pub fn solve_p2() -> Result<()> {
    let input = get_input()?;

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
