use std::io::stdin;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

fn preorder(res: u64, values: &[u64], ops: &[Op]) -> bool {
    let Some(&top) = values.first() else {
        return false;
    };
    if values.len() == 1 && res == top {
        return true;
    }
    let mut path = false;
    if res % top == 0 {
        let mut next_ops = Vec::from(ops);
        next_ops.push(Op::Mul);
        path |= preorder(res / top, &values[1..], &next_ops);
    }
    if res > top {
        let mut next_ops = Vec::from(ops);
        next_ops.push(Op::Add);
        path |= preorder(res - top, &values[1..], &next_ops);
    }
    path
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
        if !preorder(r, &values, &[]) {
            continue;
        };
        sum += r;
    }
    println!("calibrated to {sum}");
    Ok(())
}
