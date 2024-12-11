#[derive(Debug, Clone, Copy)]
pub struct Occupied {
    ident: u64,
    size: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(input) = std::env::args().nth(1) else {
        eprintln!("please specify an input filename");
        return Ok(());
    };
    let disk = std::fs::read_to_string(input)?;
    let mut disk_literals = vec![];
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
    let mut last_ident = map.len() / 2;
    println!("last index is = {last_ident}");
    let mut last_block = Occupied {
        ident: last_ident as u64,
        size: map[2 * last_ident],
    };
    for (i, &size) in map.iter().enumerate() {
        if i % 2 == 0 {
            disk_literals.push(Occupied {
                ident: (i / 2) as u64,
                size,
            });
            // it's a block
        } else {
            let mut size = size;
            while size > 0 {
                if last_block.size == 0 {
                    continue;
                }

                // is the last block larger than the free space?
                if last_block.size > size {
                    last_block.size -= size;
                    disk_literals.push(Occupied {
                        ident: last_ident as u64,
                        size,
                    });
                    break;
                }

                // dump the whole last block and reassign
                // it to the next last block
                size -= last_block.size;
                disk_literals.push(last_block);
                last_ident -= 1;
                last_block = Occupied {
                    ident: last_ident as u64,
                    size: map[2 * last_ident],
                };
                if i >= 2 * last_ident - 1 {
                    break;
                }
            }
            // it's a free space
        }
        if i >= 2 * last_ident - 1 {
            if last_block.size > 0 {
                disk_literals.push(last_block);
            }
            break;
        }
    }

    // println!("{:#?}", disk_literals);
    let mut i: u64 = 0;
    let mut acc = 0;
    for Occupied { ident, size } in disk_literals {
        for _ in 0..size {
            acc += ident * i;
            i += 1;
        }
    }
    println!("{acc}");
    Ok(())
}
