use std::io;
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

    let mut omitting_n_files = 0;

    let mut it = 0;

    loop {
        // print_blocks(&disk.segments);
        // println!("Omitting {} files", omitting_n_files);
        // println!();

        if (it % 1000) == 0 {
            println!("Iteration {}", it);
        }

        let (current_file_pos, current_file) = match disk.find_last_file_segment_omitting_n(omitting_n_files) {
            Some((pos, segment)) => (pos, segment.clone()),
            None => break
        };

        match disk.find_first_free_segment_of_size(current_file.file_length()) {
            Some(first_fitting_free_segment_pos) => {
                if first_fitting_free_segment_pos < current_file_pos {
                    disk.remove_file(current_file_pos);
                    disk.insert_file(current_file, first_fitting_free_segment_pos);
                } else {
                    // file too big, did not move
                    omitting_n_files += 1;
                }
            }
            None => {
                omitting_n_files += 1;
            },
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
