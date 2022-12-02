use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("assets/day1/input2.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut biggest = 0;
    let mut sum = 0;

    for line in reader.lines() {
        let value = line.unwrap();

        if value == "" {
            if sum > biggest {
                biggest = sum;
            }

            sum = 0;
        } else {
            sum += value.parse::<u32>().unwrap();
        }
    }

    println!("{}", biggest);
}
