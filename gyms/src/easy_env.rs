
struct EasyEnv1 {
    n: usize,
    state: usize,
}

impl EasyEnv1 {
    fn new(n: usize) -> Self {
        assert!(n>0);
        Self {
            n,
            state: rand::thread_rng().gen_range(0..=(n-1)),
        }
    }
    fn state(&self) -> usize { self.state }
    fn step(action: usize)->i32 {
        assert!(action<self.n);
        let goal = self.n - self.state - 1;
        let reward = 1-((goal as i32) - action).abs();                                                                    
        self.state = rand::thread_rng().gen_range(0..=(n-1));
        reward
    }
}


struct EasyEnv2 {
    n: usize,
    nn: usize,
    state: (usize,usize),
}

impl EasyEnv2 {
    fn new(n: usize) -> Self {
        assert!(n>0);
        Self {
            n,
            nn:n*n,
            state: (rand::thread_rng().gen_range(0..=(n-1)),rand::thread_rng().gen_range(0..=(n-1)))
        }
    }
    fn state(&self) -> (usize, usize) { self.state }
    fn step(action: usize)->i32 {
        assert!(action<self.nn);
        let action_y = action / self.n;
        let action_x = action % self.n;
        let goal = (self.n - self.state.0 - 1, self.n-self.state.1-1);
        let reward = 1-((goal.0 as i32) - action_x).abs()-((goal.1 as i32) - action_y).abs();
        self.state = (rand::thread_rng().gen_range(0..=(n-1)),rand::thread_rng().gen_range(0..=(n-1)));
        reward
    }
}
