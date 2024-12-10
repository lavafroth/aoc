use std::collections::BTreeMap;
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let input = std::fs::read_to_string(arg)?;
    let lines: Vec<_> = input.lines().collect();
    let cap = lines.len();

    // part 1
    let mut left: BinaryHeap<i64> = BinaryHeap::with_capacity(cap);
    let mut right: BinaryHeap<i64> = BinaryHeap::with_capacity(cap);
    for line in &lines {
        let Some((a, b)) = line.trim().split_once("   ") else {
            eprintln!("erroneous line: {line:?}");
            return Ok(());
        };
        left.push(a.parse()?);
        right.push(b.parse()?);
    }
    let mut delta = 0i64;
    while let (Some(a), Some(b)) = (left.pop(), right.pop()) {
        delta += (a - b).abs();
    }
    println!("[part 1] delta = {delta}");

    // part 2
    let mut uniq: BTreeMap<i64, i64> = BTreeMap::default();
    let mut right: Vec<i64> = Vec::with_capacity(cap);
    let mut left: Vec<i64> = Vec::with_capacity(cap);
    for line in &lines {
        let Some((a, b)) = line.trim().split_once("   ") else {
            eprintln!("erroneous line: {line:?}");
            return Ok(());
        };
        uniq.insert(a.parse()?, 0);
        left.push(a.parse()?);
        right.push(b.parse()?);
    }
    for r in right {
        uniq.entry(r).and_modify(|e| *e += 1);
    }

    let score: i64 = left
        .iter()
        .map(|l| l * uniq.get(l).cloned().unwrap_or_default())
        .sum();
    println!("[part 2] score = {score}");

    Ok(())
}
