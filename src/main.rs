#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day17;
use day17 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
