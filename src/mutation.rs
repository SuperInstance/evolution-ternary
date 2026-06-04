//! Mutation operators: point mutation, crossover (single-point, uniform).

use crate::population::{TernaryAgent, SimpleRng};

/// How crossover is performed.
#[derive(Clone, Debug)]
pub enum CrossoverMethod {
    /// Single-point crossover at a random cut.
    SinglePoint,
    /// Uniform crossover — each position picked randomly from either parent.
    Uniform,
}

/// Mutation configuration.
#[derive(Clone, Debug)]
pub struct Mutation {
    /// Probability of point mutation per position.
    pub point_rate: f64,
    /// Crossover method.
    pub crossover: CrossoverMethod,
    /// Crossover probability (if not crossed over, child = clone of parent).
    pub crossover_rate: f64,
}

impl Mutation {
    /// Default mutation: 1% point mutation rate, single-point crossover at 70%.
    pub fn default_rates() -> Self {
        Self {
            point_rate: 0.01,
            crossover: CrossoverMethod::SinglePoint,
            crossover_rate: 0.7,
        }
    }

    /// Apply point mutation to a single agent (modifies in place).
    /// Each position has `point_rate` chance of being randomly changed.
    pub fn point_mutate(&self, agent: &mut TernaryAgent, rng: &mut SimpleRng) {
        for v in &mut agent.strategy {
            if rng.next_f64() < self.point_rate {
                // Flip to a random different ternary value
                let current = *v;
                let new_val = (rng.next_u32() % 2) as u8;
                *v = if new_val < current { new_val } else { new_val + 1 };
                if *v > 2 { *v = 0; }
                debug_assert!(*v <= 2);
            }
        }
    }

    /// Crossover two parents to produce two children.
    pub fn crossover(&self, parent_a: &TernaryAgent, parent_b: &TernaryAgent, rng: &mut SimpleRng) -> (TernaryAgent, TernaryAgent) {
        if rng.next_f64() > self.crossover_rate {
            return (parent_a.clone(), parent_b.clone());
        }
        match &self.crossover {
            CrossoverMethod::SinglePoint => {
                let point = if parent_a.strategy.is_empty() {
                    0
                } else {
                    ((rng.next_u64() as usize) % parent_a.strategy.len()).max(1)
                };
                let mut child_a = parent_a.clone();
                let mut child_b = parent_b.clone();
                for i in point..parent_a.strategy.len() {
                    child_a.strategy[i] = parent_b.strategy[i];
                    child_b.strategy[i] = parent_a.strategy[i];
                }
                (child_a, child_b)
            }
            CrossoverMethod::Uniform => {
                let mut child_a = parent_a.clone();
                let mut child_b = parent_b.clone();
                for i in 0..parent_a.strategy.len() {
                    if rng.next_f64() < 0.5 {
                        child_a.strategy[i] = parent_b.strategy[i];
                        child_b.strategy[i] = parent_a.strategy[i];
                    }
                }
                (child_a, child_b)
            }
        }
    }

    /// Apply mutation (point mutation) to an agent.
    pub fn mutate(&self, agent: &mut TernaryAgent, rng: &mut SimpleRng) {
        self.point_mutate(agent, rng);
    }
}

/// Autofill mutation: analogous to spreadsheet autofill — extrapolate patterns
/// in the strategy vector to fill or replace positions.
pub fn autofill_mutate(agent: &mut TernaryAgent, start: usize, rng: &mut SimpleRng) {
    if agent.strategy.len() < 2 || start >= agent.strategy.len() {
        return;
    }
    // Find the nearest two preceding values to extrapolate a pattern
    let len = agent.strategy.len();
    if start >= 2 {
        let diff = (agent.strategy[start - 1] as i32 - agent.strategy[start - 2] as i32).rem_euclid(3);
        for i in start..len {
            agent.strategy[i] = (agent.strategy[i - 1] as i32 + diff) as u8 % 3;
        }
    } else {
        // Random fill if not enough context
        for i in start..len {
            agent.strategy[i] = (rng.next_u32() % 3) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autofill_constant_pattern() {
        let mut agent = TernaryAgent::new_unchecked(vec![1, 1, 1, 0, 0, 0]);
        let mut rng = SimpleRng::from_seed(99);
        autofill_mutate(&mut agent, 3, &mut rng);
        assert_eq!(agent.strategy[3], 1);
        assert_eq!(agent.strategy[4], 1);
        assert_eq!(agent.strategy[5], 1);
    }

    #[test]
    fn test_autofill_ascending_pattern() {
        let mut agent = TernaryAgent::new_unchecked(vec![0, 1, 0, 0, 0, 0]);
        let mut rng = SimpleRng::from_seed(99);
        autofill_mutate(&mut agent, 2, &mut rng);
        assert_eq!(agent.strategy[2], 2); // 0,1 -> diff=1 -> next=2
        assert_eq!(agent.strategy[3], 0); // 2+1=3%3=0
        assert_eq!(agent.strategy[4], 1);
        assert_eq!(agent.strategy[5], 2);
    }
}
