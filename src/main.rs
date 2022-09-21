#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day5;
use day5 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
