#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day8;
use day8 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
