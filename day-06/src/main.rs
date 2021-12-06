use util::*;

#[derive(Debug)]
struct Puzzle {
    fish: [usize; 9],
}

impl Puzzle {
    fn days(&self, n: usize) -> usize {
        let mut fish = self.fish.clone();

        for i in 0..n {
            fish[(i + 7) % 9] += fish[i % 9];
        }

        fish.into_iter().sum()
    }

    fn part_1(&self) -> usize {
        self.days(80)
    }

    fn part_2(&self) -> usize {
        self.days(256)
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fish = [0; 9];

        for n in s.trim().split(',') {
            fish[usize::from_str(n)?] += 1;
        }

        Ok(Self { fish })
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 1: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 5934);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2(), 26984457539);
        Ok(())
    }
}
