#[derive(Debug, Clone, Copy)]
pub enum Block {
    Free(u32),
    Occupied { ident: u64, size: u32 },
}

fn pretty(bs: &[Block]) {
    for b in bs {
        match b {
            Block::Free(s) => print!("{}", ".".repeat(*s as usize)),
            Block::Occupied { ident, size } => {
                print!("{}", ident.to_string().repeat(*size as usize))
            }
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(input) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let disk = std::fs::read_to_string(input)?;
    let mut blocks = vec![];
    let diskmap: Vec<_> = disk
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let map = if diskmap.len() % 2 == 0 {
        &diskmap[..diskmap.len() - 1]
    } else {
        &diskmap
    };
    let mut last_ident = (map.len() / 2) as u64;
    println!("last index is = {last_ident}");
    for (i, &size) in map.iter().enumerate() {
        if size == 0 {
            continue;
        }
        if i % 2 == 0 {
            blocks.push(Block::Occupied {
                ident: (i / 2) as u64,
                size,
            });
            // it's a block
        } else {
            blocks.push(Block::Free(size))
        }
    }

    while last_ident != 0 {
        for i in (0..blocks.len()).rev() {
            let src = blocks[i];
            let Block::Occupied { ident, size } = src else {
                continue;
            };
            if ident > last_ident {
                continue;
            }

            for j in 0..i {
                let Block::Free(free) = blocks[j] else {
                    continue;
                };

                let Some(delta) = free.checked_sub(size) else {
                    continue;
                };

                blocks[j] = src;
                // since we go down from the largest file ID,
                // we can assure that we will never touch this
                // free space ever again.
                blocks[i] = Block::Free(size);
                blocks.insert(j + 1, Block::Free(delta));
                break;
            }
            break;
        }
        last_ident -= 1;

        // use this function to pretty print a layout
        // pretty(&blocks);
    }

    let mut i: u64 = 0;
    let mut acc = 0;
    for block in blocks {
        match block {
            Block::Free(s) => i += s as u64,
            Block::Occupied { ident, size } => {
                for _ in 0..size {
                    acc += ident * i;
                    i += 1;
                }
            }
        }
    }
    println!("{acc}");
    Ok(())
}
