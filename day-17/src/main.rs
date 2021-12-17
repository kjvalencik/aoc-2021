use util::*;

#[derive(Debug)]
struct Puzzle {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s: &str, c| -> Result<_, Error> {
            let (l, r) = s
                .trim()
                .trim_start_matches(c)
                .split_once("..")
                .context("Missing end")?;

            let start = i64::from_str(l.trim())?;
            let end = i64::from_str(r.trim())?;

            Ok(start..=end)
        };

        let (x, y) = s
            .trim()
            .trim_start_matches("target area: ")
            .split_once(", ")
            .context("Missing area range")?;

        let x = parse(x, "x=")?;
        let y = parse(y, "y=")?;

        Ok(Self { x, y })
    }
}

impl Puzzle {
    fn experiment(&self, mut velocity: (i64, i64)) -> Option<i64> {
        let (mut x, mut y) = (0, 0);
        let mut max = 0;

        while x <= *self.x.end() && y > *self.y.start() {
            x += velocity.0;
            y += velocity.1;

            if y > max {
                max = y;
            }

            if self.x.contains(&x) && self.y.contains(&y) {
                return Some(max);
            }

            if x == 0 {
                return None;
            }

            match velocity.0 {
                n if n > 0 => velocity.0 -= 1,
                n if n < 0 => velocity.0 += 1,
                _ => {}
            }

            velocity.1 -= 1;
        }

        None
    }

    fn part_1(&self) -> i64 {
        let mut max = 0;

        for y in 0..=(self.y.end().abs() * 2) {
            for x in 0..=*self.x.end() {
                if let Some(n) = self.experiment((x, y)) {
                    if n > max {
                        max = n;
                    }
                }
            }
        }

        max
    }

    fn part_2(&self) -> i64 {
        let mut count = 0;

        for y in *self.y.start()..=(self.y.end().abs() * 2) {
            for x in 0..=*self.x.end() {
                if self.experiment((x, y)).is_some() {
                    count += 1;
                }
            }
        }

        count
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

    static INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1(), 45);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2(), 112);
        Ok(())
    }
}
