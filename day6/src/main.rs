use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Direction {
    x: isize,
    y: isize,
}

impl Point {
    fn add(self, dir: Direction) -> Option<Point> {
        Some(Self {
            x: self.x.checked_add_signed(dir.x)?,
            y: self.y.checked_add_signed(dir.y)?,
        })
    }
}

impl Direction {
    const UP: Self = Self { x: 0, y: -1 };
    fn rotate(self) -> Self {
        Self {
            x: if self.x == 0 { -self.y } else { self.y },
            y: if self.x == 0 { -self.x } else { self.x },
        }
    }
}

pub struct Map(Vec<Vec<char>>);
impl Map {
    fn get(&self, p: Point) -> Option<char> {
        self.0.get(p.y).and_then(|line| line.get(p.x).cloned())
    }
}

fn creates_cycle(obstacle: Point, mut pos: Point, map: &Map) -> bool {
    let mut old = pos;
    let mut approaches: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let mut direction = Direction::UP;
    while let Some(c) = map.get(pos) {
        if c == '#' || pos == obstacle {
            // did I approach this obstacle from the same direction
            // previously?
            if !approaches.entry(pos).or_default().insert(direction) {
                return true;
            };
            pos = old;
            direction = direction.rotate();
        }

        let Some(p) = pos.add(direction) else {
            break;
        };
        old = pos;
        pos = p;
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let input = std::fs::read_to_string(arg)?;
    let mut pos = None;
    let map = Map(input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, v)| {
                    if v == '^' {
                        pos.replace(Point { x, y });
                    }
                    v
                })
                .collect()
        })
        .collect());

    let Some(pos) = pos else {
        eprintln!("no caret");
        return Ok(());
    };

    let mut uniqs = BTreeSet::new();

    let mut direction = Direction::UP;
    let mut transient_pos = pos;
    let mut old = transient_pos;
    while let Some(c) = map.get(transient_pos) {
        if c == '#' {
            transient_pos = old;
            direction = direction.rotate();
        } else {
            uniqs.insert(transient_pos);
        }

        let Some(p) = transient_pos.add(direction) else {
            break;
        };
        old = transient_pos;
        transient_pos = p;
    }

    println!("{}", uniqs.len());

    // we can only place an obstacle in places
    // where the guard can reach
    let obstacles = uniqs
        .into_iter()
        .filter(|&bob| creates_cycle(bob, pos, &map))
        .count();
    println!("{}", obstacles);
    Ok(())
}
