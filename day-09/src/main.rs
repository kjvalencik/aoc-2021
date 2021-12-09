use std::collections::HashSet;

use util::*;

struct Puzzle {
    heights: Vec<Vec<u32>>,
}

impl Puzzle {
    fn is_lower(&self, n: u32, x: usize, y: usize) -> bool {
        self.heights
            .get(y)
            .and_then(|row| row.get(x))
            .map(|m| n < *m)
            .unwrap_or(true)
    }

    fn low_points(&self) -> impl Iterator<Item = (u32, (usize, usize))> + '_ {
        self.heights
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, n)| (*n, (x, y))))
            .filter(|&(n, (x, y))| {
                (y == 0 || self.is_lower(n, x, y - 1))
                    && self.is_lower(n, x, y + 1)
                    && (x == 0 || self.is_lower(n, x - 1, y))
                    && self.is_lower(n, x + 1, y)
            })
    }

    fn fill(&self, state: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
        let m = self
            .heights
            .get(y)
            .and_then(|row| row.get(x).copied())
            .unwrap_or(9);

        if m >= 9 || !state.insert((x, y)) {
            return;
        }

        self.fill(state, x + 1, y);
        if x > 0 {
            self.fill(state, x - 1, y);
        }

        self.fill(state, x, y + 1);
        if y > 0 {
            self.fill(state, x, y - 1);
        }
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut state = HashSet::new();

        self.fill(&mut state, x, y);

        state.len()
    }

    fn part_1(&self) -> u32 {
        self.low_points().map(|(n, _)| n + 1).sum()
    }

    fn part_2(&self) -> Result<usize, Error> {
        let mut sizes = self
            .low_points()
            .map(|(_, (x, y))| self.basin_size(x, y))
            .collect::<Vec<_>>();

        sizes.sort_unstable();
        sizes.reverse();

        if sizes.len() < 3 {
            Err(Error::msg("Less than 3 basins"))
        } else {
            Ok(sizes[0] * sizes[1] * sizes[2])
        }
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).context("Invalid digit"))
                    .collect()
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { heights })
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 15);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 1134);
        Ok(())
    }
}
