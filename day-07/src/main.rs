use std::convert;

use util::*;

#[derive(Debug)]
struct Puzzle {
    crabs: Vec<i64>,
}

impl Puzzle {
    fn min_max(&self) -> Result<impl Iterator<Item = i64>, Error> {
        self.crabs
            .iter()
            .min()
            .and_then(|&min| self.crabs.iter().max().map(|&max| min..=max))
            .context("No crabs")
    }

    fn fuel(&self, rate: impl Fn(i64) -> i64) -> Result<i64, Error> {
        self.min_max()?
            .map(|n| self.crabs.iter().map(|m| rate((n - m).abs())).sum::<i64>())
            .min()
            .context("No crabs")
    }

    fn part_1(&self) -> Result<i64, Error> {
        self.fuel(convert::identity)
    }

    fn part_2(&self) -> Result<i64, Error> {
        self.fuel(|n| (n * (n + 1)) / 2)
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let crabs = s
            .trim()
            .split(',')
            .map(i64::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { crabs })
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1()?);
    println!("Part 2: {}", puzzle.part_2()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 37);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 168);
        Ok(())
    }
}
