use dashu::integer::UBig;
use std::{collections::BTreeMap, ops::AddAssign};

#[derive(Clone)]
pub enum Stone {
    Split { left: UBig, right: UBig },
    Single(UBig),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let input = std::fs::read_to_string(arg)?;
    let mut memo = BTreeMap::new();
    let mut numbers: BTreeMap<UBig, u64> = BTreeMap::new();
    for n in input.trim().split_whitespace() {
        let n = UBig::from_str_radix(n, 10)?;
        numbers.entry(n).or_default().add_assign(1u64);
    }

    for _ in 0..75 {
        let mut next_layer: BTreeMap<UBig, u64> = BTreeMap::new();
        for (number, count) in numbers {
            let precomputed = memo.get(&number).cloned().unwrap_or_else(|| {
                if number == UBig::ZERO {
                    return Stone::Single(UBig::ONE);
                }
                let nstr = number.to_string();
                let digits = nstr.len();
                let uncached = if digits & 1 == 0 {
                    // `number` has an even number of digits
                    let (left, right) = nstr.split_at(digits / 2);
                    let left = UBig::from_str_radix(left, 10).unwrap_or_default();
                    let right = UBig::from_str_radix(right, 10).unwrap_or_default();
                    Stone::Split { left, right }
                } else {
                    Stone::Single(number.clone() * 2024u16)
                };
                memo.insert(number, uncached.clone());
                uncached
            });
            match precomputed {
                Stone::Split { left, right } => {
                    *next_layer.entry(left.clone()).or_default() += count;
                    *next_layer.entry(right.clone()).or_default() += count;
                }
                Stone::Single(val) => {
                    *next_layer.entry(val.clone()).or_default() += count;
                }
            }
        }

        numbers = next_layer;
    }
    let sum: u64 = numbers.values().sum();
    println!("{:?}", sum);
    Ok(())
}
