use util::*;

#[derive(Clone, Debug)]
struct Puzzle {
    maze: Vec<Vec<u64>>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            maze: parse_grid(s)?,
        })
    }
}

impl Puzzle {
    fn part_1(&self) -> Result<u64, Error> {
        let size = self.maze.iter().flat_map(|r| r.iter()).count();
        let mut spt = HashSet::new();
        let mut distances = HashMap::new();
        let update_distance = |distances: &mut HashMap<_, _>, (x, y), d| {
            if let Some(e) = self.maze.get(y).and_then(|r: &Vec<u64>| r.get(x)) {
                let d = d + e;

                if distances.get(&(x, y)).map(|e| d < *e).unwrap_or(true) {
                    distances.insert((x, y), d);
                }
            }
        };

        distances.insert((0usize, 0usize), 0u64);

        while spt.len() < size {
            let ((x, y), d) = distances
                .iter()
                .filter(|(k, _)| !spt.contains(*k))
                .min_by_key(|(_, d)| *d)
                .map(|(p, d)| (*p, *d))
                .context("Invalid maze")?;

            spt.insert((x, y));

            if x > 0 {
                update_distance(&mut distances, (x - 1, y), d);
            }

            if y > 0 {
                update_distance(&mut distances, (x, y - 1), d);
            }

            update_distance(&mut distances, (x + 1, y), d);
            update_distance(&mut distances, (x, y + 1), d);
        }

        let dest = self
            .maze
            .iter()
            .enumerate()
            .flat_map(|(y, r)| (0..r.len()).map(move |x| (x, y)))
            .last()
            .context("Invalid maze")?;

        Ok(*distances.get(&dest).context("No path found")?)
    }

    fn part_2(&self) -> Result<u64, Error> {
        let mut puzzle = self.clone();

        for (i, row) in puzzle.maze.iter_mut().enumerate() {
            for j in 1..5 {
                let additional = self.maze[i]
                    .iter()
                    .map(|&n| n + j)
                    .map(|n| if n > 9 { n % 9 } else { n })
                    .collect::<Vec<_>>();

                row.extend_from_slice(&additional);
            }
        }

        for i in 1..5 {
            let additional = puzzle
                .maze
                .iter()
                .take(self.maze.len())
                .map(|row| {
                    row.iter()
                        .map(|&n| n + i)
                        .map(|n| if n > 9 { n % 9 } else { n })
                        .collect()
                })
                .collect::<Vec<_>>();

            puzzle.maze.extend_from_slice(&additional);
        }

        puzzle.part_1()
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
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 40);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 315);
        Ok(())
    }
}
