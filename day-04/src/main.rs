use util::*;

#[derive(Debug)]
struct Puzzle {
    nums: Vec<u8>,
    cards: Vec<Card>,
}

impl Puzzle {
    fn part_1(&self) -> Result<u64, Error> {
        let mut cards = self.cards.clone();

        for n in self.nums.iter().copied() {
            for card in cards.iter_mut() {
                card.mark(n);

                if card.is_complete() {
                    return Ok(card.score(n));
                }
            }
        }

        bail!("No winner")
    }

    fn part_2(&self) -> Result<u64, Error> {
        let mut cards = self.cards.clone();
        let mut winners = vec![false; cards.len()];
        let mut last = None;

        for n in self.nums.iter().copied() {
            for (i, card) in cards.iter_mut().enumerate() {
                if winners[i] {
                    continue;
                }

                card.mark(n);

                if card.is_complete() {
                    winners[i] = true;
                    last = Some(card.score(n));
                }
            }
        }

        last.context("No winner")
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.trim().split_once("\n\n").context("Missing cards")?;
        let nums = l
            .trim()
            .split(',')
            .map(u8::from_str)
            .collect::<Result<_, _>>()?;

        let cards = r
            .trim()
            .split("\n\n")
            .map(Card::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { nums, cards })
    }
}

#[derive(Clone, Debug)]
struct Card {
    nums: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl Card {
    fn mark(&mut self, n: u8) {
        let point = self.nums.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find(|(_, v)| **v == n)
                .map(|(x, _)| (x, y))
        });

        if let Some((x, y)) = point {
            self.marks[y][x] = true;
        }
    }

    fn is_complete(&self) -> bool {
        self.marks.iter().enumerate().any(|(i, row)| {
            if row.iter().all(|n| *n) {
                return true;
            }

            self.marks.iter().map(|row| row[i]).all(|n| n)
        })
    }

    fn score(&self, n: u8) -> u64 {
        let sum = self
            .marks
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, n)| !*n)
                    .map(move |(x, _)| (x, y))
            })
            .map(|(x, y)| u64::from(self.nums[y][x]))
            .sum::<u64>();

        sum * u64::from(n)
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .lines()
            .map(|line| -> Result<_, Error> {
                line.trim()
                    .split_whitespace()
                    .map(u8::from_str)
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
                    .map_err(|_| Error::msg("Incorrect column length"))
            })
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| Error::msg("Incorrect row size"))?;

        Ok(Self {
            nums,
            marks: Default::default(),
        })
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

    const INPUT: &str = r#"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 4512);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 1924);
        Ok(())
    }
}
