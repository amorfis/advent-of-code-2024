use crate::day13::Machine;
use crate::day13::machine::Convergence;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct State<'a> {
    pub machine: &'a Machine,
    pub a: i64,
    pub b: i64,
}

impl State<'_> {
    pub fn distance(&self, a_travel: i64, b_travel: i64) -> f64 {
        (self.a as f64 * a_travel as f64).hypot(self.b as f64 * b_travel as f64)
    }
    
    pub fn multiply(&mut self, m: i64) {
        println!("Multiplying a: {} b: {} by {}", self.a, self.b, m);
        self.a = self.a * m;
        self.b = self.b * m;
    }
    
    pub fn ratio(&self) -> f64 {
        (self.machine.a.0 as f64 * self.a as f64 + self.machine.b.0 as f64 * self.b as f64) /
            (self.machine.a.1 as f64 * self.a as f64 + self.machine.b.1 as f64 * self.b as f64)
    }
    
    pub fn distance_to_prize(&self) -> f64 {
        let x = self.a * self.machine.a.0 + self.b * self.machine.b.0;
        let y = self.a * self.machine.a.1 + self.b * self.machine.b.1;
        
        let x_to_prize = self.machine.prize.0 - x;
        let y_to_prize = self.machine.prize.1 - y;
        
        (x_to_prize as f64).hypot(y_to_prize as f64)
    }
    
    pub fn beyond_price(&self) -> bool {
        if (self.a * self.machine.a.0) + self.b * self.machine.b.0 > self.machine.prize.0 {
            return true
        }
        
        if (self.a * self.machine.a.1) + self.b * self.machine.b.1 > self.machine.prize.1 {
            return true
        }
        
        false
    }
    
    pub fn current_convergence(&self) -> Convergence {
        let x = self.a * self.machine.a.0 + self.b * self.machine.b.0;
        let y = self.a * self.machine.a.1 + self.b * self.machine.b.1;
        
        if x as f64 / y as f64 > self.machine.prize.0 as f64 / self.machine.prize.1 as f64 {
            Convergence::Above
        } else if (x as f64 / y as f64) < (self.machine.prize.0 as f64 / self.machine.prize.1 as f64) {
            Convergence::Below
        } else {
            Convergence::On
        }
    }
    
    pub fn at_prize(&self) -> bool {
        let x = self.a * self.machine.a.0 + self.b * self.machine.b.0;
        let y = self.a * self.machine.a.1 + self.b * self.machine.b.1;
        
        if x == self.machine.prize.0 && y == self.machine.prize.1 {
            return true
        }
        
        false
    }
}