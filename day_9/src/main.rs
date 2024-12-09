use std::str::FromStr;

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
                _ => 0,
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
}
