//! # evolution-ternary
//!
//! Evolutionary dynamics on ternary strategy spaces.
//!
//! Each agent carries a strategy vector over the alphabet {0, 1, 2}. Natural
//! selection is sort-based fitness ranking, mutation is analogous to spreadsheet
//! autofill, and the fitness landscape is visualised through conditional-format
//! style heatmaps.

pub mod population;
pub mod fitness;
pub mod selection;
pub mod mutation;
pub mod engine;
pub mod species;
pub mod convergence;

pub use population::{TernaryAgent, TernaryPopulation};
pub use fitness::{FitnessFunction, Environment};
pub use selection::{SelectionMethod, Selection};
pub use population::SimpleRng;
pub use mutation::{Mutation, CrossoverMethod, autofill_mutate};
pub use engine::{EvolutionEngine, EvolutionConfig, EvolutionResult};
pub use species::{Species, SpeciesTracker};
pub use convergence::ConvergenceDetector;
