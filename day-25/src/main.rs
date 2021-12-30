use util::*;

#[derive(Clone, Debug)]
struct Puzzle {
    grid: Vec<Vec<u8>>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, line) in self.grid.iter().enumerate() {
            let line = str::from_utf8(line).unwrap();

            if i + 1 == self.height() {
                write!(f, "{}", line)?;
            } else {
                writeln!(f, "{}", line)?;
            }
        }

        Ok(())
    }
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .lines()
            .map(|line| line.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();

        if grid.is_empty() {
            bail!("Empty grid");
        }

        let width = grid[0].len();

        for line in &grid {
            if line.len() != width {
                bail!("Expected width {}, found {}", width, line.len());
            }

            for c in line {
                if !matches!(c, b'v' | b'>' | b'.') {
                    bail!("Unexpected character: {}", c);
                }
            }
        }

        Ok(Self { grid })
    }
}

impl ops::Index<(usize, usize)> for Puzzle {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let y = y % self.height();
        let x = x % self.width();
        &self.grid[y][x]
    }
}

impl ops::IndexMut<(usize, usize)> for Puzzle {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let y = y % self.height();
        let x = x % self.width();
        &mut self.grid[y][x]
    }
}

impl Puzzle {
    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn part_1(&self) -> usize {
        let mut state = self.clone();

        for i in 1.. {
            let prev = &state;
            let mut next = prev.clone();

            for (y, row) in prev.grid.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    if *c == b'>' && prev[(x + 1, y)] == b'.' {
                        next[(x, y)] = b'.';
                        next[(x + 1, y)] = b'>';
                    }
                }
            }

            let moved_right = prev.grid != next.grid;
            let prev = &next;
            let mut next = prev.clone();

            for (y, row) in prev.grid.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    if *c == b'v' && prev[(x, y + 1)] == b'.' {
                        next[(x, y)] = b'.';
                        next[(x, y + 1)] = b'v';
                    }
                }
            }

            if !moved_right && prev.grid == next.grid {
                return i;
            }

            state = next;
        }

        unreachable!()
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 58);
        Ok(())
    }
}
