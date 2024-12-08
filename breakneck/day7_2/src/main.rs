use std::io::stdin;

fn preorder(res: u64, values: &[u64]) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    (values.len() == 1 && res == top)
        || (res % top == 0 && preorder(res / top, &values[1..]))
        || (res > top && preorder(res - top, &values[1..]))
        || res
            .checked_sub(top)
            .zip(top.checked_ilog10().map(|log10| 10u64.pow(log10 + 1)))
            .and_then(|(delta, power_of_10)| {
                (delta % power_of_10 == 0).then_some(preorder(delta / power_of_10, &values[1..]))
            })
            .unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0;
    for line in stdin().lines() {
        let line = line?;
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
        if !preorder(r, &values) {
            continue;
        };

        sum += r;
    }
    println!("calibrated to {sum}");
    Ok(())
}
