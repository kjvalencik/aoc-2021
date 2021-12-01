use std::io;

use anyhow::Error;

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

    println!("Part 1: {}", solution(&nums, 2));
    println!("Part 2: {}", solution(&nums, 4));

    Ok(())
}
