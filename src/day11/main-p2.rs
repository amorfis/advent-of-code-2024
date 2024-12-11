use std::io;
use std::fs::File;
use std::io::Read;
use lib::day11::cache::{Cache, Stones, StateAfterNIterations, ModifiableCache};

fn main() -> io::Result<()> {
    let mut file = File::open("input/day11/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut init_stones: Vec<usize> = Vec::new();

    contents.trim().split(char::is_whitespace).for_each(|s| {
        let rr = s.parse();
        let number = rr.unwrap();
        init_stones.push(number);
    });


    let mut mcache = ModifiableCache::new();
    for n in 0..10 {
        fill_in_cache(&mut mcache, n);
    }

    let mmm: &'static ModifiableCache = Box::leak(Box::new(mcache));
    let cache: &'static Cache = Box::leak(Box::new(Cache::new(&mmm)));

    println!("Cache built");

    let init_state = StateAfterNIterations { state: init_stones, iteration: 0 };
    let mut stones = Stones { stones_states: vec![init_state], cache: &cache };

    stones.blink(10);
    // for i in 0..10 {
    //     println!("Blink {}, {} stones", i, stones.stones.iter().count());
    //     stones.blink();
    //     println!("Stones: {:?}", stones.stones);
    // }
    //
    // println!("Stones count: {:?}", stones.stones.iter().count());

    Ok(())
}

fn fill_in_cache(cache: &mut ModifiableCache, number: usize) {
    let mut state = StateAfterNIterations { state: vec![number], iteration: 0 };
    for _ in 0..10 {
        state.blink();
        cache.put_state_inside(number, state.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn test() {
        let mut linked_list = LinkedList::new();
        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);
        linked_list.push_back(4);

        let mut right_part = linked_list.split_off(2);
        *right_part.front_mut().unwrap() = 5;

        linked_list.append(&mut right_part);

        assert_eq!(linked_list.iter().collect::<Vec<&i32>>(), vec![&1, &2, &5, &4]);
    }
}
