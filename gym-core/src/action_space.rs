/// Define common action spaces.
use rand::Rng;

pub trait ActionSpaceTrait<Dtype> {
    fn dim(&self) -> usize;
    // fn sample(&self, rng: &mut dyn rand::RngCore) -> ndarray::ArrayD<Dtype>;
    fn sample(&self, rng: &mut dyn rand::RngCore) -> ndarray::ArrayD<i32>;
}

pub type ActionSpace<Dtype> = Box<dyn ActionSpaceTrait<Dtype>>;

/// action 1 ~ action n
pub struct CategoricalActionSpace {
    pub n: usize,
}
impl CategoricalActionSpace {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
    pub fn sample(&self, rng: &mut dyn rand::RngCore) -> i32 {
        rng.gen_range(1..self.n + 1) as i32
    }
}
impl ActionSpaceTrait<i32> for CategoricalActionSpace {
    fn dim(&self) -> usize {
        0
    }
    fn sample(&self, rng: &mut dyn rand::RngCore) -> ndarray::ArrayD<i32> {
        ndarray::arr0(self.sample(rng)).into_dyn()
    }
}

pub struct MultiCategoricalActionSpace {
    pub n: Vec<usize>,
}
impl MultiCategoricalActionSpace {
    pub fn new(n: Vec<usize>) -> Self {
        Self { n }
    }
    pub fn sample(&self, rng: &mut dyn rand::RngCore) -> Vec<i32> {
        self.n
            .iter()
            .map(|n| rng.gen_range(1..n + 1) as i32)
            .collect()
    }
}

pub struct ContinuousActionSpace {
    low: f32,
    high: f32,
}
impl ContinuousActionSpace {
    pub fn new(low: f32, high: f32) -> Self {
        Self { low, high }
    }
    pub fn sample(&self, rng: &mut dyn rand::RngCore) -> f32 {
        rng.gen_range(self.low..self.high) as f32
    }
}

// fn sample<R: rand::Rng>(&self, rng: R) -> f32 {
//     rng.gen_range(self.low..self.high) as f32
// }
pub struct MultiContinuousActionSpace {
    pub low: Vec<f32>,
    pub high: Vec<f32>,
}
impl MultiContinuousActionSpace {
    pub fn new(low: Vec<f32>, high: Vec<f32>) -> Self {
        Self { low, high }
    }
    pub fn sample(&self, rng: &mut dyn rand::RngCore) -> Vec<f32> {
        self.low
            .iter()
            .zip(self.high.iter())
            .map(|(low, high)| rng.gen_range(*low..*high) as f32)
            .collect()
    }
}
