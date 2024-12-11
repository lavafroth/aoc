fn preorder(res: u64, values: &[u64], pipe: bool) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    (values.len() == 1 && res == top)
        || (res % top == 0 && preorder(res / top, &values[1..], pipe))
        || (res > top && preorder(res - top, &values[1..], pipe))
        || (pipe
            && res
                .checked_sub(top)
                .zip(top.checked_ilog10().map(|log10| 10u64.pow(log10 + 1)))
                .and_then(|(delta, power_of_10)| {
                    (delta % power_of_10 == 0).then_some(preorder(
                        delta / power_of_10,
                        &values[1..],
                        pipe,
                    ))
                })
                .unwrap_or_default())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };

    let input = std::fs::read_to_string(arg)?;
    let mut lines: Vec<_> = vec![];
    for line in input.trim().lines() {
        if line.is_empty() {
            break;
        }
        let (r, vals) = line.split_once(":").expect("bad line");
        let r: u64 = r.trim().parse()?;
        let values = vals
            .trim()
            .split(' ')
            .map(|v| v.trim().parse())
            .rev()
            .collect::<Result<Vec<u64>, _>>()?;
        lines.push((r, values));
    }

    let sum: u64 = lines
        .iter()
        .filter(|(r, v)| preorder(*r, v, false))
        .map(|(r, _)| r)
        .sum();
    println!("[part 1] {sum}");

    let sum: u64 = lines
        .iter()
        .filter(|(r, v)| preorder(*r, v, true))
        .map(|(r, _)| r)
        .sum();
    println!("[part 2] {sum}");
    Ok(())
}
