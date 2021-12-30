use util::*;

#[derive(Clone, Debug)]
struct Alu {
    registers: [isize; 4],
    instructions: Vec<Instruction>,
}

#[derive(Clone, Debug)]
enum Instruction {
    Input(Variable),
    Add(Variable, Argument),
    Multiply(Variable, Argument),
    Divide(Variable, Argument),
    Modulo(Variable, Argument),
    Equal(Variable, Argument),
}

#[derive(Clone, Copy, Debug)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
enum Argument {
    Value(isize),
    Variable(Variable),
}

impl FromStr for Alu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s.trim().try_from_lines()?;

        Ok(Self {
            instructions,
            registers: [0; 4],
        })
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, args) = s.trim().split_once(' ').context("Missing arguments")?;

        if instruction == "inp" {
            return Ok(Self::Input(Variable::from_str(args)?));
        }

        let (variable, argument) = args.trim().split_once(' ').context("Missing argument")?;
        let (variable, argument) = (Variable::from_str(variable)?, Argument::from_str(argument)?);

        Ok(match instruction {
            "add" => Self::Add(variable, argument),
            "mul" => Self::Multiply(variable, argument),
            "div" => Self::Divide(variable, argument),
            "mod" => Self::Modulo(variable, argument),
            "eql" => Self::Equal(variable, argument),
            _ => bail!("Invalid instruction"),
        })
    }
}

impl FromStr for Variable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "w" => Self::W,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            v => bail!("Invalid variable: {}", v),
        })
    }
}

impl FromStr for Argument {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = Variable::from_str(s) {
            return Ok(Self::Variable(v));
        }

        Ok(Self::Value(isize::from_str(s.trim())?))
    }
}

impl Alu {
    fn variable(&self, v: &Variable) -> isize {
        match v {
            Variable::W => self.registers[0],
            Variable::X => self.registers[1],
            Variable::Y => self.registers[2],
            Variable::Z => self.registers[3],
        }
    }

    fn argument(&self, a: &Argument) -> isize {
        match a {
            Argument::Value(v) => *v,
            Argument::Variable(v) => self.variable(v),
        }
    }

    fn set_variable(&mut self, v: &Variable, n: isize) {
        match v {
            Variable::W => self.registers[0] = n,
            Variable::X => self.registers[1] = n,
            Variable::Y => self.registers[2] = n,
            Variable::Z => self.registers[3] = n,
        }
    }

    fn exec(&self, input: isize) -> Result<isize, Error> {
        let mut alu = self.clone();
        let mut input = input
            .to_string()
            .chars()
            .rev()
            .map(|c| -> Result<isize, Error> {
                Ok(match c {
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => bail!("Invalid input"),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        if input.len() != 14 {
            bail!("Invalid input");
        }

        for instruction in &self.instructions {
            match instruction {
                Instruction::Input(v) => {
                    let input = input.pop().context("Out of input")?;

                    alu.set_variable(v, input);
                }
                Instruction::Add(v, a) => {
                    let x = alu.variable(v);
                    let y = alu.argument(a);

                    alu.set_variable(v, x + y);
                }
                Instruction::Multiply(v, a) => {
                    let x = alu.variable(v);
                    let y = alu.argument(a);

                    alu.set_variable(v, x * y);
                }
                Instruction::Divide(v, a) => {
                    let x = alu.variable(v);
                    let y = alu.argument(a);

                    alu.set_variable(v, x / y);
                }
                Instruction::Modulo(v, a) => {
                    let x = alu.variable(v);
                    let y = alu.argument(a);

                    alu.set_variable(v, x % y);
                }
                Instruction::Equal(v, a) => {
                    let x = alu.variable(v);
                    let y = alu.argument(a);

                    alu.set_variable(v, if x == y { 1 } else { 0 });
                }
            }
        }

        Ok(alu.variable(&Variable::Z))
    }

    // This is from hand solving the problem and converting to code.
    // It works for my input, but might not work for all. There are some simple checks
    // that the data matches expectations, but YMMV.
    fn solve(&self, is_min: bool) -> Result<isize, Error> {
        const DIGITS: usize = 14;

        if self.instructions.len() % DIGITS != 0 {
            bail!("Expected matching chunk sizes");
        }

        let chunk_size = self.instructions.len() / DIGITS;
        let mut model = vec![0; 14];
        let mut stack = vec![];

        if chunk_size < 17 {
            bail!("Expected minimum size chunks");
        }

        for (i, section) in self.instructions.chunks(chunk_size).enumerate() {
            if !matches!(section[0], Instruction::Input(_)) {
                bail!("Expected section to start with an input");
            }

            let divisor = match &section[4] {
                Instruction::Divide(Variable::Z, d) => d,
                i => bail!("Unexpected instruction: {:?}", i),
            };

            match divisor {
                Argument::Value(1) => {
                    let v = match &section[15] {
                        Instruction::Add(Variable::Y, Argument::Value(v)) => v,
                        i => bail!("Unexpected instruction: {:?}", i),
                    };

                    stack.push((i, v));
                }
                Argument::Value(26) => {
                    let (j, v) = stack.pop().context("Empty stack")?;
                    let x = v + match &section[5] {
                        Instruction::Add(Variable::X, Argument::Value(v)) => v,
                        i => bail!("Unexpected instruction: {:?}", i),
                    };

                    let (i, j, x) = if x >= 0 { (i, j, x) } else { (j, i, -x) };

                    if is_min {
                        model[i] = 1 + x;
                        model[j] = 1;
                    } else {
                        model[i] = 9;
                        model[j] = 9 - x;
                    }
                }
                a => bail!("Unexpected argument: {:?}", a),
            }
        }

        let model = model.into_iter().fold(0, |y, x| y * 10 + x);

        if self.exec(model)? != 0 {
            bail!("Incorrect solution");
        }

        Ok(model)
    }

    fn part_1(&self) -> Result<isize, Error> {
        self.solve(false)
    }

    fn part_2(&self) -> Result<isize, Error> {
        self.solve(true)
    }
}

fn main() -> Result<(), Error> {
    let alu = Alu::from_str(&read_stdin()?)?;

    println!("Part 1: {}", alu.part_1()?);
    println!("Part 2: {}", alu.part_2()?);

    Ok(())
}
