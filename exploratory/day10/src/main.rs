use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Map {
    inner: Vec<Vec<u32>>,
}

impl Map {
    // a btreeset to only account for unqiue peaks
    fn lookaround(&self, src: Point, seen: &mut BTreeSet<Point>, unique: bool) -> u64 {
        let mut paths = 0;
        let last = self.inner[src.y][src.x];
        for (xd, yd) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let Some(x) = src.x.checked_add_signed(xd) else {
                continue;
            };
            let Some(y) = src.y.checked_add_signed(yd) else {
                continue;
            };

            let Some(&next) = self.inner.get(y).and_then(|line| line.get(x)) else {
                continue;
            };

            if next == last + 1 {
                if next == 9 {
                    if unique {
                        if !seen.contains(&Point { x, y }) {
                            paths += 1;
                            seen.insert(Point { x, y });
                        }
                    } else {
                        paths += 1;
                    }
                } else {
                    paths += self.lookaround(Point { x, y }, seen, unique);
                }
            }
        }
        paths
    }

    // fn print(&self) {
    //     for row in &self.inner {
    //         println!("{row:?}");
    //     }
    // }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(input) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let input = std::fs::read_to_string(input)?;
    let mut zeros = vec![];
    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    let c = c.to_digit(10).unwrap();
                    if c == 0 {
                        zeros.push(Point { x: j, y: i });
                    }
                    c
                })
                .collect()
        })
        .collect();
    let map = Map { inner: map };
    // map.print();
    // println!("zeros are at {zeros:?}");
    let paths: u64 = zeros
        .iter()
        .map(|&zero| map.lookaround(zero, &mut BTreeSet::new(), true))
        .sum();
    let unique_paths: u64 = zeros
        .iter()
        .map(|&zero| map.lookaround(zero, &mut BTreeSet::new(), false))
        .sum();
    // let looka = map.lookaround(zeros.first().cloned().unwrap(), &mut BTreeSet::new());
    println!("paths = {paths}");
    println!("unique paths = {unique_paths}");
    Ok(())
}
