#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day13;
use day13 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
