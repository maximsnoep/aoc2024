use anyhow::Result;
use std::fs;

fn part1(input: &str) -> Result<()> {
    let mut result = 0;
    let matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                if i >= 3 && matrix[i - 1][j] == 'M' && matrix[i - 2][j] == 'A' && matrix[i - 3][j] == 'S' {
                    result += 1;
                }
                if j + 3 < matrix[i].len() && matrix[i][j + 1] == 'M' && matrix[i][j + 2] == 'A' && matrix[i][j + 3] == 'S' {
                    result += 1;
                }
                if i + 3 < matrix.len() && matrix[i + 1][j] == 'M' && matrix[i + 2][j] == 'A' && matrix[i + 3][j] == 'S' {
                    result += 1;
                }
                if j >= 3 && matrix[i][j - 1] == 'M' && matrix[i][j - 2] == 'A' && matrix[i][j - 3] == 'S' {
                    result += 1;
                }
                if i >= 3 && j + 3 < matrix[i].len() && matrix[i - 1][j + 1] == 'M' && matrix[i - 2][j + 2] == 'A' && matrix[i - 3][j + 3] == 'S' {
                    result += 1;
                }
                if i + 3 < matrix.len() && j + 3 < matrix[i].len() && matrix[i + 1][j + 1] == 'M' && matrix[i + 2][j + 2] == 'A' && matrix[i + 3][j + 3] == 'S'
                {
                    result += 1;
                }
                if i + 3 < matrix.len() && j >= 3 && matrix[i + 1][j - 1] == 'M' && matrix[i + 2][j - 2] == 'A' && matrix[i + 3][j - 3] == 'S' {
                    result += 1;
                }
                if i >= 3 && j >= 3 && matrix[i - 1][j - 1] == 'M' && matrix[i - 2][j - 2] == 'A' && matrix[i - 3][j - 3] == 'S' {
                    result += 1;
                }
            }
        }
    }

    println!("{result}");

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut result = 0;
    let matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] == 'A' && matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' && matrix[i - 1][j - 1] == 'S' {
                result += 1;
            }
            if matrix[i][j] == 'A' && matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' && matrix[i - 1][j - 1] == 'M' {
                result += 1;
            }
            if matrix[i][j] == 'A' && matrix[i - 1][j + 1] == 'M' && matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'S' {
                result += 1;
            }
            if matrix[i][j] == 'A' && matrix[i - 1][j + 1] == 'S' && matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'M' {
                result += 1;
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
