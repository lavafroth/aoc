use dashu::integer::UBig;
use std::collections::BTreeMap;

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
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|n| UBig::from_str_radix(n, 10))
        .collect::<Result<Vec<_>, _>>()?;

    let mut popo: BTreeMap<UBig, u64> = BTreeMap::new();
    for n in numbers {
        *popo.entry(n).or_default() += 1u64;
    }

    let mut numbers = popo;

    for _ in 0..75 {
        let mut out: BTreeMap<UBig, u64> = BTreeMap::new();
        for (number, count) in numbers {
            let precomputed = memo.get(&number).cloned().unwrap_or_else(|| {
                let mut momo = Stone::Single(UBig::ONE);
                if number != UBig::ZERO {
                    let nstr = number.to_string();
                    let digi = nstr.len();
                    if digi % 2 == 0 {
                        let left = UBig::from_str_radix(&nstr[..digi / 2], 10).unwrap();
                        let right = UBig::from_str_radix(&nstr[digi / 2..], 10).unwrap();
                        momo = Stone::Split { left, right };
                        memo.insert(number, momo.clone());
                        // it is even
                    } else {
                        let val = number.clone() * 2024u16;
                        momo = Stone::Single(val);
                        memo.insert(number, momo.clone());
                    }
                }
                momo
            });
            match precomputed {
                Stone::Split { left, right } => {
                    *out.entry(left.clone()).or_default() += count;
                    *out.entry(right.clone()).or_default() += count;
                }
                Stone::Single(val) => {
                    *out.entry(val.clone()).or_default() += count;
                }
            }
        }

        numbers = out;
    }
    let mut sum = 0;
    for (_, count) in numbers {
        sum += count;
    }
    println!("{:?}", sum);
    Ok(())
}
