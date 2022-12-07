use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

// A|X  => ROCK(1), B|Y => Paper(2), C|Z => Scissors(3)
// Score = sum(chosen + (0 || 3 || 6))

fn main() -> Result<()> {
    let file = File::open("assets/day2/input2.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        let values = value.split(" ").collect::<Vec<&str>>();

        let opponent = *values.first().unwrap();
        let me = *values.last().unwrap();

        let plays = vec!["A", "B", "C"];

        let i = plays.iter().position(|&x| x == opponent).unwrap();

        if me == "Y" { // draw
            score += i + 3 + 1;
        } else if me == "Z" { // win
            score += (i + 1) % 3 + 6 + 1;
        } else if me == "X" { // lose
            score += (i + 3 - 1) % 3 + 1; // wrap around on iterating to previous = (idx + len - 1) % len
        }
    }

    println!("{}", score);

    Ok(())
}
