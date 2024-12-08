use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::from_stdin()?;

    let mut part_1 = HashSet::new();
    let mut part_2 = HashSet::new();

    for frequency in map.frequencies.values_mut() {
        frequency.find_antinodes(&map.size);

        part_1.extend(frequency.part_1.iter().copied());

        part_2.extend(frequency.part_1.iter().copied());
        part_2.extend(frequency.part_2.iter().copied());
    }

    println!("Part 1: {}", part_1.len());
    println!("Part 1: {}", part_2.len());

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl Vec2<isize> {
    fn is_inside(&self, size: &Vec2<usize>) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }

        (self.x as usize) < size.x && (self.y as usize) < size.y
    }
}

impl<T: Add> Add for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub> Sub for Vec2<T> {
    type Output = Vec2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Map {
    size: Vec2<usize>,
    frequencies: HashMap<char, Frequency>,
}

impl Map {
    fn from_stdin() -> std::io::Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut frequencies: HashMap<char, Frequency> = HashMap::new();

        for line in std::io::stdin().lines() {
            let line = line?;
            width = width.max(line.len());

            for (i, c) in line.char_indices() {
                if c.is_ascii_alphanumeric() {
                    frequencies.entry(c).or_default().antennae.push(Vec2 {
                        x: i as isize,
                        y: height as isize,
                    });
                }
            }

            height += 1;
        }

        Ok(Self {
            size: Vec2 {
                x: width,
                y: height,
            },
            frequencies,
        })
    }
}

#[derive(Default)]
struct Frequency {
    antennae: Vec<Vec2<isize>>,
    part_1: HashSet<Vec2<isize>>,
    part_2: HashSet<Vec2<isize>>,
}

impl Frequency {
    fn find_antinodes(&mut self, size: &Vec2<usize>) {
        for (first, second) in self
            .antennae
            .iter()
            .enumerate()
            .flat_map(|(first_i, first)| {
                self.antennae[first_i + 1..]
                    .iter()
                    .map(move |second| (first, second))
            })
        {
            let delta = *second - *first;

            let first_antinode = *first - delta;
            let second_antinode = *second + delta;

            if first_antinode.is_inside(size) {
                self.part_1.insert(first_antinode);
            }

            if second_antinode.is_inside(size) {
                self.part_1.insert(second_antinode);
            }

            self.part_2.insert(*first);
            self.part_2.insert(*second);

            let mut w = first_antinode;
            while {
                w = w - delta;
                w.is_inside(size)
            } {
                self.part_2.insert(w);
            }

            let mut w = second_antinode;
            while {
                w = w + delta;
                w.is_inside(size)
            } {
                self.part_2.insert(w);
            }
        }
    }
}
