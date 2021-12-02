use std::io;
use std::str::FromStr;

use anyhow::{Context, Error};

fn read_stdin() -> Result<String, Error> {
    let mut buf = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut buf)?;
    Ok(buf)
}

enum Direction {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.trim().split_once(' ').context("Missing distance")?;

        let d = r.parse()?;

        Ok(match l {
            "up" => Self::Up(d),
            "down" => Self::Down(d),
            "forward" => Self::Forward(d),
            _ => anyhow::bail!("Invalid direction {}", l),
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<Direction>, Error> {
    input
        .trim()
        .lines()
        .map(Direction::from_str)
        .collect()
}

fn part_1(directions: &[Direction]) -> i64 {
    let (mut x, mut z) = (0, 0);

    for d in directions {
        match d {
            Direction::Up(n) => z -= n,
            Direction::Down(n) => z += n,
            Direction::Forward(n) => x += n,
        }
    }

    x * z
}

fn part_2(directions: &[Direction]) -> i64 {
    let (mut x, mut z, mut aim) = (0, 0, 0);

    for d in directions {
        match d {
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
            Direction::Forward(n) => {
                x += n;
                z += aim * n;
            }
        }
    }

    x * z
}

fn main() -> Result<(), Error> {
    let directions = parse_input(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&directions));
    println!("Part 2: {}", part_2(&directions));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        let directions = parse_input(INPUT)?;
        assert_eq!(super::part_1(&directions), 150);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        let directions = parse_input(INPUT)?;
        assert_eq!(super::part_2(&directions), 900);
        Ok(())
    }
}
