use util::*;

type Point = (i32, i32, i32);

#[derive(Debug)]
struct Puzzle {
    scanners: Vec<Scanner>,
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: HashSet<Point>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scanners = s
            .trim()
            .split("--- scanner")
            .skip(1)
            .map(Scanner::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { scanners })
    }
}

impl FromStr for Scanner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = s
            .trim()
            .lines()
            .skip(1)
            .map(|s| {
                let mut nums = s.trim().split(',').map(i32::from_str);
                let mut next = || nums.next().context("Missing coordinate");

                Ok((next()??, next()??, next()??))
            })
            .collect::<Result<HashSet<_>, Self::Err>>()?;

        Ok(Self { beacons })
    }
}

impl Scanner {
    fn rotate(&self, m: Point, f: fn(Point) -> Point) -> Self {
        let beacons = self
            .beacons
            .iter()
            .copied()
            .map(move |(x, y, z)| f((x * m.0, y * m.1, z * m.2)))
            .collect();

        Self { beacons }
    }

    fn translate(&self, p: Point) -> Self {
        let beacons = self
            .beacons
            .iter()
            .copied()
            .map(move |(x, y, z)| (x + p.0, y + p.1, z + p.2))
            .collect();

        Self { beacons }
    }

    fn shift_axis(&self, scanner: &Self, f: fn(&Point) -> i32) -> Option<i32> {
        let target = scanner.beacons.iter().map(f).collect::<HashSet<_>>();

        (-10000..10000).find_map(|n| {
            let count = self
                .beacons
                .iter()
                .map(|p| f(p) + n)
                .filter(|n| target.contains(n))
                .count();

            (count >= 12 && count != target.len()).then(|| n)
        })
    }

    fn shift(&self, target: &Self) -> Option<(Self, Point)> {
        fn get_x((x, _, _): &Point) -> i32 {
            *x
        }

        fn get_y((_, y, _): &Point) -> i32 {
            *y
        }

        fn get_z((_, _, z): &Point) -> i32 {
            *z
        }

        let x = self.shift_axis(target, get_x)?;
        let y = self.translate((x, 0, 0)).shift_axis(target, get_y)?;
        let z = self.translate((x, y, 0)).shift_axis(target, get_z)?;

        Some((self.translate((x, y, z)), (x, y, z)))
    }

    fn align(&self, target: &Self) -> Option<(Self, Point)> {
        fn flip_1((x, y, z): Point) -> Point {
            (x, y, z)
        }

        fn flip_2((x, y, z): Point) -> Point {
            (x, z, y)
        }

        fn flip_3((x, y, z): Point) -> Point {
            (y, x, z)
        }

        fn flip_4((x, y, z): Point) -> Point {
            (z, x, y)
        }

        fn flip_5((x, y, z): Point) -> Point {
            (y, z, x)
        }

        fn flip_6((x, y, z): Point) -> Point {
            (z, y, x)
        }

        let tries = [1, -1]
            .into_iter()
            .flat_map(|x| [1, -1].into_iter().map(move |y| (x, y)))
            .flat_map(|(x, y)| [1, -1].into_iter().map(move |z| (x, y, z)))
            .flat_map(|p| {
                [flip_1, flip_2, flip_3, flip_4, flip_5, flip_6]
                    .into_iter()
                    .map(move |f| (p, f))
            })
            .collect::<Vec<_>>();

        tries
            .into_par_iter()
            .find_map_any(|(p, f)| self.rotate(p, f).shift(target))
    }
}

impl Puzzle {
    fn part_1_and_2(&self) -> Result<(usize, i32), Error> {
        let mut known = self.scanners.iter().take(1).cloned().collect::<Vec<_>>();
        let mut offsets = vec![(0, 0, 0)];
        let mut scanners = self.scanners.iter().skip(1).cloned().collect::<Vec<_>>();

        while !scanners.is_empty() {
            let (i, (s, o)) = scanners
                .iter()
                .enumerate()
                .find_map(|(i, s)| {
                    known
                        .iter()
                        .find_map(move |target| s.align(target).map(move |s| (i, s)))
                })
                .context("No overlap")?;

            offsets.push(o);
            known.push(s);
            scanners.remove(i);
        }

        let beacons = known
            .into_iter()
            .flat_map(|scanner| scanner.beacons.into_iter())
            .collect::<HashSet<_>>();

        let max = offsets
            .iter()
            .enumerate()
            .flat_map(|(i, p)| {
                offsets
                    .iter()
                    .enumerate()
                    .filter(move |(j, _)| i != *j)
                    .map(move |(_, q)| (p, q))
            })
            .map(|(p, q)| (p.0 - q.0).abs() + (p.1 - q.1).abs() + (p.2 - q.2).abs())
            .max()
            .context("No scanners")?;

        Ok((beacons.len(), max))
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;
    let (part_1, part_2) = puzzle.part_1_and_2()?;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = include_str!("../test/input.txt");

    #[test]
    fn part_1_and_2() -> Result<(), Error> {
        let (part_1, part_2) = Puzzle::from_str(INPUT)?.part_1_and_2()?;
        assert_eq!(part_1, 79);
        assert_eq!(part_2, 3621);
        Ok(())
    }
}
