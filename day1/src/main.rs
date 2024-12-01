use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut right_frequency = std::collections::HashMap::<_, u32>::new();

    for line in io::stdin().lines() {
        let line = line?;
        let (l, r) = line.split_once(' ').unwrap();

        let l = l.trim().parse::<u32>()?;
        let r = r.trim().parse::<u32>()?;

        left.push(l);
        right.push(r);

        *right_frequency.entry(r).or_default() += 1;
    }

    left.sort();
    right.sort();

    let mut diff = 0;
    let mut sim = 0;

    for (l, r) in left.iter().zip(right) {
        diff += l.abs_diff(r);
        sim += l * right_frequency.get(l).map_or(0, |f| *f);
    }

    println!("Difference: {diff}");
    println!("Similarity: {sim}");

    Ok(())
}
