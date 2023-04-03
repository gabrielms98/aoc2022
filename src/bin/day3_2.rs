use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("assets/day3/input1.txt");

    let arr = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut sum = 0;

    let lines = input.lines().collect::<Vec<&str>>();

    for group in lines.chunks(3) {
        let mut g: Vec<&str> = vec![];
        for line in group {
            g.push(&line[0..line.len()/2]);
        }

        for line in g {

        }
    }


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
