use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

// 2a - b
fn reverse_midpoint(a: Point, b: Point) -> Point {
    Point {
        x: 2 * a.x - b.x,
        y: 2 * a.y - b.y,
    }
}

impl Point {
    fn bounds_check(self, x_max: i64, y_max: i64) -> Option<Self> {
        (self.x > -1 && self.x < x_max && self.y > -1 && self.y < y_max).then_some(self)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map: Vec<Vec<_>> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let y_max = map.len() as i64;
    let x_max = map[0].len() as i64;
    let mut points: HashMap<char, Vec<Point>> = HashMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == '.' || v == '#' {
                continue;
            }
            let point = Point {
                x: i as i64,
                y: j as i64,
            };
            if points.contains_key(&v) {
                points.get_mut(&v).unwrap().push(point)
            } else {
                points.insert(v, vec![point]);
            }
        }
    }

    println!("len(points) = {}", points.len());
    let mut antinotes = BTreeSet::new();
    for (_, points) in points.iter() {
        for (i, &a) in points.iter().enumerate() {
            if points.len() - 1 == i {
                break;
            }
            for &b in &points[i + 1..] {
                let mut left_on_hold = Some((a, b));
                let mut right_on_hold = Some((a, b));

                while let Some((a, b)) = left_on_hold.take() {
                    if let Some(p) = reverse_midpoint(a, b).bounds_check(x_max, y_max) {
                        left_on_hold.replace((p, a));
                        antinotes.insert(p);
                    }
                }

                while let Some((a, b)) = right_on_hold.take() {
                    if let Some(p) = reverse_midpoint(b, a).bounds_check(x_max, y_max) {
                        right_on_hold.replace((b, p));
                        antinotes.insert(p);
                    }
                }

                antinotes.insert(a);
                antinotes.insert(b);
            }
        }
    }
    // println!("{:?}", antinotes);
    println!("{}", antinotes.len());
    Ok(())
}
