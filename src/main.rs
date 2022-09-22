#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day11;
use day11 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
