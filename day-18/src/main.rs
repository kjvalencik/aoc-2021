use serde::Deserialize;
use util::*;

#[derive(Debug)]
struct Puzzle {
    nums: Vec<Value>,
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.try_from_lines()?))
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum Value {
    Num(u32),
    Pair(Box<(Value, Value)>),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(input)?)
    }
}

impl Value {
    fn find_explosion(&mut self, depth: usize, i: &mut usize) -> Result<Option<(u32, u32)>, Error> {
        let pair = match self {
            Value::Num(_) => {
                *i += 1;
                return Ok(None);
            }
            Value::Pair(pair) => pair.as_mut(),
        };

        if depth >= 4 {
            match pair {
                (Value::Num(l), Value::Num(r)) => {
                    let pair = (*l, *r);
                    *self = Self::Num(0);
                    return Ok(Some(pair));
                }
                _ => bail!("Expected two numbers"),
            }
        }

        if let Some(result) = pair.0.find_explosion(depth + 1, i)? {
            return Ok(Some(result));
        }

        pair.1.find_explosion(depth + 1, i)
    }

    fn explode(&mut self, target: usize, v: (u32, u32), i: &mut usize) {
        let pair = match self {
            Value::Num(n) => {
                *i += 1;

                if *i == target {
                    *n += v.0;
                } else if *i == target + 2 {
                    *n += v.1;
                }

                return;
            }
            Value::Pair(pair) => pair.as_mut(),
        };

        pair.0.explode(target, v, i);
        pair.1.explode(target, v, i);
    }

    fn split(&mut self) -> bool {
        let pair = match self {
            Value::Num(n) if *n >= 10 => {
                let n = *n;
                let l = Self::Num(n / 2);
                let r = Self::Num((n / 2) + (n % 2));
                *self = Self::Pair(Box::new((l, r)));
                return true;
            }
            Value::Pair(pair) => pair.as_mut(),
            _ => return false,
        };

        if pair.0.split() {
            return true;
        }

        pair.1.split()
    }

    fn step(&mut self) -> Result<bool, Error> {
        let mut i = 0;

        if let Some(pair) = self.find_explosion(0, &mut i)? {
            let mut j = 0;
            self.explode(i, pair, &mut j);
            return Ok(true);
        };

        Ok(self.split())
    }

    fn reduce(&mut self) -> Result<(), Error> {
        while self.step()? {}
        Ok(())
    }

    fn magnitude(&self) -> u32 {
        match self {
            Value::Num(n) => *n,
            Value::Pair(pair) => 3 * pair.as_ref().0.magnitude() + 2 * pair.as_ref().1.magnitude(),
        }
    }
}

impl Puzzle {
    fn new(nums: Vec<Value>) -> Self {
        Self { nums }
    }

    fn part_1(&self) -> Result<u32, Error> {
        let mut nums = self.nums.clone().into_iter();
        let mut left = nums.next().context("Expected a number")?;

        for right in nums {
            left = Value::Pair(Box::new((left, right)));
            left.reduce()?;
        }

        Ok(left.magnitude())
    }

    fn part_2(&self) -> Result<u32, Error> {
        self.nums
            .iter()
            .enumerate()
            .flat_map(|(i, x)| self.nums.iter().enumerate().map(move |(j, y)| (i, j, x, y)))
            .filter(|(i, j, _, _)| i != j)
            .flat_map(|(_, _, x, y)| {
                [
                    Puzzle::new(vec![x.clone(), y.clone()]),
                    Puzzle::new(vec![y.clone(), x.clone()]),
                ]
            })
            .filter_map(|p| p.part_1().ok())
            .max()
            .context("Missing input")
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
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "#;

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_1()?, 4140);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str(INPUT)?.part_2()?, 3993);
        Ok(())
    }
}
