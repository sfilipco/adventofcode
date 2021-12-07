use std::fs;

use anyhow::{anyhow, Result};

const BSIZE: usize = 5;

struct Player {
    board: [[i32; BSIZE]; BSIZE],
    checked: [[bool; BSIZE]; BSIZE],
    line_checked: [i32; BSIZE],
    col_checked: [i32; BSIZE],
    finished: bool,
}

impl Player {
    fn new(board: [[i32; BSIZE]; BSIZE]) -> Self {
        Player {
            board,
            checked: [[false; BSIZE]; BSIZE],
            line_checked: [0i32; BSIZE],
            col_checked: [0i32; BSIZE],
            finished: false,
        }
    }

    fn mark(&mut self, number: i32) -> Option<i32> {
        for i in 0..BSIZE {
            for j in 0..BSIZE {
                if self.board[i][j] == number {
                    self.checked[i][j] = true;
                    self.line_checked[i] += 1;
                    self.col_checked[j] += 1;
                }
            }
        }

        for i in 0..BSIZE {
            if self.line_checked[i] == BSIZE as i32 {
                self.finished = true;
                return Some(self.sum_unchecked());
            }
            if self.col_checked[i] == BSIZE as i32 {
                self.finished = true;
                return Some(self.sum_unchecked());
            }
        }
        None
    }

    fn sum_unchecked(&self) -> i32 {
        let mut sum = 0;
        for i in 0..BSIZE {
            for j in 0..BSIZE {
                if self.checked[i][j] == false {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let mut lines = input.lines();
    let number_list = lines
        .next()
        .ok_or_else(|| anyhow!("number list not found"))?
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).map_err(|err| err.into()))
        .collect::<Result<Vec<_>>>()?;
    let mut players: Vec<Player> = vec![];
    while lines.next() != None {
        let mut board = [[032; BSIZE]; BSIZE];
        for i in 0..BSIZE {
            let mut line = lines
                .next()
                .ok_or_else(|| anyhow!("expecting {}th line", i))?
                .split_whitespace();
            for j in 0..BSIZE {
                board[i][j] = i32::from_str_radix(
                    line.next()
                        .ok_or_else(|| anyhow!("expecting {}th column", j))?,
                    10,
                )?;
            }
        }
        players.push(Player::new(board));
    }
    let mut last_score = 0;
    for number in number_list {
        for player in players.iter_mut() {
            if player.finished == false {
                if let Some(score) = player.mark(number) {
                    last_score = number * score;
                }
            }
        }
    }
    println!("{}", last_score);
    Ok(())
}
