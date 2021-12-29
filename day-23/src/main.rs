use util::*;

type Row = [char; 13];

#[derive(Clone, Debug)]
struct Puzzle<const HEIGHT: usize> {
    energy: usize,
    map: [Row; HEIGHT],
}

impl FromStr for Puzzle<5> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = [[' '; 13]; 5];

        for (i, line) in s
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<_>>())
            .enumerate()
        {
            if i >= 5 {
                bail!("Too many lines");
            }

            if i < 3 {
                if line.len() != 13 {
                    bail!("Invalid line");
                }

                map[i].copy_from_slice(&line);
            } else {
                if line.len() != 9 {
                    bail!("Invalid line");
                }

                (&mut map[i][2..11]).copy_from_slice(&line);
            }
        }

        Ok(Self { energy: 0, map })
    }
}

impl<const HEIGHT: usize> fmt::Display for Puzzle<HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = self
            .map
            .iter()
            .map(|line| line.iter().collect::<String>())
            .join("\n");

        f.write_str(&m)
    }
}

impl<const HEIGHT: usize> Puzzle<HEIGHT> {
    const ROOMS: [(usize, char); 4] = [(3, 'A'), (5, 'B'), (7, 'C'), (9, 'D')];

    fn is_hall(x: &usize) -> bool {
        matches!(x, 1 | 2 | 4 | 6 | 8 | 10 | 11)
    }

    fn energy(c: char, source: (usize, usize), dest: (usize, usize)) -> usize {
        let diff = |a, b| if a > b { a - b } else { b - a };

        let d = diff(source.0, dest.0) + diff(source.1, dest.1);

        d * match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        }
    }

    fn is_solved(&self) -> bool {
        for (x, r) in Self::ROOMS {
            for row in &self.map[2..(self.map.len() - 1)] {
                if row[x] != r {
                    return false;
                }
            }
        }

        true
    }

    fn swap(&self, c: char, source: (usize, usize), dest: (usize, usize)) -> Self {
        let mut puzzle = self.clone();

        puzzle.map[source.1][source.0] = '.';
        puzzle.map[dest.1][dest.0] = c;
        puzzle.energy += Self::energy(c, source, dest);
        puzzle
    }

    fn run(&self, cache: &mut HashMap<[Row; HEIGHT], usize>, min: &mut usize) {
        if self.is_solved() {
            *min = self.energy.min(*min);
            return;
        }

        match cache.entry(self.map) {
            Entry::Occupied(mut o) => {
                if self.energy >= *o.get() {
                    return;
                }

                o.insert(self.energy);
            }
            Entry::Vacant(v) => {
                v.insert(self.energy);
            }
        }

        let hall = &self.map[1];

        for (x, r) in Self::ROOMS {
            let rooms = &self.map[2..];

            for (y, row) in rooms.iter().enumerate() {
                let c = row[x];

                if c != '.' {
                    let is_end = y == rooms.len() - 1;
                    let is_mixed = !is_end
                        && (&rooms[y..(rooms.len() - 1)])
                        .iter()
                        .any(|room| room[x] != r);

                    let mut check = |dest| {
                        let d = hall[dest];

                        if d != '.' {
                            if d == r && !is_mixed {
                                self.swap(d, (dest, 1), (x, y + 1)).run(cache, min);
                            }

                            return true;
                        }

                        if is_mixed {
                            self.swap(c, (x, y + 2), (dest, 1)).run(cache, min);
                        }

                        false
                    };

                    for dest in (1..x).filter(Self::is_hall).rev() {
                        if check(dest) {
                            break;
                        }
                    }

                    for dest in (x..=11).filter(Self::is_hall) {
                        if check(dest) {
                            break;
                        }
                    }

                    break;
                }
            }
        }
    }

    fn part_1(&self) -> Result<usize, Error> {
        let mut cache = HashMap::new();
        let mut min = usize::MAX;

        self.run(&mut cache, &mut min);

        if min == usize::MAX {
            bail!("No solution found");
        }

        Ok(min)
    }

    fn part_2(&self) -> Result<usize, Error> {
        let puzzle = Puzzle {
            energy: 0,
            map: [
                self.map[0],
                self.map[1],
                self.map[2],
                [
                    ' ', ' ', '#', 'D', '#', 'C', '#', 'B', '#', 'A', '#', ' ', ' ',
                ],
                [
                    ' ', ' ', '#', 'D', '#', 'B', '#', 'A', '#', 'C', '#', ' ', ' ',
                ],
                self.map[3],
                self.map[4],
            ],
        };

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
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 12521);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 44169);
        Ok(())
    }
}
