#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day12;
use day12 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
