//! Species classification and ecology tracking.

use crate::population::TernaryAgent;

/// A species classification based on strategy composition.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Species {
    /// Dominated by 0s — cautious, risk-averse.
    Explorer,
    /// Dominated by 1s — moderate, balanced.
    Diplomat,
    /// Dominated by 2s — aggressive, risk-seeking.
    Warrior,
    /// No dominant value — mixed strategy.
    Hybrid,
}

impl Species {
    /// Classify an agent into a species based on its strategy composition.
    pub fn classify(agent: &TernaryAgent) -> Self {
        if agent.strategy.is_empty() {
            return Species::Hybrid;
        }
        let counts = count_ternary(&agent.strategy);
        let total = agent.strategy.len() as f64;
        let threshold = 0.5;

        if counts[0] as f64 / total > threshold {
            Species::Explorer
        } else if counts[1] as f64 / total > threshold {
            Species::Diplomat
        } else if counts[2] as f64 / total > threshold {
            Species::Warrior
        } else {
            Species::Hybrid
        }
    }
}

fn count_ternary(strategy: &[u8]) -> [usize; 3] {
    let mut counts = [0usize; 3];
    for &v in strategy {
        if v <= 2 {
            counts[v as usize] += 1;
        }
    }
    counts
}

/// Tracks species composition across the population over time.
#[derive(Clone, Debug)]
pub struct SpeciesTracker {
    /// History of species counts per generation: Vec<(explorer, diplomat, warrior, hybrid)>
    pub history: Vec<(usize, usize, usize, usize)>,
}

impl SpeciesTracker {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    /// Record the species composition of the current population.
    pub fn record(&mut self, agents: &[TernaryAgent]) -> (usize, usize, usize, usize) {
        let mut counts = (0, 0, 0, 0);
        for agent in agents {
            match Species::classify(agent) {
                Species::Explorer => counts.0 += 1,
                Species::Diplomat => counts.1 += 1,
                Species::Warrior => counts.2 += 1,
                Species::Hybrid => counts.3 += 1,
            }
        }
        self.history.push(counts);
        counts
    }

    /// Get the current dominant species (most members).
    pub fn dominant(&self) -> Option<Species> {
        let last = self.history.last()?;
        let max = last.0.max(last.1).max(last.2).max(last.3);
        if max == last.0 { Some(Species::Explorer) }
        else if max == last.1 { Some(Species::Diplomat) }
        else if max == last.2 { Some(Species::Warrior) }
        else { Some(Species::Hybrid) }
    }

    /// Get the diversity metric (number of species present).
    pub fn diversity(&self) -> usize {
        let last = match self.history.last() {
            Some(l) => l,
            None => return 0,
        };
        let mut count = 0;
        if last.0 > 0 { count += 1; }
        if last.1 > 0 { count += 1; }
        if last.2 > 0 { count += 1; }
        if last.3 > 0 { count += 1; }
        count
    }

    /// Number of generations tracked.
    pub fn generations(&self) -> usize {
        self.history.len()
    }

    /// Whether species composition has stabilized (no change for `n` generations).
    pub fn is_stable(&self, n: usize) -> bool {
        if self.history.len() < n + 1 {
            return false;
        }
        let recent = &self.history[self.history.len() - n..];
        let first = recent[0];
        recent.iter().all(|&r| r == first)
    }
}

impl Default for SpeciesTracker {
    fn default() -> Self {
        Self::new()
    }
}
