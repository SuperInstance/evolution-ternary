//! Core population types — agents with ternary strategy vectors.

use core::fmt;

/// A single value in the ternary alphabet {0, 1, 2}.
pub type TernaryValue = u8;

/// Validate that a value is in {0, 1, 2}.
fn valid_ternary(v: TernaryValue) -> bool {
    matches!(v, 0 | 1 | 2)
}

/// An agent whose strategy is a vector of ternary values.
#[derive(Clone, PartialEq)]
pub struct TernaryAgent {
    /// The strategy vector — each element is 0, 1, or 2.
    pub strategy: Vec<TernaryValue>,
    /// Cached fitness (set by the engine after evaluation).
    pub fitness: f64,
}

impl TernaryAgent {
    /// Create a new agent, validating all values are ternary.
    pub fn new(strategy: Vec<TernaryValue>) -> Option<Self> {
        if strategy.iter().all(|&v| valid_ternary(v)) {
            Some(Self { strategy, fitness: 0.0 })
        } else {
            None
        }
    }

    /// Create an agent without validation (for internal use where values are known good).
    pub fn new_unchecked(strategy: Vec<TernaryValue>) -> Self {
        Self { strategy, fitness: 0.0 }
    }

    /// Strategy length.
    pub fn len(&self) -> usize {
        self.strategy.len()
    }

    /// Whether the strategy is empty.
    pub fn is_empty(&self) -> bool {
        self.strategy.is_empty()
    }
}

impl fmt::Debug for TernaryAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Agent({:?}, fit={:.3})", self.strategy, self.fitness)
    }
}

/// A population of ternary agents.
#[derive(Clone)]
pub struct TernaryPopulation {
    pub agents: Vec<TernaryAgent>,
    /// Strategy vector length (constant across the population).
    pub strategy_len: usize,
}

impl TernaryPopulation {
    /// Create a new population from a list of agents.
    ///
    /// Returns `None` if agents have mismatched strategy lengths or the list is empty.
    pub fn new(agents: Vec<TernaryAgent>) -> Option<Self> {
        if agents.is_empty() {
            return None;
        }
        let len = agents[0].len();
        if !agents.iter().all(|a| a.len() == len) {
            return None;
        }
        Some(Self { agents, strategy_len: len })
    }

    /// Create a random population of `n` agents, each with `strategy_len` ternary values.
    pub fn random(n: usize, strategy_len: usize) -> Self {
        let mut rng = SimpleRng::from_seed(42);
        let agents = (0..n)
            .map(|_| {
                let strategy: Vec<TernaryValue> = (0..strategy_len)
                    .map(|_| (rng.next_u32() % 3) as TernaryValue)
                    .collect();
                TernaryAgent::new_unchecked(strategy)
            })
            .collect();
        Self { agents, strategy_len }
    }

    /// Create a random population with a given seed for reproducibility.
    pub fn random_seeded(n: usize, strategy_len: usize, seed: u64) -> Self {
        let mut rng = SimpleRng::from_seed(seed);
        let agents = (0..n)
            .map(|_| {
                let strategy: Vec<TernaryValue> = (0..strategy_len)
                    .map(|_| (rng.next_u32() % 3) as TernaryValue)
                    .collect();
                TernaryAgent::new_unchecked(strategy)
            })
            .collect();
        Self { agents, strategy_len }
    }

    /// Population size.
    pub fn size(&self) -> usize {
        self.agents.len()
    }

    /// Sort agents by fitness descending (natural selection = sort).
    pub fn sort_by_fitness(&mut self) {
        self.agents.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Get the best agent.
    pub fn best(&self) -> &TernaryAgent {
        self.agents.iter().max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(std::cmp::Ordering::Equal)).unwrap()
    }

    /// Get the average fitness.
    pub fn avg_fitness(&self) -> f64 {
        if self.agents.is_empty() { return 0.0; }
        self.agents.iter().map(|a| a.fitness).sum::<f64>() / self.agents.len() as f64
    }

    /// Get the worst agent.
    pub fn worst(&self) -> &TernaryAgent {
        self.agents.iter().min_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(std::cmp::Ordering::Equal)).unwrap()
    }
}

/// Minimal xorshift PRNG — no external deps needed.
#[derive(Clone)]
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn from_seed(seed: u64) -> Self {
        // Ensure non-zero state
        Self { state: if seed == 0 { 0xDEAD_BEEF_CAFE_BABE } else { seed } }
    }

    pub fn next_u32(&mut self) -> u32 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state as u32
    }

    pub fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;
        (hi << 32) | lo
    }

    /// Uniform f64 in [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64)
    }
}
