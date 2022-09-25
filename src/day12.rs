use color_eyre::eyre::{eyre, Result};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Cave {
    is_big: bool,
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

    let mut ret: HashMap<String, Cave> = HashMap::new();
    for line in lines {
        let splits: Vec<&str> = line.split('-').collect();
        for curr in 0..2 {
            let other: usize = if curr == 0 { 1 } else { 0 };
            if let Some(existing) = ret.get_mut(splits[curr]) {
                existing.connections.push(splits[other].to_owned());
            } else {
                ret.insert(
                    splits[curr].to_owned(),
                    Cave {
                        connections: Vec::from([splits[other].to_owned()]),
                        is_big: splits[curr].chars().next().unwrap().is_uppercase(),
                    },
                );
            }
        }
    }

    Ok(ret)
}

fn solve_p1() -> Result<()> {
    let mut caves = get_input()?;

    let mut stack: Vec<(String, Vec<String>)> = Vec::from([("start".to_owned(), Vec::new())]);
    let mut ends = 0;
    loop {
        if let Some((mut visiting_str, mut visited)) = stack.pop() {
            // dbg!(&visiting_str);
            let visiting = caves.get(&visiting_str).unwrap();
            visited.push(visiting_str.clone());

            if visiting_str.as_str() == "end" {
                // dbg!(&visited);
                ends += 1;
                continue;
            }

            for connection_str in visiting.connections.iter() {
                let connection = caves.get(connection_str).unwrap();
                if visited.contains(connection_str) && !connection.is_big {
                    continue;
                }
                stack.push((connection_str.clone(), visited.clone()));
            }
        } else {
            break;
        }
    }
    dbg!(ends);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let mut caves = get_input()?;

    let mut stack: Vec<(String, Option<String>, Vec<String>)> =
        Vec::from([("start".to_owned(), None, Vec::new())]);
    let mut ends = 0;
    loop {
        if let Some((mut visiting_str, doubled_option, mut visited)) = stack.pop() {
            // dbg!(&visiting_str);
            let visiting = caves.get(&visiting_str).unwrap();

            let doubled = if visited.contains(&visiting_str) && !visiting.is_big {
                Some(&visiting_str)
            } else {
                if let Some(ref old) = doubled_option {
                    Some(old)
                } else {
                    None
                }
            };
            visited.push(visiting_str.clone());

            if visiting_str.as_str() == "end" {
                // dbg!(&visited);
                ends += 1;
                continue;
            }

            for connection_str in visiting.connections.iter() {
                if connection_str == "start" {
                    continue;
                }
                // dbg!(connection_str);
                let connection = caves.get(connection_str).unwrap();
                if let Some(doubled_str) = doubled {
                    if visited.contains(connection_str) && !connection.is_big {
                        continue;
                    }
                    stack.push((
                        connection_str.clone(),
                        Some(doubled_str.to_owned()),
                        visited.clone(),
                    ));
                } else {
                    stack.push((connection_str.clone(), None, visited.clone()));
                }
            }
        } else {
            break;
        }
    }
    dbg!(ends);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
