use rand::Rng;
use colored::*;

pub struct EasyEnv1 {
    n: usize,
    state: usize,
}

impl EasyEnv1 {
    pub fn new(n: usize) -> Self {
        assert!(n>0);
        Self {
            n,
            state: rand::thread_rng().gen_range(0..=(n-1)),
        }
    }
    pub fn state(&self) -> usize { self.state }
    pub fn step(&mut self, action: usize)->i32 {
        assert!(action<self.n);
        let goal = self.n - self.state - 1;
        let reward = 1-((goal as i32) - (action as i32)).abs();
        self.state = rand::thread_rng().gen_range(0..=(self.n-1));
        reward
    }
}


pub struct EasyEnv2 {
    pub n: usize,
    pub nn: usize,
    state: Vec<u8>,
    step: usize,
}

impl EasyEnv2 {
    pub fn new(n: usize) -> Self {
        assert!(n>0 && n<=100);
        let mut s  = Self {
            n,
            nn:n*n,
            state: vec![0; n*n+1],
            step: 0,
        };
        s.update_state();
        s
    }
    pub fn update_state(&mut self) {
        let (x,y) = (rand::thread_rng().gen_range(0..=(self.n-1)),rand::thread_rng().gen_range(0..=(self.n-1)));
        for i in 0..self.n {
            for j in 0..self.n {
                self.state[i*self.n + j]=((x as i32 - j as i32).abs()+(y as i32- i as i32).abs()) as u8;
            }
        }
        self.step += 1;
        self.state[self.nn] = self.step as u8;
    }

    pub fn is_over(&self) -> bool { self.step > 10 }
    pub fn state_size(&self) -> usize { self.nn + 1 }
    pub fn state(&self) -> &[u8] { self.state.as_slice() }
    pub fn reset(&mut self) {
        *self = Self::new(self.n);
    }

    pub fn step(&mut self, action: usize) -> i32 {
        assert!(action<self.nn);
        let reward = 1 - (self.state[action] as i32);
        self.update_state();
        reward
    }

    pub fn debug_print_state(&self, action: Option<usize>) {
        println!("state:");
        let y = action.map(|a| a / self.n).unwrap_or(self.n);
        let x = action.map(|a| a % self.n).unwrap_or(self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                let value: u8 = ((self.state[i*self.n+j] as f32) / (self.n as f32 * 2.0) * 255.0) as u8;
                if i==y && j==x {
                    print!("{}", "O".on_truecolor(value,value, value).truecolor(255-value, 255-value, 0));
                } else {
                    print!("{}", " ".on_truecolor(value,value, value));
                }
                
            }
            println!("");
        }
    }
}
