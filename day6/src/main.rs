fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lab = Lab::from_stdin()?;
    let mut lab_pt1 = lab.clone();

    let mut part_1 = 1;
    loop {
        match lab_pt1.advance() {
            AdvanceResult::NewPosition => part_1 += 1,
            AdvanceResult::AlreadyVisited => {}
            AdvanceResult::Stuck => unreachable!(),
            AdvanceResult::OutOfBounds => break,
        }
    }

    println!("Part 1: {part_1}");

    let mut part_2 = 0;
    'positions: for (x, y) in (0..lab.width)
        .flat_map(|x| (0..lab.data.len() / lab.width).map(move |y| (x, y)))
        .filter(|(x, y)| matches!(lab.cell(*x, *y), Position::Empty))
    {
        let mut lab = lab.clone();
        *lab.cell_mut(x, y) = Position::Obstructed;

        loop {
            match lab.advance() {
                AdvanceResult::NewPosition | AdvanceResult::AlreadyVisited => {}
                AdvanceResult::Stuck => {
                    part_2 += 1;
                    continue 'positions;
                }
                AdvanceResult::OutOfBounds => continue 'positions,
            }
        }
    }

    println!("Part 2: {part_2}");

    Ok(())
}

#[derive(Clone, Copy)]
enum Position {
    Empty,
    Visited(Direction),
    Obstructed,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[derive(Clone)]
struct Lab {
    position: (usize, usize),
    direction: Direction,
    width: usize,
    data: Box<[Position]>,
}

impl Lab {
    fn from_stdin() -> Result<Self, std::io::Error> {
        let mut position = (0, 0);
        let mut width = 0;
        let mut data = Vec::new();

        for (line_nr, line_text) in std::io::stdin().lines().enumerate() {
            let line_text = line_text?;
            width = width.max(line_text.len());

            for (column_nr, c) in line_text.chars().enumerate() {
                data.push(match c {
                    '#' => Position::Obstructed,
                    '^' => {
                        position = (column_nr, line_nr);
                        Position::Visited(Direction::Up)
                    }
                    _ => Position::Empty,
                });
            }
        }

        Ok(Lab {
            position,
            direction: Direction::Up,
            width,
            data: data.into_boxed_slice(),
        })
    }

    fn next_position(&self) -> Option<(usize, usize)> {
        let (x, y) = self.position;

        match &self.direction {
            Direction::Up => {
                let y = y.checked_sub(1)?;
                Some((x, y))
            }
            Direction::Right => {
                if x == self.width {
                    return None;
                }
                Some((x + 1, y))
            }
            Direction::Down => {
                if (y + 1) * self.width >= self.data.len() {
                    return None;
                }
                Some((x, y + 1))
            }
            Direction::Left => {
                let x = x.checked_sub(1)?;
                Some((x, y))
            }
        }
    }

    fn cell(&self, x: usize, y: usize) -> Position {
        self.data[y * self.width + x]
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> &mut Position {
        &mut self.data[y * self.width + x]
    }

    fn advance(&mut self) -> AdvanceResult {
        while let Some((x, y)) = self.next_position() {
            match self.cell(x, y) {
                Position::Empty => {
                    *self.cell_mut(x, y) = Position::Visited(self.direction.clone());
                    self.position = (x, y);
                    return AdvanceResult::NewPosition;
                }
                Position::Visited(prev_direction) => {
                    self.position = (x, y);

                    if prev_direction == self.direction
                        && self
                            .next_position()
                            .is_some_and(|(x, y)| matches!(self.cell(x, y), Position::Obstructed))
                    {
                        return AdvanceResult::Stuck;
                    }

                    return AdvanceResult::AlreadyVisited;
                }
                Position::Obstructed => {
                    self.direction.turn();
                }
            }
        }

        AdvanceResult::OutOfBounds
    }
}

enum AdvanceResult {
    NewPosition,
    AlreadyVisited,
    Stuck,
    OutOfBounds,
}
