use util::*;

#[derive(Clone, Debug)]
struct Puzzle {
    algorithm: [char; 512],
    image: Vec<Vec<char>>,
    overflow: char,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.image.iter() {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.trim().split_once("\n\n").context("Missing image")?;
        let algorithm = l
            .trim()
            .lines()
            .flat_map(|line| line.trim().chars())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| Error::msg("Invalid algorithm"))?;

        let image = r
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self {
            algorithm,
            image,
            overflow: '.',
        })
    }
}

impl Puzzle {
    fn height(&self) -> usize {
        self.image.len()
    }

    fn width(&self) -> usize {
        self.image[0].len()
    }

    fn enhance(&self) -> Result<Self, Error> {
        let mut source = vec![vec![self.overflow; self.width() + 4]; self.height() + 4];
        let overflow = if self.overflow == '#' {
            self.algorithm[511]
        } else {
            self.algorithm[0]
        };

        for (y, line) in self.image.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                source[y + 2][x + 2] = *c;
            }
        }

        let mut image = vec![vec![overflow; source[0].len()]; source.len()];

        for (y, row) in image
            .iter_mut()
            .enumerate()
            .take((self.height() + 2) + 1)
            .skip(1)
        {
            for (x, cell) in row
                .iter_mut()
                .enumerate()
                .take((self.width() + 2) + 1)
                .skip(1)
            {
                let i = ((y - 1)..=(y + 1))
                    .flat_map(|y| ((x - 1)..=(x + 1)).map(move |x| (x, y)))
                    .map(|(x, y)| if source[y][x] == '#' { '1' } else { '0' })
                    .collect::<String>();

                let i = usize::from_str_radix(&i, 2)?;

                *cell = self.algorithm[i];
            }
        }

        Ok(Self {
            algorithm: self.algorithm,
            image,
            overflow,
        })
    }

    fn count(&self) -> usize {
        self.image
            .iter()
            .flat_map(|line| line.iter())
            .filter(|c| **c == '#')
            .count()
    }

    fn part_1(&self) -> Result<usize, Error> {
        Ok(self.enhance()?.enhance()?.count())
    }

    fn part_2(&self) -> Result<usize, Error> {
        let mut puzzle = self.clone();

        for _ in 0..50 {
            puzzle = puzzle.enhance()?;
        }

        Ok(puzzle.count())
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

    static INPUT: &str = r#"
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 35);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 3351);
        Ok(())
    }
}
