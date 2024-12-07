fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut equations = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        equations.push(line.parse::<Equation>()?);
    }

    let mut part_1 = 0;
    'equations: for eqn in &equations {
        for operators in Operator::permute_part_1(eqn.operands.len() as u32 - 1) {
            if eqn.is_valid(operators) {
                part_1 += eqn.value;
                continue 'equations;
            }
        }
    }

    println!("Part 1: {part_1}");

    let mut part_2 = 0;
    'equations: for eqn in &equations {
        for operators in Operator::permute_part_2(eqn.operands.len() as u32 - 1) {
            if eqn.is_valid(operators) {
                part_2 += eqn.value;
                continue 'equations;
            }
        }
    }

    println!("Part 2: {part_2}");

    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn permute_part_1(length: u32) -> OperatorPart1Permutation {
        OperatorPart1Permutation {
            length,
            variation: 0,
        }
    }

    fn permute_part_2(length: u32) -> OperatorPart2Permutation {
        OperatorPart2Permutation {
            length,
            variation: 0,
        }
    }
}

struct OperatorPart1Permutation {
    length: u32,
    variation: u64,
}

impl Iterator for OperatorPart1Permutation {
    type Item = OperatorPart1Iter;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.variation >> self.length) != 0 {
            return None;
        }

        let data = self.variation;
        self.variation += 1;

        Some(OperatorPart1Iter {
            length: self.length,
            data,
            position: 0,
        })
    }
}

#[derive(Debug)]
struct OperatorPart1Iter {
    length: u32,
    data: u64,
    position: u32,
}

impl Iterator for OperatorPart1Iter {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.length {
            return None;
        }

        let msk = 1 << self.position;

        let op = if self.data & msk == msk {
            Operator::Multiply
        } else {
            Operator::Add
        };

        self.position += 1;

        Some(op)
    }
}

#[derive(Debug)]
struct OperatorPart2Permutation {
    length: u32,
    variation: u64,
}

impl Iterator for OperatorPart2Permutation {
    type Item = OperatorPart2Iter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.variation >= 3u64.pow(self.length) {
            return None;
        }

        let data = self.variation;
        self.variation += 1;

        Some(OperatorPart2Iter {
            step: self.length,
            data,
        })
    }
}

#[derive(Debug)]
struct OperatorPart2Iter {
    step: u32,
    data: u64,
}

impl Iterator for OperatorPart2Iter {
    type Item = Operator;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step == 0 {
            return None;
        }

        self.step -= 1;

        let op = match self.data % 3 {
            0 => Operator::Add,
            1 => Operator::Multiply,
            2 => Operator::Concatenate,
            n => unreachable!("self.data % 3 yielded {n}"),
        };

        self.data /= 3;

        Some(op)
    }
}

#[derive(Debug)]
struct Equation {
    value: u64,
    operands: Box<[u64]>,
}

impl std::str::FromStr for Equation {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, rest) = s.split_once(": ").ok_or("no colon character")?;
        let value = value.parse()?;

        Ok(Self {
            value,
            operands: rest.split(' ').map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

impl Equation {
    fn is_valid(&self, operators: impl IntoIterator<Item = Operator>) -> bool {
        eval(&self.operands, operators) == self.value
    }
}

fn eval(operands: &[u64], operators: impl IntoIterator<Item = Operator>) -> u64 {
    let mut iter = operands.iter().copied();
    let Some(mut output) = iter.next() else {
        return 0;
    };

    for (n, o) in iter.zip(operators) {
        match o {
            Operator::Add => output += n,
            Operator::Multiply => output *= n,
            Operator::Concatenate => {
                let log = n.ilog10();
                let mut shift = 10u64.pow(log);
                shift *= 10;
                output = output * shift + n;
            }
        }
    }

    output
}
