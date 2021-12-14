use util::*;

#[derive(Debug)]
struct Puzzle {
    points: Vec<(usize, usize)>,
    folds: Vec<Fold>,
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().lines().map(str::trim).join("\n");
        let (l, r) = s.split_once("\n\n").context("Missing folds")?;

        let points = l
            .trim()
            .lines()
            .map(|l| {
                let (l, r) = l.trim().split_once(',').context("Malformed point")?;

                Ok((usize::from_str(l)?, usize::from_str(r)?))
            })
            .collect::<Result<_, Error>>()?;

        let folds = r
            .trim()
            .lines()
            .map(|l| {
                let (l, r) = l.trim().split_once('=').context("Malformed fold")?;
                let n = usize::from_str(r)?;

                match l {
                    "fold along x" => Ok(Fold::X(n)),
                    "fold along y" => Ok(Fold::Y(n)),
                    _ => bail!("Malformed fold"),
                }
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self { points, folds })
    }
}

impl Puzzle {
    fn fold(points: impl Iterator<Item = (usize, usize)>, fold: &Fold) -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();

        for (x, y) in points {
            set.insert(match fold {
                Fold::X(n) => {
                    if x < *n {
                        (x, y)
                    } else {
                        (2 * *n - x, y)
                    }
                }
                Fold::Y(n) => {
                    if y < *n {
                        (x, y)
                    } else {
                        (x, 2 * *n - y)
                    }
                }
            });
        }

        set
    }

    fn part_1(&self) -> Result<usize, Error> {
        let fold = self.folds.get(0).context("No folds")?;
        let points = Self::fold(self.points.iter().cloned(), fold);

        Ok(points.len())
    }

    fn part_2(&self) -> Result<String, Error> {
        let mut points = self.points.iter().cloned().collect::<HashSet<_>>();

        for fold in self.folds.iter() {
            points = Self::fold(points.into_iter(), fold);
        }

        let min_x = points.iter().map(|(x, _)| *x).min().context("No points")?;
        let min_y = points.iter().map(|(_, y)| *y).min().context("No points")?;
        let max_x = points.iter().map(|(x, _)| *x).max().context("No points")?;
        let max_y = points.iter().map(|(_, y)| *y).max().context("No points")?;

        let mut canvas = vec![vec![' '; max_x - min_x + 1]; max_y - min_y + 1];

        for (x, y) in points {
            canvas[y - min_y][x - min_x] = '#';
        }

        let output = canvas
            .into_iter()
            .map(|line| line.into_iter().collect::<String>())
            .join("\n");

        Ok(output)
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1()?);
    println!("Part 2:\n{}", puzzle.part_2()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 17);
        Ok(())
    }
}
