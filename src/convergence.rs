//! Convergence detection — fitness plateaus and species stabilization.

use crate::species::SpeciesTracker;

/// Result of convergence detection.
#[derive(Clone, Debug)]
pub struct ConvergenceStatus {
    /// Whether the population has converged.
    pub converged: bool,
    /// Reason for convergence (if any).
    pub reason: Option<String>,
    /// How many generations fitness has been stable.
    pub plateau_generations: usize,
}

/// Detects convergence based on fitness plateau and species stability.
#[derive(Clone, Debug)]
pub struct ConvergenceDetector {
    /// How many generations of flat fitness before declaring convergence.
    pub plateau_threshold: usize,
    /// Minimum relative change in avg fitness to count as "improving".
    pub fitness_tolerance: f64,
    /// How many generations of stable species before declaring convergence.
    pub species_stability_threshold: usize,
    /// History of average fitness per generation.
    pub fitness_history: Vec<f64>,
}

impl ConvergenceDetector {
    pub fn new(plateau_threshold: usize, fitness_tolerance: f64) -> Self {
        Self {
            plateau_threshold,
            fitness_tolerance,
            species_stability_threshold: plateau_threshold,
            fitness_history: Vec::new(),
        }
    }

    /// Default detector: 15 generations, 0.001 tolerance.
    pub fn default_detector() -> Self {
        Self::new(15, 0.001)
    }

    /// Record a generation's average fitness.
    pub fn record(&mut self, avg_fitness: f64) {
        self.fitness_history.push(avg_fitness);
    }

    /// Check convergence based on fitness history and species tracker.
    pub fn check(&self, species: &SpeciesTracker) -> ConvergenceStatus {
        let mut plateau_gens = 0;
        let tol = self.fitness_tolerance;

        if self.fitness_history.len() >= 2 {
            for i in (1..self.fitness_history.len()).rev() {
                let prev = self.fitness_history[i - 1];
                let curr = self.fitness_history[i];
                if prev != 0.0 && (curr - prev).abs() / prev.abs() < tol {
                    plateau_gens += 1;
                } else if prev == 0.0 && curr.abs() < tol {
                    plateau_gens += 1;
                } else {
                    break;
                }
            }
        }

        let fitness_plateau = plateau_gens >= self.plateau_threshold;
        let species_stable = species.is_stable(self.species_stability_threshold);

        let (converged, reason) = match (fitness_plateau, species_stable) {
            (true, true) => (true, Some("Both fitness plateau and species stability".into())),
            (true, false) => (true, Some("Fitness plateau".into())),
            (false, true) => (true, Some("Species stabilization".into())),
            (false, false) => (false, None),
        };

        ConvergenceStatus {
            converged,
            reason,
            plateau_generations: plateau_gens,
        }
    }
}
