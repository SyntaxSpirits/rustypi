use rand::Rng;

/// A Monte Carlo simulation library for estimating the value of Pi.
/// This is `rustypi`, a library that utilizes Monte Carlo methods to estimate numerical values.
pub struct RustyPi {
    samples: usize,
}

impl RustyPi {
    /// Constructs a new `RustyPi`.
    /// 
    /// # Arguments
    ///
    /// * `samples` - The number of random points to generate for the simulation.
    pub fn new(samples: usize) -> Self {
        Self { samples }
    }

    /// Estimates the value of Pi using a Monte Carlo method.
    pub fn estimate_pi(&self) -> f64 {
        let mut inside_circle = 0;
        let mut rng = rand::thread_rng();

        for _ in 0..self.samples {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();
            if x * x + y * y <= 1.0 {
                inside_circle += 1;
            }
        }

        4.0 * (inside_circle as f64) / (self.samples as f64)
    }
}
