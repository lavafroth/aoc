use std::io::stdin;

fn preorder(res: u64, values: &[u64]) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    (values.len() == 1 && res == top)
        || (res % top == 0 && preorder(res / top, &values[1..]))
        || (res > top && preorder(res - top, &values[1..]))
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
