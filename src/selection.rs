//! Selection methods: tournament, roulette wheel, rank-based.

use crate::population::{TernaryAgent, TernaryPopulation, SimpleRng};

/// Selection method to use.
#[derive(Clone, Debug)]
pub enum SelectionMethod {
    /// Tournament selection with given tournament size.
    Tournament { size: usize },
    /// Roulette wheel (fitness-proportionate) selection.
    RouletteWheel,
    /// Rank-based selection: linear ranking with selective pressure.
    RankBased { pressure: f64 },
}

impl SelectionMethod {
    /// Default tournament selection (size 3).
    pub fn tournament() -> Self {
        SelectionMethod::Tournament { size: 3 }
    }

    /// Default rank-based (pressure 1.5).
    pub fn rank_based() -> Self {
        SelectionMethod::RankBased { pressure: 1.5 }
    }
}

/// Selection trait — picks parents from a population.
pub trait Selection {
    /// Select `n` parents from the population.
    fn select(&self, pop: &TernaryPopulation, n: usize, rng: &mut SimpleRng) -> Vec<TernaryAgent>;
}

impl Selection for SelectionMethod {
    fn select(&self, pop: &TernaryPopulation, n: usize, rng: &mut SimpleRng) -> Vec<TernaryAgent> {
        match self {
            SelectionMethod::Tournament { size } => tournament_select(pop, n, *size, rng),
            SelectionMethod::RouletteWheel => roulette_select(pop, n, rng),
            SelectionMethod::RankBased { pressure } => rank_select(pop, n, *pressure, rng),
        }
    }
}

fn tournament_select(pop: &TernaryPopulation, n: usize, k: usize, rng: &mut SimpleRng) -> Vec<TernaryAgent> {
    let pop_size = pop.size();
    let k = k.max(1).min(pop_size);
    (0..n)
        .map(|_| {
            let contenders: Vec<usize> = (0..k)
                .map(|_| (rng.next_u64() as usize) % pop_size)
                .collect();
            let best_idx = *contenders.iter().max_by(|&&i, &&j| {
                pop.agents[i].fitness.partial_cmp(&pop.agents[j].fitness).unwrap_or(std::cmp::Ordering::Equal)
            }).unwrap();
            pop.agents[best_idx].clone()
        })
        .collect()
}

fn roulette_select(pop: &TernaryPopulation, n: usize, rng: &mut SimpleRng) -> Vec<TernaryAgent> {
    let min_fit = pop.agents.iter().map(|a| a.fitness).fold(f64::INFINITY, f64::min);
    // Shift to make all fitnesses positive
    let shift = if min_fit < 0.0 { -min_fit + 1.0 } else { 0.0 };
    let shifted: Vec<f64> = pop.agents.iter().map(|a| a.fitness + shift).collect();
    let total: f64 = shifted.iter().sum();
    
    if total == 0.0 {
        // Uniform random if all fitness is zero
        return (0..n)
            .map(|_| pop.agents[(rng.next_u64() as usize) % pop.size()].clone())
            .collect();
    }

    (0..n)
        .map(|_| {
            let mut r = rng.next_f64() * total;
            for (i, &s) in shifted.iter().enumerate() {
                r -= s;
                if r <= 0.0 {
                    return pop.agents[i].clone();
                }
            }
            pop.agents.last().unwrap().clone()
        })
        .collect()
}

fn rank_select(pop: &TernaryPopulation, n: usize, pressure: f64, rng: &mut SimpleRng) -> Vec<TernaryAgent> {
    // Sort by fitness descending, assign rank-based probabilities
    let mut indexed: Vec<(usize, f64)> = pop.agents.iter().enumerate()
        .map(|(i, a)| (i, a.fitness))
        .collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    let pop_size = pop.size() as f64;
    // Linear ranking: p_i = (2 - s + 2*(s-1)*(rank_i)/(N-1)) / N
    let s = pressure.clamp(1.0, 2.0);
    let probs: Vec<f64> = (0..indexed.len())
        .map(|rank| {
            let rank = rank as f64;
            (2.0 - s + 2.0 * (s - 1.0) * (indexed.len() as f64 - 1.0 - rank) / (indexed.len() as f64 - 1.0).max(1.0)) / pop_size
        })
        .collect();
    
    let total: f64 = probs.iter().sum();
    (0..n)
        .map(|_| {
            let mut r = rng.next_f64() * total;
            for (i, &p) in probs.iter().enumerate() {
                r -= p;
                if r <= 0.0 {
                    return pop.agents[indexed[i].0].clone();
                }
            }
            pop.agents[indexed.last().unwrap().0].clone()
        })
        .collect()
}
