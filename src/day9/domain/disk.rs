use std::mem;
use crate::domain::segment::Segment;

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
        for (idx, segment) in self.segments.iter().enumerate() {
            if segment.is_free() {
                return Some(idx);
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

    pub fn find_last_file_segment(&self) -> Option<usize> {
        for (idx, segment) in self.segments.iter().enumerate().rev() {
            if segment.is_file() {
                return Some(idx);
            }
        }
        None
    }

    // Returns removed block file id
    pub fn remove_last_file_block(&mut self) -> usize {
        let (last, last_file_segment) = self.segments.iter_mut().enumerate().rev().find(|(pos, segment)| segment.is_file()).unwrap();

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

    pub fn calculate_checksum(&self) -> usize {
        let mut checksum = 0;
        let mut position = 0;
        for segment in self.segments.iter() {
            match segment {
                Segment::File { length, id: _ } => {
                    checksum += segment.segment_checksum(position);
                    position = position + length;
                },
                Segment::Free(_) => {
                    break;
                }
            }
        }
        checksum
    }
}
