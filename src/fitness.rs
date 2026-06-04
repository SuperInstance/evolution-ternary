//! Fitness functions and multi-environment support.

/// A named environment with its own fitness weights.
#[derive(Clone, Debug)]
pub struct Environment {
    pub name: String,
    /// Per-position weights. Length should match strategy vector length.
    /// For each position, weights[0] = value of 0, weights[1] = value of 1, weights[2] = value of 2.
    pub weights: Vec<[f64; 3]>,
}

impl Environment {
    /// Create a simple environment where each position values 2 > 1 > 0.
    pub fn simple(len: usize) -> Self {
        Self {
            name: "simple".into(),
            weights: vec![[0.0, 1.0, 2.0]; len],
        }
    }

    /// An environment that rewards the "diagonal" pattern: position i prefers value i%3.
    pub fn diagonal(len: usize) -> Self {
        let weights: Vec<[f64; 3]> = (0..len)
            .map(|i| {
                let mut w = [0.0, 0.0, 0.0];
                w[i % 3] = 2.0;
                // Give small fitness to others so no zero fitness
                w[(i + 1) % 3] = 0.5;
                w[(i + 2) % 3] = 0.5;
                w
            })
            .collect();
        Self { name: "diagonal".into(), weights }
    }

    /// All zeros preferred.
    pub fn zeros(len: usize) -> Self {
        Self {
            name: "zeros".into(),
            weights: vec![[2.0, 0.5, 0.5]; len],
        }
    }

    /// Random environment with given seed.
    pub fn random_seeded(len: usize, seed: u64) -> Self {
        use crate::population::SimpleRng;
        let mut rng = SimpleRng::from_seed(seed);
        let weights: Vec<[f64; 3]> = (0..len)
            .map(|_| {
                let mut w = [0.0; 3];
                for v in &mut w {
                    *v = rng.next_f64();
                }
                w
            })
            .collect();
        Self { name: format!("random-{}", seed), weights }
    }

    /// Evaluate the fitness of a strategy in this environment.
    pub fn evaluate(&self, strategy: &[u8]) -> f64 {
        assert_eq!(strategy.len(), self.weights.len(), "Strategy length must match environment");
        strategy.iter().zip(self.weights.iter())
            .map(|(&v, w)| w[v as usize])
            .sum()
    }
}

/// A fitness function that may combine multiple environments.
#[derive(Clone, Debug)]
pub struct FitnessFunction {
    pub environments: Vec<Environment>,
    /// How to combine multi-environment fitness: "sum", "max", or "avg".
    pub combine: String,
}

impl FitnessFunction {
    /// Single-environment fitness function.
    pub fn single(env: Environment) -> Self {
        Self { environments: vec![env], combine: "sum".into() }
    }

    /// Multi-environment with sum combination.
    pub fn multi_sum(envs: Vec<Environment>) -> Self {
        Self { environments: envs, combine: "sum".into() }
    }

    /// Multi-environment with max combination.
    pub fn multi_max(envs: Vec<Environment>) -> Self {
        Self { environments: envs, combine: "max".into() }
    }

    /// Evaluate fitness across all environments.
    pub fn evaluate(&self, strategy: &[u8]) -> f64 {
        let scores: Vec<f64> = self.environments.iter()
            .map(|e| e.evaluate(strategy))
            .collect();
        match self.combine.as_str() {
            "sum" => scores.iter().sum(),
            "max" => scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            "avg" => scores.iter().sum::<f64>() / scores.len().max(1) as f64,
            _ => scores.iter().sum(),
        }
    }
}
