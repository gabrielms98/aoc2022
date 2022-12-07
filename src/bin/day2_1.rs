use anyhow::Result;
use std::collections::HashMap;
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

        let plays_w = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
        let plays_l = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
        let points = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

        if plays_w[opponent] == me {
            score += 6 + points[me];
        } else if plays_l[opponent] == me {
            score += points[me];
        } else {
            score += 3 + points[me];
        }
    }

    println!("{}", score);

    Ok(())
}
