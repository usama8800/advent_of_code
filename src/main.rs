#![allow(dead_code, unused)]

use color_eyre::eyre::Result;

mod day16;
use day16 as day;

fn main() -> Result<()> {
    color_eyre::install()?;

    day::solve()
}
