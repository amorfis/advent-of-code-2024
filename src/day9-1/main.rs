use std::{io, mem};
use std::fs::File;
use std::io::Read;
use day9::domain::{Disk, Segment};

fn main() -> io::Result<()> {
    let mut file = File::open("input/day9/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut current_id = 0_usize;
    let mut segments = Vec::new();

    for (idx, c) in contents.chars().enumerate() {
        if c == '\n' {
            continue;
        }
        let len = c.to_digit(10).unwrap();
        let block = if idx % 2 == 0 {
            // It's a file
            if len == 0 {
                panic!("Zero length file!")
            }
            let block = Segment::File { length: len as usize, id: current_id };
            current_id += 1;
            block
        } else {
            Segment::Free(len as usize)
        };

        segments.push(block);
    }

    let mut disk = Disk { segments: segments };

    disk.compact();

    print_blocks(&disk.segments);

    let mut it = 0;

    loop {
        if it % 10000 == 0 {
            println!("Iteration: {}", it);
            print_blocks(&disk.segments);
        }
        let last_file_segment = disk.find_last_file_segment().unwrap();
        let first_free_segment = disk.find_first_free_segment().unwrap();
        if first_free_segment < last_file_segment {
            let removed_block_id = disk.remove_last_file_block();
            disk.insert_file_block(removed_block_id, first_free_segment);
        } else {
            break;
        }
        it += 1;
    }

    print_blocks(&disk.segments);

    let checksum = disk.calculate_checksum();
    println!("Checksum: {}", checksum);

    Ok(())
}

fn print_blocks(blocks: &Vec<Segment>) {
    for block in blocks {
        match block {
            Segment::File { length, id } => {
                let str = id.to_string().repeat(*length);
                print!("{}", str);
            },
            Segment::Free(length) => {
                let str = ".".repeat(*length);
                print!("{}", str);
            }
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use claims::assert_some_eq;
    use day9::domain::{Disk, Segment};

    #[test]
    fn test() {
        let mut disk = Disk{ segments: Vec::new() };
        disk.segments.push(Segment::File { length: 1, id: 0 });
        disk.segments.push(Segment::Free(2));
        disk.segments.push(Segment::File { length: 3, id: 1 });
        disk.segments.push(Segment::File { length: 2, id: 2 });
        disk.segments.push(Segment::Free(3));

        assert_some_eq!(disk.find_first_free_segment(), 1);
        assert_some_eq!(disk.find_last_file_segment(), 3);
    }
}
