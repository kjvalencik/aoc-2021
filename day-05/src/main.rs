use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

use util::*;

#[derive(Debug)]
struct Puzzle {
    segments: Vec<Segment>,
}

impl Puzzle {
    fn part_1(&self) -> usize {
        let segments = self
            .segments
            .iter()
            .filter(|s| s.start.0 == s.end.0 || s.start.1 == s.end.1)
            .cloned()
            .collect();

        let puzzle = Self { segments };

        puzzle.part_2()
    }

    fn part_2(&self) -> usize {
        let mut counts = HashMap::<_, usize>::new();
        let mut incr = |x, y| *counts.entry((x, y)).or_default() += 1;
        let delta = |a: i64, b: i64| match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        for segment in &self.segments {
            let i = delta(segment.start.0, segment.end.0);
            let j = delta(segment.start.1, segment.end.1);
            let (mut x, mut y) = segment.start;

            while (x, y) != segment.end {
                incr(x, y);
                x += i;
                y += j;
            }

            // Range is inclusive
            incr(x, y);
        }

        counts.into_iter().filter(|(_, v)| *v > 1).count()
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s
            .trim()
            .lines()
            .map(Segment::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { segments })
    }
}

#[derive(Clone, Debug)]
struct Segment {
    start: (i64, i64),
    end: (i64, i64),
}

impl FromStr for Segment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_pair = |pair: &str| -> Result<_, Error> {
            let (l, r) = pair.split_once(',').context("Invalid pair")?;

            Ok((i64::from_str(l)?, i64::from_str(r)?))
        };

        let (start, end) = s.trim().split_once(" -> ").context("Invalid segment")?;

        Ok(Self {
            start: parse_pair(start)?,
            end: parse_pair(end)?,
        })
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 5);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2(), 12);
        Ok(())
    }
}
