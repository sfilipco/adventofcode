use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let mut depth = 0;
    let mut aim = 0;
    let mut forward = 0;

    for line in fs::read_to_string("input.txt")?.lines() {
        let mut iter = line.split(" ");
        let command = iter.next().ok_or_else(|| anyhow!("command not found"))?;
        let units_str = iter.next().ok_or_else(|| anyhow!("units not found"))?;
        let units = i64::from_str_radix(units_str, 10)?;
        match command {
            "forward" => {
                forward = forward + units;
                depth = i64::max(0, depth + aim * units);
            }
            "down" => aim = aim + units,
            "up" => aim = aim - units,
            _ => return Err(anyhow!("unknown command")),
        }
    }

    println!("{}", depth * forward);
    Ok(())
}
