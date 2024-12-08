use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Eqn {
    slope: Slope,
    abcis: Abcissa,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Abcissa(f64);
impl Eq for Abcissa {}
impl Ord for Abcissa {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Slope(f64);
impl Eq for Slope {}
impl Ord for Slope {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

pub fn gcd(mut n: i64, mut m: i64) -> i64 {
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
    let mut eqns = BTreeSet::new();
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
    for (_, points) in points.iter() {
        for (i, a) in points.iter().enumerate() {
            if points.len() - 1 == i {
                break;
            }
            for b in &points[i + 1..] {
                let denom = b.x - a.x;
                let numer = b.y - a.y;
                let (slope, abcis) = if denom == 0 {
                    (Slope(f64::INFINITY), Abcissa(0f64))
                } else if numer == 0 {
                    (Slope(0f64), Abcissa(a.y as f64))
                } else {
                    let n = a.y * denom - numer * a.x;
                    let d = denom;

                    (
                        Slope(numer as f64 / denom as f64),
                        Abcissa(n as f64 / d as f64),
                    )
                };
                eqns.insert(Eqn { slope, abcis });
            }
        }
    }

    let mut point_collection: BTreeMap<Eqn, Vec<Point>> = BTreeMap::new();
    for eqn in eqns {
        for (_, points) in points.iter() {
            for point in points {
                if eqn.slope.0 * point.x as f64 + eqn.abcis.0 == point.y as f64 {
                    if point_collection.contains_key(&eqn) {
                        point_collection.get_mut(&eqn).unwrap().push(*point);
                    } else {
                        point_collection.insert(eqn, vec![*point]);
                    }
                }
            }
        }
    }

    println!("{:?}", point_collection);
    // println!("{:?}", eqns);
    Ok(())
}
