use std::io::stdin;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
    Pip,
}

fn preorder(res: u64, values: &[u64], ops: &[Op]) -> Option<Vec<Op>> {
    let &top = values.first()?;
    if values.len() == 1 && res == top {
        return Some(Vec::from(ops));
    }
    let mut path = None;
    if res % top == 0 {
        let mut next_ops = Vec::from(ops);
        next_ops.push(Op::Mul);
        path = path.or(preorder(res / top, &values[1..], &next_ops));
    }

    if res > top {
        let mut next_ops = Vec::from(ops);
        next_ops.push(Op::Add);
        path = path.or(preorder(res - top, &values[1..], &next_ops));
    }

    // log requires operand to be nonzero
    if top != 0 && res > top {
        let power_of_10 = 10u64.pow(top.ilog10() + 1);
        let delta = res - top;
        if delta % power_of_10 == 0 {
            let mut next_ops = Vec::from(ops);
            next_ops.push(Op::Pip);
            path = path.or(preorder(delta / power_of_10, &values[1..], &next_ops));
        }
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
        let mut values = vals
            .trim()
            .split(' ')
            .map(|v| v.trim().parse())
            .rev()
            .collect::<Result<Vec<u64>, _>>()?;
        let Some(ops) = preorder(r, &values, &[]) else {
            continue;
        };
        if ops.is_empty() {
            continue;
        }

        // because there are parentheses from the inside out
        // run with the examples to undestand better
        values.reverse();
        let braces = "(".repeat(values.len() - 2);
        print!("{r} = {braces}");
        for (i, op) in ops.into_iter().enumerate() {
            let endbrace = if i != 0 { ")" } else { "" };
            match op {
                Op::Add => print!("{}{endbrace} + ", values[i]),
                Op::Mul => print!("{}{endbrace} * ", values[i]),
                Op::Pip => print!("{}{endbrace} || ", values[i]),
            }
        }
        println!("{}", values.last().unwrap());
        // you can remove everything between these comment blocks

        sum += r;
    }
    println!("calibrated to {sum}");
    Ok(())
}
