use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Slope {
    Zero,
    Some { numer: u64, denom: u64 },
    Inf,
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map: Vec<Vec<_>> = std::fs::read_to_string("input")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let first_row = map.first().expect("what map is this anyway");
    let y_max = map.len();
    let x_max = first_row.len();
    let mut slopes = HashSet::new();
    slopes.insert(Slope::Zero);
    slopes.insert(Slope::Inf);
    for x in 1..x_max {
        for y in 1..y_max {
            let x = x as u64;
            let y = y as u64;
            if gcd(x, y) != 1 {
                continue;
            }
            let slope = Slope::Some { numer: y, denom: x };
            slopes.insert(slope);
            println!("slope: {:?}", slope);
        }
    }
    Ok(())
}
