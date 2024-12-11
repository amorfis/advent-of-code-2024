use crate::day11::cache::{Cache, StateAfterNIterations};

pub struct Stones {
    pub stones_states: Vec<StateAfterNIterations>,
    pub cache: &'static Cache,
    blinks_done: usize
}

impl Stones {

    pub fn new(init_vec: Vec<StateAfterNIterations>, cache: &'static Cache) -> Stones {
        Stones {
            stones_states: init_vec,
            cache: cache,
            blinks_done: 0
        }
    }

    pub fn blink(&mut self, blinks_limit: usize) {
        let mut new_states = Vec::new();
        for substones_state in self.stones_states.iter() {
            if substones_state.iteration == blinks_limit || substones_state.iteration >= (self.blinks_done + 1) {
                new_states.push(substones_state.clone());
                continue;
            }

            for stone in substones_state.state.iter() {
                if *stone > self.cache.max_stone_cached() {
                    let mut new_state = StateAfterNIterations { state: vec![*stone], iteration: substones_state.iteration };
                    new_state.blink();
                    // let s = StateAfterNIterations::blink_single_number(*stone);
                    // new_states.push(StateAfterNIterations { state: s, iteration: substones_state.iteration + 1 });
                    new_states.push(new_state);
                } else {
                    // We have something in cache for this stone number
                    // Let's take from cache.
                    let iterations_to_blink = blinks_limit - substones_state.iteration;
                    assert!(iterations_to_blink > 0);

                    let mut state_from_cache = if iterations_to_blink > self.cache.max_iterations_cached() {
                        self.cache.get_state_after_max_iterations(*stone).unwrap().clone()
                    } else {
                        self.cache.get_state_after_n_iterations(*stone, iterations_to_blink).unwrap().clone()
                    };

                    state_from_cache.iteration = substones_state.iteration + state_from_cache.iteration;
                    new_states.push(state_from_cache);
                }
            }
        }
        self.stones_states = new_states;
        self.blinks_done = self.blinks_done + 1;
    }

    pub fn count_stones(&self) -> usize {
        let iteration = self.stones_states.iter().next().unwrap().iteration;

        let mut count = 0;
        for state in self.stones_states.iter() {
            // assert_eq!(state.iteration, iteration);
            count = count + state.state.iter().count();
        }

        count
    }
}
