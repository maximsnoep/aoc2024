use anyhow::Result;
use std::{collections::HashMap, fs};

fn part1(input: &str) -> Result<()> {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in input.lines() {
        let vals = line.split_whitespace().collect::<Vec<&str>>();
        assert!(vals.len() == 2);
        list1.push(vals[0].parse::<i32>()?);
        list2.push(vals[1].parse::<i32>()?);
    }

    let mut sorted1 = list1.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    sorted1.sort_by_key(|(_, val)| *val);
    let mut sorted2 = list2.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    sorted2.sort_by_key(|(_, val)| *val);

    let zipped = sorted1
        .iter()
        .zip(sorted2.iter())
        .map(|((_, val1), (_, val2))| (val1 - val2).abs())
        .sum::<i32>();

    println!("{zipped}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let vals = line.split_whitespace().collect::<Vec<&str>>();
        assert!(vals.len() == 2);
        list1.push(vals[0].parse::<i32>()?);
        list2.push(vals[1].parse::<i32>()?);
    }

    let mut count = HashMap::new();
    for val in list2 {
        count.get_mut(&val).map(|v| *v += 1).unwrap_or_else(|| {
            count.insert(val, 1);
        });
    }

    let mut similarity = 0;
    for val in list1 {
        similarity += val * count.get(&val).unwrap_or(&0);
    }

    println!("{similarity}");

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
