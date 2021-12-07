use std::fs;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let positions = fs::read_to_string("input.txt")?
        .split(',')
        .map(|x| i32::from_str_radix(x.trim(), 10).map_err(|err| err.into()))
        .collect::<Result<Vec<_>>>()?;
    let min_pos: i32 = *positions
        .iter()
        .min()
        .ok_or_else(|| anyhow!("not empty positions"))?;
    let max_pos: i32 = *positions
        .iter()
        .max()
        .ok_or_else(|| anyhow!("not empty positions"))?;
    let min_cost: i32 = (min_pos..max_pos)
        .map(|pos| {
            positions
                .iter()
                .map(|x| (*x - pos).abs())
                .map(|dist| dist * (dist + 1) / 2)
                .sum()
        })
        .min()
        .ok_or_else(|| anyhow!("multiple positions considered"))?;
    println!("{}", min_cost);
    Ok(())
}
