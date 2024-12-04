fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut part_1 = 0;

    // get input and check horizontal matches
    for line in std::io::stdin().lines() {
        let line = line?;
        part_1 += line_xmas(line.chars());
        input.push(line.chars().collect());
    }

    let height = input.len();
    let width = input[0].len();

    // check vertical matches
    for column in 0..width {
        part_1 += line_xmas(input.iter().map(|line| line[column]));
    }

    // check downward diagonal matches
    for column in 1..width - 3 {
        part_1 += line_xmas((column..width).zip(0..height).map(|(x, y)| input[y][x]));
    }
    for row in 0..height - 3 {
        part_1 += line_xmas((0..width).zip(row..height).map(|(x, y)| input[y][x]));
    }

    // check upward diagonal matches
    for column in 3..width - 1 {
        part_1 += line_xmas((0..=column).rev().zip(0..height).map(|(x, y)| input[y][x]));
    }
    for row in 0..height - 3 {
        part_1 += line_xmas((0..width).rev().zip(row..height).map(|(x, y)| input[y][x]));
    }

    let mut part_2 = 0;

    for line in 1..height - 1 {
        'x: for column in 1..width - 1 {
            if input[line][column] != 'A' {
                continue 'x;
            }

            let top_left = input[line - 1][column - 1];
            let top_right = input[line - 1][column + 1];
            let bottom_left = input[line + 1][column - 1];
            let bottom_right = input[line + 1][column + 1];

            match (top_left, top_right, bottom_left, bottom_right) {
                ('M', 'M', 'S', 'S')
                | ('M', 'S', 'M', 'S')
                | ('S', 'M', 'S', 'M')
                | ('S', 'S', 'M', 'M') => {
                    part_2 += 1;
                }
                _ => {}
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn line_xmas<I>(line: I) -> usize
where
    I: IntoIterator<Item = char>,
    <I as IntoIterator>::IntoIter: Clone,
{
    let mut count = 0;
    let iter = line.into_iter();

    let mut i = iter.clone().peekable();
    while i.peek().is_some() {
        if find_xmas(&mut i).is_some() {
            count += 1;
        }
    }

    let mut i = iter.clone().peekable();
    while i.peek().is_some() {
        if find_samx(&mut i).is_some() {
            count += 1;
        }
    }

    count
}

fn find_xmas<I>(iter: &mut std::iter::Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = char>,
{
    let pos = iter.position(|c| c == 'X')?;

    let 'M' = iter.peek()? else { return None };
    iter.next()?;
    let 'A' = iter.peek()? else { return None };
    iter.next()?;
    let 'S' = iter.peek()? else { return None };
    iter.next()?;

    Some(pos)
}

fn find_samx<I>(iter: &mut std::iter::Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = char>,
{
    let pos = iter.position(|c| c == 'S')?;

    let 'A' = iter.peek()? else { return None };
    iter.next()?;
    let 'M' = iter.peek()? else { return None };
    iter.next()?;
    let 'X' = iter.peek()? else { return None };
    iter.next()?;

    Some(pos)
}
