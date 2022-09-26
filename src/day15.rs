use color_eyre::eyre::{eyre, Result};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq)]
struct State<'a> {
    cost: u32,
    node: &'a Node,
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    val: u8,
    position: Point,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_pos = self.position.0 + self.position.1;
        let other_pos = other.position.0 + other.position.1;
        other_pos.cmp(&self_pos)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Grid {
    nodes: HashMap<Point, Node>,
}

impl Grid {
    fn a_star(&self) -> Option<u32> {
        let start = self.nodes.get(&Point(0, 0)).unwrap();
        let mut costs: HashMap<Point, _> =
            HashMap::from_iter(self.nodes.iter().map(|(point, node)| (*point, u32::MAX)));
        costs.insert(start.position, 0);
        let mut min_heap = BinaryHeap::new();
        min_heap.push(State {
            cost: 0,
            node: start,
        });

        while let Some(State {
            cost,
            node: Node { val, position },
        }) = min_heap.pop()
        {
            if *costs.get(&position).unwrap() < cost {
                continue;
            }
            // dbg!(val, cost, position);
            let Point(x, y) = *position;

            let adjacents = Vec::from([
                if y == 0 {
                    None
                } else {
                    self.nodes.get(&Point(x, y - 1))
                },
                self.nodes.get(&Point(x, y + 1)),
                if x == 0 {
                    None
                } else {
                    self.nodes.get(&Point(x - 1, y))
                },
                self.nodes.get(&Point(x + 1, y)),
            ]);

            if adjacents[1] == None && adjacents[3] == None {
                return Some(cost);
            }

            for (i, adjacent) in adjacents.iter().enumerate() {
                if let Some(adjacent) = *adjacent {
                    let next = State {
                        cost: (
                            cost as i32
                            // + match i {
                            //     0 | 2 => -1 as i32,
                            //     1 | 3 => 1 as i32,
                            //     _ => unreachable!(),
                            // }
                        ) as u32
                            + adjacent.val as u32,
                        node: adjacent,
                    };
                    if next.cost < *costs.get(&next.node.position).unwrap() {
                        // dbg!(&next);
                        costs.insert(next.node.position, next.cost);
                        min_heap.push(next);
                    }
                }
            }
        }

        None
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut points: Vec<_> = self.nodes.keys().collect();
        points.sort();
        let mut prev_x = 0;
        for point in points {
            if point.0 > prev_x {
                writeln!(f);
                prev_x = point.0;
            }
            write!(f, "{}", self.nodes.get(point).unwrap().val);
        }
        Ok(())
    }
}

fn get_input() -> Result<Grid> {
    let contents = fs::read_to_string("inputs/day15.txt").expect("");
    let lines = contents.split('\n').filter_map(|v| {
        if v.len() == 0 || v.chars().nth(0) == Some('#') {
            None
        } else {
            Some(v)
        }
    });

    let mut nodes = HashMap::new();
    for (i, line) in lines.enumerate() {
        // let mut row = Vec::new();
        for (j, ch) in line.chars().enumerate() {
            // row.push(ch as u8 - 0x30);
            // nodes.push(Node {
            //     val: ch as u8 - 0x30,
            //     position: Point(i, j),
            // })
            nodes.insert(
                Point(i, j),
                Node {
                    position: Point(i, j),
                    val: ch as u8 - 0x30,
                },
            );
        }
        // nodes.push(row);
    }

    Ok(Grid { nodes })
}

fn solve_p1() -> Result<()> {
    let grid = get_input()?;
    let ans = grid.a_star();
    dbg!(ans);

    Ok(())
}

fn solve_p2() -> Result<()> {
    let small_grid = get_input()?;
    let original_nodes = small_grid.nodes;
    let mut new_nodes =
        HashMap::from_iter(original_nodes.iter().map(|(point, node)| (*point, *node)));

    let Point(x, ..) = original_nodes.keys().max().unwrap();
    let len = x + 1;

    for i in 1..25 {
        let row = i / 5;
        let col = i % 5;
        for (Point(x, y), Node { val, .. }) in original_nodes.iter() {
            let point = Point(*x + len * col, *y + len * row);
            let mut val = *val + row as u8 + col as u8;
            while val > 9 {
                val -= 9;
            }
            new_nodes.insert(
                point,
                Node {
                    position: point,
                    val,
                },
            );
        }
    }

    let grid = Grid { nodes: new_nodes };
    // println!("{}", &grid);

    let ans = grid.a_star();
    dbg!(ans);

    Ok(())
}

pub fn solve() -> Result<()> {
    solve_p1();
    solve_p2();

    Ok(())
}
