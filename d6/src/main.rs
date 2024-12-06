use anyhow::Result;
use std::{collections::HashSet, fs};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum State {
    U(usize, usize),
    R(usize, usize),
    D(usize, usize),
    L(usize, usize),
}

fn part1(input: &str) -> Result<()> {
    let mut matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut visited = HashSet::new();
    let mut start = None;
    'o: for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            start = match cell {
                '^' => Some(State::U(i, j)),
                '>' => Some(State::R(i, j)),
                'v' => Some(State::D(i, j)),
                '<' => Some(State::L(i, j)),
                _ => None,
            };
            if start.is_some() {
                visited.insert((i, j));
                matrix[i][j] = '.';
                break 'o;
            }
        }
    }

    if let Some(mut state) = start {
        while let Some(next) = match state {
            State::U(i, j) => {
                if i == 0 {
                    None
                } else if matrix[i - 1][j] == '.' {
                    visited.insert((i, j));
                    Some(State::U(i - 1, j))
                } else {
                    assert!(matrix[i - 1][j] == '#');
                    Some(State::R(i, j))
                }
            }
            State::R(i, j) => {
                if j == matrix[i].len() - 1 {
                    None
                } else if matrix[i][j + 1] == '.' {
                    visited.insert((i, j));
                    Some(State::R(i, j + 1))
                } else {
                    assert!(matrix[i][j + 1] == '#');
                    Some(State::D(i, j))
                }
            }
            State::D(i, j) => {
                if i == matrix.len() - 1 {
                    None
                } else if matrix[i + 1][j] == '.' {
                    visited.insert((i, j));
                    Some(State::D(i + 1, j))
                } else {
                    assert!(matrix[i + 1][j] == '#');
                    Some(State::L(i, j))
                }
            }
            State::L(i, j) => {
                if j == 0 {
                    None
                } else if matrix[i][j - 1] == '.' {
                    visited.insert((i, j));
                    Some(State::L(i, j - 1))
                } else {
                    assert!(matrix[i][j - 1] == '#');
                    Some(State::U(i, j))
                }
            }
        } {
            state = next;
        }
    }

    println!("{}", visited.len() + 1);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut matrix = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut start = None;
    'o: for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            start = match cell {
                '^' => Some(State::U(i, j)),
                '>' => Some(State::R(i, j)),
                'v' => Some(State::D(i, j)),
                '<' => Some(State::L(i, j)),
                _ => None,
            };
            if start.is_some() {
                matrix[i][j] = 's';
                break 'o;
            }
        }
    }

    let mut result = 0;
    for (ii, row) in matrix.iter().enumerate() {
        for (jj, &cell) in row.iter().enumerate() {
            if cell == '#' || cell == 's' {
                continue;
            }
            let mut matrix_copy = matrix.clone();
            let start_copy = start.clone();
            let mut visited = HashSet::new();
            matrix_copy[ii][jj] = '#';

            if let Some(mut state) = start_copy {
                'o: while let Some(next) = {
                    if visited.contains(&state) {
                        result += 1;
                        break 'o;
                    }
                    visited.insert(state.clone());
                    match state {
                        State::U(i, j) => {
                            if i == 0 {
                                None
                            } else if matrix_copy[i - 1][j] == '.' || matrix_copy[i - 1][j] == 's' {
                                Some(State::U(i - 1, j))
                            } else {
                                assert!(matrix_copy[i - 1][j] == '#');
                                Some(State::R(i, j))
                            }
                        }
                        State::R(i, j) => {
                            if j == matrix_copy[i].len() - 1 {
                                None
                            } else if matrix_copy[i][j + 1] == '.' || matrix_copy[i][j + 1] == 's' {
                                Some(State::R(i, j + 1))
                            } else {
                                assert!(matrix_copy[i][j + 1] == '#');
                                Some(State::D(i, j))
                            }
                        }
                        State::D(i, j) => {
                            if i == matrix_copy.len() - 1 {
                                None
                            } else if matrix_copy[i + 1][j] == '.' || matrix_copy[i + 1][j] == 's' {
                                Some(State::D(i + 1, j))
                            } else {
                                assert!(matrix_copy[i + 1][j] == '#');
                                Some(State::L(i, j))
                            }
                        }
                        State::L(i, j) => {
                            if j == 0 {
                                None
                            } else if matrix_copy[i][j - 1] == '.' || matrix_copy[i][j - 1] == 's' {
                                Some(State::L(i, j - 1))
                            } else {
                                assert!(matrix_copy[i][j - 1] == '#');
                                Some(State::U(i, j))
                            }
                        }
                    }
                } {
                    state = next;
                }
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
