use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("assets/day3/input2.txt");

    let arr = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut sum = 0;
    for line in input.lines() {
        let mid = line.len() / 2;

        let chunk = &line[mid..line.len()];
        for c in line[0..mid].chars() {
            if let Some(_) = chunk.find(c) {
                let priority = arr.find(|x| x == c).unwrap() + 1;

                sum += priority;

                break;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
