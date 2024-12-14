struct ConsistentStep {
    a: u128,
    b: u128
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct State<'a> {
    pub machine: &'a Machine,
    pub a: u128,
    pub b: u128,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Machine {
    pub a: (u128, u128),
    pub b: (u128, u128),
    pub prize: (u128, u128),
}

impl Machine {
    
    pub fn new(a: (u128, u128), b: (u128, u128), prize: (u128, u128)) -> Self {
        Machine {
            a,
            b,
            prize
        }
    }
    
    fn check_validity(&self, a: u128, b: u128) -> bool {
        let x = a * self.a.0 + b * self.b.0;
        let y = a * self.a.1 + b * self.b.1;
        
        if x == self.prize.0 && y == self.prize.1 {
            true
        } else {
            false
        }
    }
    
    fn consistent_step_for_x(&self) -> ConsistentStep {
        let lcm = num::integer::lcm(self.a.0, self.b.0);
        let a = lcm / self.a.0;
        let b = lcm / self.b.0;
        
        ConsistentStep { a, b }
    }
    
    pub fn get_best_way_to_prize(&self) -> Option<State> {
        let ax = self.a.0;
        let bx = self.b.0;
        let px = self.prize.0;
        
        let max_a_for_x = if (px / ax) * ax == px {
            px / ax
        } else {
            (px / ax) + 1
        };
        
        let consistent_step_for_x = self.consistent_step_for_x();
        
        //find valid for x
        let mut a = max_a_for_x;
        let mut b = 0;
        let mut count = 0;
        loop {
            count = count + 1;
            if count > 10000 {
                break;
            }
            let distance = a * ax + b * bx;
            if distance == px {
                break;
            }
            if distance > px {
                if a == 0 {
                    return None
                }
                a = a - 1;
                
                let missing_distance = px - (a * ax);
                b = missing_distance / bx;
            } else {
                b = b + 1;
            }
        }
        
        if self.check_validity(a, b) {
            Some(State { machine: self, a, b })
        } else {
            let current_y = a * self.a.1 + b * self.b.1;
            let y_in_consistent_step = -(consistent_step_for_x.a as i128) * self.a.1 as i128 + consistent_step_for_x.b as i128 * self.b.1 as i128;
            
            // How many consistent steps we need to converge?
            let y_diff = self.prize.1 as i128 - current_y as i128;
            
            let steps_needed = y_diff / y_in_consistent_step;
            if steps_needed < 0 {
                return None
            }
            
            if consistent_step_for_x.a * steps_needed as u128 > a {
                return None
            }
            
            a = a - consistent_step_for_x.a * steps_needed as u128;
            b = b + consistent_step_for_x.b * steps_needed as u128;
            
            if self.check_validity(a, b) {
                Some(State { machine: self, a, b })
            } else {
                None
            }
        }
    }
}
