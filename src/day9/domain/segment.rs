#[derive(Clone, Debug)]
pub enum Segment {
    File {
        length: usize, id: usize
    },
    Free(usize)
}

impl Segment {
    pub fn is_free(&self) -> bool {
        match self {
            Segment::Free(_) => true,
            _ => false
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Segment::File { length: _, id: _ } => true,
            _ => false
        }
    }

    pub fn segment_checksum(&self, start_position: usize) -> usize {
        let mut sum = 0;
        match self {
            Segment::File { length, id } => {
                for i in start_position..start_position + *length {
                    sum += i * id;
                }
                sum
            },
            Segment::Free(length) => panic!("Checksum for free block!")
        }
    }
}
