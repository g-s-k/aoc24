use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut input = input.as_str();

    let mut part_1 = 0;

    let mut enabled = true;
    let mut part_2 = 0;

    while !input.is_empty() {
        if parse_do(&mut input) {
            enabled = true;
            continue;
        }

        if parse_dont(&mut input) {
            enabled = false;
            continue;
        }

        if let Some(mul) = parse_mul(&mut input) {
            let product = mul.x * mul.y;

            part_1 += product;
            if enabled {
                part_2 += product;
            }

            continue;
        }

        input = &input[1..];
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn parse_do(text: &mut &str) -> bool {
    let Some(rest) = text.strip_prefix("do()") else {
        return false;
    };
    *text = rest;
    true
}

fn parse_dont(text: &mut &str) -> bool {
    let Some(rest) = text.strip_prefix("don't()") else {
        return false;
    };
    *text = rest;
    true
}

fn parse_mul(text: &mut &str) -> Option<Mul> {
    let Some(rest) = text.strip_prefix("mul(") else {
        return None;
    };
    *text = rest;

    let Some(x) = parse_number(text) else {
        return None;
    };

    let Some(rest) = text.strip_prefix(",") else {
        return None;
    };
    *text = rest;

    let Some(y) = parse_number(text) else {
        return None;
    };

    let Some(rest) = text.strip_prefix(")") else {
        return None;
    };
    *text = rest;

    Some(Mul { x, y })
}

struct Mul {
    x: u32,
    y: u32,
}

fn parse_number(text: &mut &str) -> Option<u32> {
    let mut num = 0;

    let mut stop = 0;

    for (i, c) in text.char_indices().take(3) {
        if c.is_ascii_digit() {
            num = num * 10 + c as u32 - '0' as u32;
            stop = i + 1;
        } else {
            break;
        }
    }

    if stop == 0 {
        return None;
    }

    *text = &text[stop..];

    Some(num)
}
