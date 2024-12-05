use anyhow::Result;
use std::{cmp::Ordering, collections::HashMap, fs};

fn part1(input: &str) -> Result<()> {
    let mut result = 0;
    let mut done = false;
    let mut ordering = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            done = true;
            continue;
        }
        if !done {
            let a = line.split("|").collect::<Vec<&str>>();
            ordering.insert((a[0], a[1]), Ordering::Less);
            ordering.insert((a[1], a[0]), Ordering::Greater);
        } else {
            let a = line.split(",").collect::<Vec<&str>>();
            if a.is_sorted_by(|a, b| ordering.get(&(a, b)).unwrap_or(&Ordering::Equal).is_le()) {
                result += a.get(a.len() / 2).unwrap().parse::<i32>().unwrap();
            }
        }
    }

    println!("{result}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut result = 0;
    let mut done = false;
    let mut ordering = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            done = true;
            continue;
        }
        if !done {
            let a = line.split("|").collect::<Vec<&str>>();
            ordering.insert((a[0], a[1]), Ordering::Less);
            ordering.insert((a[1], a[0]), Ordering::Greater);
        } else {
            let mut a = line.split(",").collect::<Vec<&str>>();
            if !a.is_sorted_by(|a, b| ordering.get(&(a, b)).unwrap_or(&Ordering::Equal).is_le()) {
                a.sort_by(|a, b| *ordering.get(&(a, b)).unwrap_or(&Ordering::Equal));
                result += a.get(a.len() / 2).unwrap().parse::<i32>().unwrap();
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
