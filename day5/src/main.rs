fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::stdin().lines();

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    while let Some(line) = input.next() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        rules.push(line.parse::<Rule>()?);
    }

    for line in input {
        updates.push(line?.parse::<Update>()?);
    }

    let mut part_1 = 0;
    let mut part_2 = 0;

    'updates: for update in &updates {
        let len = update.pages.len();

        for (preceding_page, subsequent_page) in (0..len).flat_map(|this| {
            (this + 1..len).map(move |other| (update.pages[this], update.pages[other]))
        }) {
            // find a rule that disallows this ordering
            for rule in &rules {
                if rule.a == subsequent_page && rule.b == preceding_page {
                    let sorted_update = update.sort(&rules);
                    part_2 += sorted_update.middle_page();

                    continue 'updates;
                }
            }
        }

        part_1 += update.middle_page();
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

struct Rule {
    a: u32,
    b: u32,
}

impl std::str::FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('|').ok_or(Self::Err::Split)?;
        let a = a.parse().map_err(Self::Err::ParseInt)?;
        let b = b.parse().map_err(Self::Err::ParseInt)?;
        Ok(Self { a, b })
    }
}

#[derive(Debug)]
enum ParseError {
    Split,
    ParseInt(std::num::ParseIntError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Split => f.write_str("could not split string"),
            ParseError::ParseInt(e) => write!(f, "failed to parse string as integer: {e}"),
        }
    }
}

impl std::error::Error for ParseError {}

struct Update {
    pages: Vec<u32>,
}

impl std::str::FromStr for Update {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pages: s
                .split(',')
                .map(|p| p.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(Self::Err::ParseInt)?,
        })
    }
}

impl Update {
    fn middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    fn sort(&self, rules: &[Rule]) -> Self {
        let mut pages = self.pages.clone();
        let len = pages.len();

        for a_idx in 0..len - 1 {
            for b_idx in a_idx..len {
                let a = pages[a_idx];
                let b = pages[b_idx];

                for rule in rules {
                    if rule.b == a && rule.a == b {
                        pages.swap(a_idx, b_idx);
                    }
                }
            }
        }

        Self { pages }
    }
}
