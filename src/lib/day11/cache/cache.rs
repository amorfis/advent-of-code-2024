use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter, Pointer};
use claims::assert_none;

#[derive(Debug)]
pub struct ModifiableCache {
    cache: HashMap<usize, HashMap<usize, StateAfterNIterations>>,
}

impl ModifiableCache {
    pub fn new() -> ModifiableCache {
        ModifiableCache {
            cache: HashMap::new()
        }
    }

    pub fn put_state_inside(&mut self, number: usize, state: StateAfterNIterations) {
        let states_for_number = self.cache.entry(number).or_insert(HashMap::new());

        assert_none!(states_for_number.get(&state.iteration));

        states_for_number.insert(state.iteration, state);
    }
}

pub struct Cache {
    cache: &'static ModifiableCache,
    max_stone_cached: usize,
    max_iterations_cached: usize
}

impl Cache {
    pub fn new(modifiable_cache: &'static ModifiableCache) -> Cache {
        let max_stone_number = modifiable_cache.cache.keys().max().unwrap();
        assert_eq!(modifiable_cache.cache.keys().count(), *max_stone_number + 1);

        let max_iterations = modifiable_cache.cache.values().next().unwrap().values().count();
        for (_, states) in modifiable_cache.cache.iter() {
            let highest_iteration_for_stone = states.values().map(|s| s.iteration).max().unwrap();
            assert_eq!(highest_iteration_for_stone, max_iterations);
        }

        Cache {
            cache: &modifiable_cache,
            max_stone_cached: *max_stone_number,
            max_iterations_cached: max_iterations

        }
    }

    pub fn get_state_after_n_iterations(&self, number: usize, iteration: usize) -> Option<StateAfterNIterations> {
        self.cache.cache.get(&number).and_then(|map_for_number| {
            map_for_number.get(&iteration).map(|state| state.clone())
        })
    }

    pub fn get_state_after_max_iterations(&self, number: usize) -> Option<StateAfterNIterations> {
        self.get_state_after_n_iterations(number, self.max_iterations_cached)
    }

    pub fn max_stone_cached(&self) -> usize {
        self.max_stone_cached
    }

    pub fn max_iterations_cached(&self) -> usize {
        self.max_iterations_cached
    }
}

#[derive(Debug, Clone)]
pub struct StateAfterNIterations {
    pub iteration: usize,
    pub state: Vec<usize>,
}

impl StateAfterNIterations {
    pub fn blink(&mut self) {
        // calculate splits
        let mut splits = Vec::new();
        for (idx, stone) in self.state.iter().enumerate() {
            let stone_as_string = stone.to_string();
            if stone_as_string.chars().count() % 2 == 0 {
                let (left, right) = (stone_as_string.chars().take(stone_as_string.chars().count() / 2), stone_as_string.chars().skip(stone_as_string.chars().count() / 2));
                splits.push((idx, left.collect::<String>().parse::<usize>().unwrap(), right.collect::<String>().parse::<usize>().unwrap()));
            }
        }

        let splits_indices = splits.iter().map(|(idx, _, _)| *idx).collect::<HashSet<usize>>();

        // splits calculated, but let's leave them for now. Now do other modifications
        for (idx, stone) in self.state.iter_mut().enumerate() {
            if splits_indices.contains(&idx) {
                // We'll split this stone later
                continue;
            }

            if *stone == 0 {
                *stone = 1;
            } else {
                *stone = *stone * 2024;
            }
        }

        for (split_idx, left, right) in splits.iter().rev() {
            self.state.insert(*split_idx, *left);
            self.state[split_idx + 1] = *right;
        }

        self.iteration = self.iteration + 1;
    }

    pub fn blink_single_number(number: usize) -> Vec<usize> {
        let stone_as_string = number.to_string();
        if stone_as_string.chars().count() % 2 == 0 {
            let (left, right) = (stone_as_string.chars().take(stone_as_string.chars().count() / 2), stone_as_string.chars().skip(stone_as_string.chars().count() / 2));
            vec![left.collect::<String>().parse::<usize>().unwrap(), right.collect::<String>().parse::<usize>().unwrap()]
        } else if number == 0 {
            vec![1]
        } else {
            vec![number * 2024]
        }
    }
}
