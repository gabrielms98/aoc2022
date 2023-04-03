use std::fs::File;
use std::io::{BufReader, BufRead};

fn main1() {
    let file = File::open("assets/day1/input2.txt").expect("Could not open file");
    let reader = BufReader::new(file);

    let mut sum = 0;

    let mut elfs: Vec<u32> = vec![];

    for line in reader.lines() {
        let value = line.unwrap();

        if value == "" {
            elfs.push(sum);
            sum = 0;
        } else {
            sum += value.parse::<u32>().unwrap();
        }
    }

    elfs.push(sum);

    let mut biggest = 0;
    elfs.sort_by(|a, b| b.cmp(a));

    for i in  0..3{
        biggest += elfs[i];
    }

    println!("{}", biggest);
}

fn main() {
   let input = include_str!("assets/day1/input2.txt");

   let t = input.split("\n\n").map(|x| x);

   println!("{:?}", t)
}
