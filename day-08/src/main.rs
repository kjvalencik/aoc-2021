use util::*;

const ZERO: &[u8] = b"abcefg";
const ONE: &[u8] = b"cf";
const TWO: &[u8] = b"acdeg";
const THREE: &[u8] = b"acdfg";
const FOUR: &[u8] = b"bcdf";
const FIVE: &[u8] = b"abdfg";
const SIX: &[u8] = b"abdefg";
const SEVEN: &[u8] = b"acf";
const EIGHT: &[u8] = b"abcdefg";
const NINE: &[u8] = b"abcdfg";

#[derive(Debug)]
struct Pattern {
    samples: Vec<String>,
    output: Vec<String>,
}

fn wire_lookup(pattern: &[u8], num: &[u8]) -> Result<usize, Error> {
    const OFFSET: u8 = b'a';

    let mut num = num
        .iter()
        .map(|c| pattern[usize::from(c - OFFSET)] + OFFSET)
        .collect::<Vec<_>>();

    num.sort_unstable();

    [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE]
        .into_iter()
        .enumerate()
        .find_map(|(i, n)| (num == n).then(move || i))
        .context("Invalid order")
}

impl Pattern {
    fn pattern(&self) -> Result<Vec<u8>, Error> {
        (0..7u8)
            .permutations(7)
            .find(|pattern| {
                self.samples
                    .iter()
                    .all(|sample| wire_lookup(pattern, sample.as_bytes()).is_ok())
            })
            .context("Invalid wire combination")
    }

    fn nums(&self) -> Result<Vec<usize>, Error> {
        let pattern = self.pattern()?;

        self.output
            .iter()
            .map(|n| wire_lookup(&pattern, n.as_bytes()))
            .collect()
    }

    fn output(&self) -> Result<usize, Error> {
        let (_, output) = self
            .nums()?
            .into_iter()
            .rev()
            .fold((1, 0), |(o, sum), n| (o * 10, sum + (o * n)));

        Ok(output)
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = |s: &str| s.trim().split(' ').map(String::from).collect::<Vec<_>>();
        let (samples, output) = s.trim().split_once('|').context("Missing output")?;

        Ok(Self {
            samples: parse(samples),
            output: parse(output),
        })
    }
}

fn part_1(patterns: &[Pattern]) -> usize {
    patterns
        .iter()
        .flat_map(|pattern| pattern.output.iter())
        .filter(|output| {
            [ONE, FOUR, SEVEN, EIGHT]
                .iter()
                .any(|d| output.len() == d.len())
        })
        .count()
}

fn part_2(patterns: &[Pattern]) -> Result<usize, Error> {
    patterns
        .iter()
        .try_fold(0, |acc, pattern| Ok(pattern.output()? + acc))
}

fn main() -> Result<(), Error> {
    let patterns = read_stdin()?.try_from_lines()?;

    println!("Part 1: {}", part_1(&patterns));
    println!("Part 2: {}", part_2(&patterns)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "#;

    #[test]
    fn output() -> Result<(), Error> {
        let pattern = Pattern::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        )?;
        assert_eq!(pattern.output()?, 5353);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(super::part_1(&INPUT.try_from_lines()?), 26);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(super::part_2(&INPUT.try_from_lines()?)?, 61229);
        Ok(())
    }
}
