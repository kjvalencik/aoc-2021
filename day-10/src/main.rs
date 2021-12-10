use util::*;

#[derive(Debug)]
struct Puzzle {
    lines: Vec<String>,
}

#[derive(Debug)]
enum NavError {
    Corrupted(char),
    Incomplete(Vec<char>),
    Unknown(char),
}

impl fmt::Display for NavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl error::Error for NavError {}

fn corrupt_char_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn incomplete_char_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn is_match(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => false,
    }
}

fn check_line(line: &str) -> Result<(), NavError> {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            ')' | ']' | '}' | '>' => {
                if !stack.pop().map(|open| is_match(open, c)).unwrap_or(false) {
                    return Err(NavError::Corrupted(c));
                }
            }
            c => return Err(NavError::Unknown(c)),
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(NavError::Incomplete(stack))
    }
}

fn score_incomplete(stack: Vec<char>) -> u64 {
    stack
        .into_iter()
        .rev()
        .map(incomplete_char_score)
        .fold(0, |score, x| score * 5 + x)
}

impl Puzzle {
    fn part_1(&self) -> u64 {
        self.lines
            .iter()
            .filter_map(|line| match check_line(line) {
                Err(NavError::Corrupted(c)) => Some(corrupt_char_score(c)),
                _ => None,
            })
            .sum()
    }

    fn part_2(&self) -> Result<u64, Error> {
        let mut scores = self
            .lines
            .iter()
            .filter_map(|line| match check_line(line) {
                Err(NavError::Incomplete(stack)) => Some(score_incomplete(stack)),
                _ => None,
            })
            .collect::<Vec<_>>();

        if scores.is_empty() {
            return Err(Error::msg("No incomplete lines"));
        }

        scores.sort_unstable();

        Ok(scores[scores.len() / 2])
    }
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let lines = input
            .trim()
            .lines()
            .map(|line| String::from(line.trim()))
            .collect();

        Self { lines }
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from(read_stdin()?.as_str());

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 2: {}", puzzle.part_2()?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = r#"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from(INPUT).part_1(), 26397);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from(INPUT).part_2()?, 288957);
        Ok(())
    }
}
