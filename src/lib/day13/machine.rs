use crate::day13::State;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Convergence {
    Above,
    Below,
    On,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Machine {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64),
    pub a_convergence: Convergence,
    pub b_convergence: Convergence,
}       

impl Machine {
    
    pub fn new(a: (i64, i64), b: (i64, i64), prize: (i64, i64)) -> Self {
        let price_way_ratio = prize.0 as f64 / prize.1 as f64;
        let button_a_ratio = a.0 as f64 / a.1 as f64;
        let button_b_ratio = b.0 as f64 / b.1 as f64;

        let mut a_convergence = Convergence::On;
        let mut b_convergence = Convergence::On;
        
        if button_a_ratio > price_way_ratio {
            a_convergence = Convergence::Above;
        } else if button_a_ratio < price_way_ratio {
            a_convergence = Convergence::Below
        } else {
            a_convergence = Convergence::On
        }

        if button_b_ratio > price_way_ratio {
            b_convergence = Convergence::Above;
        } else if button_b_ratio < price_way_ratio {
            b_convergence = Convergence::Below
        } else {
            b_convergence = Convergence::On
        }
        
        Machine {
            a,
            b,
            prize,
            a_convergence,
            b_convergence
        }
    }
    
    fn price_ratio(&self) -> f64 {
        self.prize.0 as f64 / self.prize.1 as f64
    }
    
    fn a_ratio(&self) -> f64 {
        self.a.0 as f64 / self.a.1 as f64
    }
    
    fn b_ratio(&self) -> f64 {
        self.b.0 as f64 / self.b.1 as f64
    }
    
    pub fn get_best_way_to_prize(&self) -> Option<State> {
        let ax = self.a.0 as f64;
        let ay = self.a.1 as f64;
        let bx = self.b.0 as f64;
        let by = self.b.1 as f64;
        let px = self.prize.0 as f64;
        let py = self.prize.1 as f64;
        
        let b = ((ay * px / ax) - py) * (1_f64 / ((ax * bx / ax) + by));
        let a = (px - bx * b) * (1_f64 / ax);
        
        
        
        let a_press = State { machine: self, a: 1, b: 0 };
        let b_press = State { machine: self, a: 0, b: 1 };
        let distance = State { machine: self, a: 0, b: 0 }.distance_to_prize();

        let a_distance = distance - a_press.distance_to_prize();
        let b_distance = distance - b_press.distance_to_prize();
        
        let better_button = if a_distance * 3.0 > b_distance {
            'a'
        } else {
            'b'
        };
        
        if (self.a_convergence == Convergence::Above && self.b_convergence == Convergence::Above) || 
            (self.a_convergence == Convergence::Below && self.b_convergence == Convergence::Below) {
            None
        } else {
            self.go_to_prize(better_button)
        }
    }
    
    fn go_to_prize(&self, better_button: char) -> Option<State> {
        let mut state = State { machine: self, a: 0, b: 0 };
        loop {
            if state.at_prize() {
                return Some(state)
            }
            if state.beyond_price() {
                return None
            }
            // println!("State {:?}, price ration {}, current ratio {}, dist: {}", state, self.price_ratio(), state.ratio(), state.distance_to_prize());
            if (self.price_ratio() == state.ratio()) {
                let distance_walked = state.distance(self.a.0, self.b.0);
                println!("On line! distance walked: {}, distance to price: {}", distance_walked, state.distance_to_prize());
                let number_of_repetitions_needed = state.distance_to_prize() / distance_walked;
                println!("Number of repetitions needed: {}", number_of_repetitions_needed);
                state.multiply(number_of_repetitions_needed as i64);
            }
            
            
            
            if state.current_convergence() == Convergence::Below {
                if self.a_convergence == Convergence::Above {
                    state.a = state.a + 1;
                } else if self.b_convergence == Convergence::Above {
                    state.b = state.b + 1;
                } 
            } else if state.current_convergence() == Convergence::Above {
                if self.a_convergence == Convergence::Below {
                    state.a = state.a + 1;
                } else if self.b_convergence == Convergence::Below {
                    state.b = state.b + 1;
                }
            } else {
                if better_button == 'a' {
                    state.a = state.a + 1;
                } else {
                    state.b = state.b + 1;
                }
            }
        }
    }
}
