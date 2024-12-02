fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part_1 = 0;
    let mut part_2 = 0;

    for line in std::io::stdin().lines() {
        let report = line?;

        let report = report
            .split(' ')
            .map(|level| level.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_report_safe(report.iter().copied()) {
            part_1 += 1;
            part_2 += 1;
        } else {
            for i in 0..report.len() {
                if is_report_safe(
                    report
                        .iter()
                        .copied()
                        .enumerate()
                        .filter_map(|(idx, e)| (idx != i).then_some(e)),
                ) {
                    part_2 += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn is_report_safe(report: impl IntoIterator<Item = u32>) -> bool {
    let mut prev: Option<u32> = None;
    let mut increasing = None;

    for level in report {
        if let Some(p) = prev {
            let d = p.abs_diff(level);
            let trend = level > p;

            if d == 0 || d > 3 || matches!(increasing, Some(i) if i^trend) {
                return false;
            }
            increasing = Some(trend);
        }

        prev = Some(level);
    }

    true
}
