use util::*;

#[derive(Debug)]
struct Puzzle {
    energy: Vec<Vec<u64>>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            energy: parse_grid(s)?,
        })
    }
}

impl Puzzle {
    fn simulation(&self, energy: &mut Vec<Vec<u64>>) -> u64 {
        fn run(energy: &mut Vec<Vec<u64>>, x: usize, y: usize) -> u64 {
            if y >= energy.len() {
                return 0;
            }

            let row = &mut energy[y];

            if x >= row.len() {
                return 0;
            }

            let n = &mut row[x];

            *n += 1;
            if *n != 10 {
                return 0;
            }

            let mut total = 1;

            if x > 0 {
                total += run(energy, x - 1, y);
                total += run(energy, x - 1, y + 1);
            }
            if y > 0 {
                total += run(energy, x, y - 1);
                total += run(energy, x + 1, y - 1);
            }
            if x > 0 && y > 0 {
                total += run(energy, x - 1, y - 1);
            }

            total += run(energy, x + 1, y);
            total += run(energy, x, y + 1);
            total += run(energy, x + 1, y + 1);

            total
        }

        let mut total = 0;

        for (y, row) in self.energy.iter().enumerate() {
            for x in 0..row.len() {
                total += run(energy, x, y);
            }
        }

        for row in energy.iter_mut() {
            for n in row.iter_mut() {
                if *n >= 10 {
                    *n = 0;
                }
            }
        }

        for row in energy.iter_mut() {
            for n in row.iter_mut() {
                if *n >= 10 {
                    *n = 0;
                }
            }
        }

        total
    }

    fn part_1(&self) -> u64 {
        let mut total = 0;
        let mut energy = self.energy.clone();

        for _ in 0..100 {
            total += self.simulation(&mut energy);
        }

        total
    }

    fn part_2(&self) -> usize {
        let mut energy = self.energy.clone();

        for i in 1.. {
            self.simulation(&mut energy);

            if energy.iter().flat_map(|row| row.iter()).all(|n| *n == 0) {
                return i;
            }
        }

        unreachable!()
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

    static INPUT: &str = r#"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 1656);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2(), 195);
        Ok(())
    }
}
