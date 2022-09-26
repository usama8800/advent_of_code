#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day15;
use day15 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
