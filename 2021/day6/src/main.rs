use std::fs;

use anyhow::Result;

const TIME_TO_HATCH: usize = 6;
const SPAWN_DELAY: usize = 2;
const MAX_DAYS: usize = TIME_TO_HATCH + SPAWN_DELAY + 1;

fn main() -> Result<()> {
    let mut state = [0i64; MAX_DAYS];
    for entry in fs::read_to_string("input.txt")?.split(',') {
        let itimer = usize::from_str_radix(entry.trim(), 10)?;
        state[itimer] += 1;
    }
    for _day in 0..256 {
        let spawning = state[0];
        for i in 1..MAX_DAYS {
            state[i - 1] = state[i];
        }
        state[TIME_TO_HATCH] += spawning;
        state[TIME_TO_HATCH + SPAWN_DELAY] = spawning;
    }
    let total: i64 = state.iter().sum();
    println!("{}", total);
    Ok(())
}
