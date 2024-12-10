use std::mem;
use crate::day9::domain::segment::Segment;

pub struct Disk {
    pub segments: Vec<Segment>
}

impl Disk {
    pub fn compact(&mut self) {
        let uncompacted = mem::replace(&mut self.segments, Vec::new());

        self.segments = uncompacted.into_iter().filter(|b| match b {
            Segment::File { length, id: _ } => {
                if *length <= 0 {
                    panic!("Zero length file!")
                }
                true
            },
            Segment::Free(length) => *length > 0
        }).collect::<Vec<Segment>>();
    }

    pub fn find_first_free_segment(&self) -> Option<usize> {
        self.find_first_free_segment_of_size(1)
    }

    pub fn find_first_free_segment_of_size(&self, size: usize) -> Option<usize> {
        for (idx, segment) in self.segments.iter().enumerate() {
            match segment {
                Segment::Free(length) => {
                    if *length >= size {
                        return Some(idx);
                    }
                },
                _ => {}
            }
        }
        None
    }

    pub fn insert_file_block(&mut self, file_id: usize, idx: usize) {
        // Insert file block
        let segment = Segment::File { length: 1, id: file_id };
        self.segments.insert(idx, segment);

        // Chop off the free block after the inserted one
        let next_segment = self.segments.get_mut(idx + 1).unwrap();
        match next_segment {
            Segment::File { length: _, id: _ } => panic!("Next segment after inserting block is a file!"),
            Segment::Free(len) => {
                match len {
                    0 => {
                        panic!("Zero length free block!")
                    },
                    1 => {
                        self.segments.remove(idx + 1);
                    },
                    n => {
                        *n = *n - 1;
                    }
                }
            }
        };
    }

    pub fn insert_file(&mut self, file: Segment, free_segment_pos: usize) {
        let free_segment_length = self.segments.get(free_segment_pos).unwrap().free_length();
        let insert_file_length = file.file_length();

        assert!(free_segment_length >= insert_file_length);

        self.segments.insert(free_segment_pos, file);

        assert!(insert_file_length > 0);

        if insert_file_length == free_segment_length {
            self.segments.remove(free_segment_pos + 1);
        } else {
            let free_to_shrink = self.segments.get_mut(free_segment_pos + 1).unwrap();
            match free_to_shrink {
                Segment::File { .. } => panic!("Free segment to chop off, but found a file!"),
                Segment::Free(len) => {
                    *len = *len - insert_file_length;
                }
            }
        }
    }

    pub fn find_last_file_segment(&self) -> Option<usize> {
        match self.find_last_file_segment_omitting_n(0) {
            Some((idx, Segment::File {length: _, id: _})) => Some(idx),
            _ => panic!("No file segments found!")
        }
    }

    pub fn find_last_file_segment_omitting_n(&self, n: usize) -> Option<(usize, &Segment)> {
        let mut omitted = 0;
        for (idx, segment) in self.segments.iter().enumerate().rev() {
            if segment.is_file() {
                if omitted == n {
                    return Some((idx, segment));
                } else {
                    omitted += 1;
                }
            }
        }
        None
    }

    // Returns removed block file id
    pub fn remove_last_file_block(&mut self) -> usize {
        let (last, last_file_segment) = self.segments.iter_mut().enumerate().rev().find(|(_, segment)| segment.is_file()).unwrap();

        let (remove_segment, file_id) = match last_file_segment {
            Segment::File { length: len, id } => {
                match len {
                    0 => { panic!("Zero length file!") },
                    1 => {
                        (true, *id)
                    },
                    l => {
                        *l = *l - 1;
                        (false, *id)
                    }
                }
            },
            _ => panic!("Last segment is not a file!")
        };

        if remove_segment {
            self.segments.remove(last);
        }

        file_id
    }

    pub fn remove_file(&mut self, idx: usize) -> usize {
        let segment = self.segments.remove(idx);
        let (removed_length, removed_id) = match segment {
            Segment::File { length, id } => (length, id),
            _ => panic!("Not a file!")
        };

        self.segments.insert(idx, Segment::Free(removed_length));

        self.compact();

        removed_id
    }

    pub fn calculate_checksum(&self) -> usize {
        let mut checksum = 0;
        let mut position = 0;
        for segment in self.segments.iter() {
            match segment {
                Segment::File { length, id: _ } => {
                    checksum += segment.segment_checksum(position);
                    position = position + length;
                },
                Segment::Free(length) => {
                    position = position + length;
                }
            }
        }
        checksum
    }
}
