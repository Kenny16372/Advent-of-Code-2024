use std::{iter::once, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(usize),
    File(usize, usize),
}

#[derive(Debug)]
struct DiskMap {
    blocks: Vec<Block>,
}

#[derive(Debug)]

struct ParseDiskMapError;

impl FromStr for DiskMap {
    type Err = ParseDiskMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = s
            .char_indices()
            .map(|(i, c)| {
                let id = i / 2;
                if let Some(size) = c.to_digit(10) {
                    if i % 2 == 0 {
                        Ok(Block::File(id, size as usize))
                    } else {
                        Ok(Block::Free(size as usize))
                    }
                } else {
                    Err(ParseDiskMapError)
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { blocks })
    }
}

impl DiskMap {
    fn compacted(&self) -> Self {
        let mut blocks = self.blocks.clone();
        let (mut idx_start, mut idx_end) = (0, blocks.len() - 1);

        while idx_start < idx_end {
            match (blocks[idx_start], blocks[idx_end]) {
                (Block::File(_, _), _) => idx_start += 1,
                (Block::Free(0), _) => idx_start += 1,
                (_, Block::Free(_)) => idx_end -= 1,
                (_, Block::File(_, 0)) => idx_end -= 1,
                (Block::Free(free), Block::File(id, size)) if size <= free => {
                    blocks[idx_start] = Block::File(id, size);
                    blocks.insert(idx_start + 1, Block::Free(free - size));
                    idx_end += 1;
                    blocks[idx_end] = Block::Free(0);
                }
                (Block::Free(free), Block::File(id, size)) if size > free => {
                    blocks[idx_start] = Block::File(id, free);
                    blocks[idx_end] = Block::File(id, size - free);
                }
                _ => unreachable!(),
            }
        }

        Self { blocks }
    }
    fn compacted_defragmented(&self) -> Self {
        let mut blocks = self.blocks.clone();

        for i in (0..blocks.len()).rev() {
            match blocks[i] {
                Block::File(_, _) => {
                    DiskMap::defragment_block(&mut blocks, i);
                    DiskMap::defragment_free_blocks(&mut blocks);
                }
                _ => continue,
            }
        }

        Self { blocks }
    }

    fn defragment_block(blocks: &mut Vec<Block>, idx_block: usize) {
        let idx = blocks.iter().position(|b| match (b, blocks[idx_block]) {
            (&Block::Free(free), Block::File(_, size)) if free >= size => true,
            _ => false,
        });
        if let Some(idx) = idx {
            if idx >= idx_block {
                return;
            }
            let free = if let Block::Free(free) = blocks[idx] {
                free
            } else {
                unreachable!()
            };
            let size = if let Block::File(_, size) = blocks[idx_block] {
                size
            } else {
                unreachable!()
            };
            blocks[idx] = blocks[idx_block];
            blocks[idx_block] = Block::Free(size);
            blocks.insert(idx + 1, Block::Free(free - size));
        }
    }

    fn defragment_free_blocks(blocks: &mut Vec<Block>) {
        let mut i = 1;
        while i < blocks.len() {
            if let (Block::Free(free1), Block::Free(free2)) = (blocks[i - 1], blocks[i]) {
                blocks.splice(i - 1..=i, once(Block::Free(free1 + free2)));
            }
            i += 1;
        }
    }

    fn checksum(&self) -> usize {
        let mut idx = 0;
        self.blocks
            .iter()
            .map(|block| match block {
                &Block::File(id, size) => {
                    let result = id * (idx..idx + size).sum::<usize>();
                    idx += size;
                    result
                }
                &Block::Free(free) => {
                    idx += free;
                    0
                }
            })
            .sum()
    }
}

fn main() {
    let contents = std::fs::read_to_string("data/input.txt").expect("Failed to read the input");

    let disk_map: DiskMap = contents
        .trim()
        .parse()
        .expect("Should be able to parse disk map");
    // println!("Disk map: {:?}", disk_map);
    let disk_map_compacted = disk_map.compacted();
    // println!("Compacted disk map: {:?}", disk_map_compacted);
    let checksum = disk_map_compacted.checksum();
    println!("Checksum: {}", checksum);
    let disk_map_compacted_defragmented = disk_map.compacted_defragmented();
    // println!(
    //     "Compacted disk map (defragmented): {:?}",
    //     disk_map_compacted_defragmented
    // );
    let checksum_defragmented = disk_map_compacted_defragmented.checksum();
    println!("Checksum defragmented: {}", checksum_defragmented);
}
