use util::*;

#[derive(Debug)]
struct Puzzle {
    initial: Vec<u8>,
    patterns: HashMap<[u8; 2], u8>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().lines().map(str::trim).join("\n");

        let (l, r) = s.split_once("\n\n").context("Missing patterns")?;

        let initial = l.trim().into();

        let patterns = r
            .trim()
            .lines()
            .map(|line| {
                let (l, r) = line.split_once(" -> ").context("Missing replacement")?;
                let l: [u8; 2] = l.as_bytes().try_into()?;
                let r = *r.as_bytes().get(0).context("Missing replacement")?;

                Ok((l, r))
            })
            .collect::<Result<_, Error>>()?;

        Ok(Self { initial, patterns })
    }
}

impl Puzzle {
    fn solve(&self, n: usize) -> Result<usize, Error> {
        let mut counts = HashMap::<[u8; 2], usize>::new();

        for pair in self.initial.windows(2) {
            *counts.entry(pair.try_into()?).or_default() += 1;
        }

        for _ in 0..n {
            let mut next = counts.clone();

            for (pair, n) in counts.into_iter() {
                if let Some(c) = self.patterns.get(&pair).copied() {
                    *next.entry([pair[0], c]).or_default() += n;
                    *next.entry([c, pair[1]]).or_default() += n;
                    *next.entry(pair).or_default() -= n;
                }
            }

            counts = next;
        }

        let mut totals = HashMap::<u8, usize>::new();
        let last = self.initial.iter().last().context("Empty initial")?;

        // The last character needs an extra count
        totals.insert(*last, 1);

        for (pair, n) in counts.into_iter() {
            *totals.entry(pair[0]).or_default() += n;
        }

        let min = totals.values().min().context("No pairs")?;
        let max = totals.values().max().context("No pairs")?;

        Ok(max - min)
    }

    fn part_1(&self) -> Result<usize, Error> {
        self.solve(10)
    }

    fn part_2(&self) -> Result<usize, Error> {
        self.solve(40)
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
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 1588);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 2188189693529);
        Ok(())
    }
}
