use util::*;

#[derive(Clone)]
struct Puzzle {
    len: usize,
    nums: Vec<u64>,
}

impl Puzzle {
    fn parse(input: &str) -> Result<Self, Error> {
        let len = input
            .trim()
            .lines()
            .map(|line| line.trim().len())
            .max()
            .context("Expected puzzle input")?;

        let nums = input
            .trim()
            .lines()
            .map(|line| u64::from_str_radix(line.trim(), 2))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { nums, len })
    }

    fn gamma_epsilon(&self) -> (u64, u64) {
        let mut gamma = 0;
        let mut epsilon = 0;
        let mut mask = 1;

        for _ in 0..self.len {
            let count = self.nums.iter().filter(|n| *n & mask != 0).count();

            if count * 2 >= self.nums.len() {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }

            mask <<= 1;
        }

        (gamma, epsilon)
    }

    fn rating(&self, use_gamma: bool) -> u64 {
        let mut puzzle = self.clone();
        let mut mask = 1 << puzzle.len - 1;

        while puzzle.nums.len() > 1 {
            let (gamma, epsilon) = puzzle.gamma_epsilon();
            let rating = if use_gamma { gamma } else { epsilon };

            puzzle.nums = puzzle
                .nums
                .into_iter()
                .filter(|n| rating & mask == n & mask)
                .collect();

            mask >>= 1;
        }

        puzzle.nums[0]
    }

    fn oxygen(&self) -> u64 {
        self.rating(true)
    }

    fn c02(&self) -> u64 {
        self.rating(false)
    }

    fn part_1(&self) -> u64 {
        let (gamma, epsilon) = self.gamma_epsilon();

        gamma * epsilon
    }

    fn part_2(&self) -> u64 {
        self.oxygen() * self.c02()
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::parse(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::parse(INPUT)?.part_1(), 198);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::parse(INPUT)?.part_2(), 230);
        Ok(())
    }

    #[test]
    fn oxygen() -> Result<(), Error> {
        assert_eq!(Puzzle::parse(INPUT)?.oxygen(), 23);
        Ok(())
    }

    #[test]
    fn c02() -> Result<(), Error> {
        assert_eq!(Puzzle::parse(INPUT)?.c02(), 10);
        Ok(())
    }
}
