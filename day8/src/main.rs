use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn bounds_check(self, x_max: i64, y_max: i64) -> Option<Self> {
        (self.x > -1 && self.x < x_max && self.y > -1 && self.y < y_max).then_some(self)
    }
    // 2a - b
    fn reverse_midpoint(self, b: Point) -> Point {
        Point {
            x: 2 * self.x - b.x,
            y: 2 * self.y - b.y,
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(input) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let map: Vec<Vec<_>> = std::fs::read_to_string(input)?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let y_max = map.len() as i64;
    let x_max = map[0].len() as i64;
    let mut points: BTreeMap<char, Vec<Point>> = BTreeMap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            if v == '.' || v == '#' {
                continue;
            }
            points.entry(v).or_default().push(Point {
                x: i as i64,
                y: j as i64,
            });
        }
    }

    println!("len(points) = {}", points.len());
    let mut antinotes = BTreeSet::new();
    for (_, points) in points.iter() {
        for (i, &a) in points[0..points.len() - 1].iter().enumerate() {
            for &b in &points[i + 1..] {
                if let Some(p) = a.reverse_midpoint(b).bounds_check(x_max, y_max) {
                    antinotes.insert(p);
                }
                if let Some(p) = b.reverse_midpoint(a).bounds_check(x_max, y_max) {
                    antinotes.insert(p);
                }
            }
        }
    }
    println!("{}", antinotes.len());
    Ok(())
}
