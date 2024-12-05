use anyhow::Result;
use std::fs;

fn part1(input: &str) -> Result<()> {
    let mut result = 0;
    for line in input.lines() {
        let vals = line.split_whitespace().flat_map(|v| v.parse::<i32>()).collect::<Vec<i32>>();

        let is_increasing = vals.windows(2).all(|w| w[0] < w[1]);
        let is_decreasing = vals.windows(2).all(|w| w[0] > w[1]);
        let differ_at_most_3 = vals.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

        if (is_increasing || is_decreasing) && differ_at_most_3 {
            result += 1;
        }
    }

    println!("{result}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut result = 0;
    for line in input.lines() {
        let vals = line.split_whitespace().flat_map(|v| v.parse::<i32>()).collect::<Vec<i32>>();

        for i in 0..vals.len() {
            let val_sans_i = vals[..i].iter().chain(vals[i + 1..].iter()).copied().collect::<Vec<i32>>();
            let is_increasing = val_sans_i.windows(2).all(|w| w[0] < w[1]);
            let is_decreasing = val_sans_i.windows(2).all(|w| w[0] > w[1]);
            let differ_at_most_3 = val_sans_i.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

            if (is_increasing || is_decreasing) && differ_at_most_3 {
                result += 1;
                break;
            }
        }
    }

    println!("{result}");

    Ok(())
}

fn main() -> Result<()> {
    println!("example");
    part1(&fs::read_to_string("example")?)?;
    println!("input");
    part1(&fs::read_to_string("input")?)?;

    println!("example");
    part2(&fs::read_to_string("example")?)?;
    println!("input");
    part2(&fs::read_to_string("input")?)?;

    Ok(())
}
