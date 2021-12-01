use std::io;

use anyhow::Error;

const PART_1_WINDOW: usize = 2;
const PART_2_WINDOW: usize = 4;

fn read_stdin() -> Result<String, Error> {
    let mut buf = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut buf)?;
    Ok(buf)
}

fn solution(nums: &[i64], n: usize) -> usize {
    nums.windows(n).filter(|w| w[n - 1] > w[0]).count()
}

fn main() -> Result<(), Error> {
    let nums = read_stdin()?
        .trim()
        .lines()
        .map(|line| Ok(line.trim().parse()?))
        .collect::<Result<Vec<i64>, Error>>()?;

    println!("Part 1: {}", solution(&nums, PART_1_WINDOW));
    println!("Part 2: {}", solution(&nums, PART_2_WINDOW));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &[i64] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1() {
        assert_eq!(solution(INPUT, PART_1_WINDOW), 7);
    }

    #[test]
    fn part_2() {
        assert_eq!(solution(INPUT, PART_2_WINDOW), 5);
    }
}
