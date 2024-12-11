use crate::day11::cache::{Cache, StateAfterNIterations};

pub struct Stones {
    pub stones_states: Vec<StateAfterNIterations>,
    pub cache: &'static Cache,
}

impl Stones {

    pub fn blink(&mut self, blinks_limit: usize) {
        let mut new_states = Vec::new();
        for substones_state in self.stones_states.iter() {
            if substones_state.iteration == blinks_limit {
                new_states.push(substones_state.clone());
            }

            for stone in substones_state.state.iter() {
                if *stone > self.cache.max_stone_cached() {
                    let s = StateAfterNIterations::blink_single_number(*stone);
                    new_states.push(StateAfterNIterations { state: s, iteration: substones_state.iteration + 1 });
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
    }
}
