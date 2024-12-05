use anyhow::Result;
use std::fs;

fn part1(input: &str) -> Result<()> {
    let result = input
        .split("mul(")
        .map(|mul| mul.chars())
        .flat_map(|mut mul| {
            if let (Ok(a), Ok(b)) = (
                mul.by_ref().take_while(|c| *c != ',').collect::<String>().parse::<i32>(),
                mul.by_ref().take_while(|c| *c != ')').collect::<String>().parse::<i32>(),
            ) {
                Some(a * b)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("{result}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let result = input
        .split("don't()")
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_string()
            } else {
                s.split("do()").skip(1).collect::<Vec<&str>>().join("")
            }
        })
        .map(|dos| {
            dos.split("mul(")
                .map(|mul| mul.chars())
                .flat_map(|mut mul| {
                    if let (Ok(a), Ok(b)) = (
                        mul.by_ref().take_while(|c| *c != ',').collect::<String>().parse::<i32>(),
                        mul.by_ref().take_while(|c| *c != ')').collect::<String>().parse::<i32>(),
                    ) {
                        Some(a * b)
                    } else {
                        None
                    }
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{result}");

    Ok(())
}

fn main() -> Result<()> {
    println!("example");
    part1(&fs::read_to_string("example")?)?;
    println!("input");
    part1(&fs::read_to_string("input")?)?;

    println!("example2");
    part2(&fs::read_to_string("example2")?)?;
    println!("input");
    part2(&fs::read_to_string("input")?)?;

    Ok(())
}
