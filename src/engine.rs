//! Evolution engine — orchestrates selection, mutation, and generational loops.

use crate::population::{TernaryPopulation, SimpleRng};
use crate::fitness::FitnessFunction;
use crate::selection::{SelectionMethod, Selection};
use crate::mutation::Mutation;
use crate::species::SpeciesTracker;
use crate::convergence::ConvergenceDetector;

/// Configuration for the evolution engine.
#[derive(Clone, Debug)]
pub struct EvolutionConfig {
    pub selection: SelectionMethod,
    pub mutation: Mutation,
    pub max_generations: usize,
    pub elitism: usize, // Number of top agents carried over unchanged
    pub seed: u64,
}

impl EvolutionConfig {
    pub fn default_config() -> Self {
        Self {
            selection: SelectionMethod::tournament(),
            mutation: Mutation::default_rates(),
            max_generations: 100,
            elitism: 2,
            seed: 42,
        }
    }
}

/// Summary of a completed evolution run.
#[derive(Clone, Debug)]
pub struct EvolutionResult {
    pub generations_run: usize,
    pub best_fitness: f64,
    pub best_strategy: Vec<u8>,
    pub fitness_history: Vec<f64>,
    pub converged: bool,
    pub convergence_reason: Option<String>,
    pub final_species: (usize, usize, usize, usize),
}

/// The main evolution engine.
pub struct EvolutionEngine {
    pub config: EvolutionConfig,
    pub fitness_fn: FitnessFunction,
    pub species_tracker: SpeciesTracker,
    pub convergence_detector: ConvergenceDetector,
}

impl EvolutionEngine {
    pub fn new(config: EvolutionConfig, fitness_fn: FitnessFunction) -> Self {
        Self {
            config,
            fitness_fn,
            species_tracker: SpeciesTracker::new(),
            convergence_detector: ConvergenceDetector::default_detector(),
        }
    }

    /// Evaluate all agents in the population.
    fn evaluate(&self, pop: &mut TernaryPopulation) {
        for agent in &mut pop.agents {
            agent.fitness = self.fitness_fn.evaluate(&agent.strategy);
        }
    }

    /// Run evolution on the given initial population.
    pub fn run(&mut self, initial_pop: TernaryPopulation) -> EvolutionResult {
        let mut rng = SimpleRng::from_seed(self.config.seed);
        let mut pop = initial_pop;
        let pop_size = pop.size();

        // Evaluate initial population
        self.evaluate(&mut pop);
        pop.sort_by_fitness();

        let mut fitness_history = Vec::new();

        for gen in 0..self.config.max_generations {
            let avg = pop.avg_fitness();
            fitness_history.push(avg);
            self.convergence_detector.record(avg);
            self.species_tracker.record(&pop.agents);

            // Check convergence
            let status = self.convergence_detector.check(&self.species_tracker);
            if status.converged && gen > 5 {
                let best = pop.best().clone();
                let final_species = *self.species_tracker.history.last().unwrap();
                return EvolutionResult {
                    generations_run: gen + 1,
                    best_fitness: best.fitness,
                    best_strategy: best.strategy,
                    fitness_history,
                    converged: true,
                    convergence_reason: status.reason,
                    final_species,
                };
            }

            // Selection
            let parents = self.config.selection.select(&pop, pop_size, &mut rng);

            // Create next generation with elitism
            let mut next_agents: Vec<_> = pop.agents[..self.config.elitism.min(pop_size)].to_vec();

            // Crossover and mutation
            let mut i = 0;
            while next_agents.len() < pop_size {
                let p1 = &parents[i % parents.len()];
                let p2 = &parents[(i + 1) % parents.len()];
                let (mut c1, mut c2) = self.config.mutation.crossover(p1, p2, &mut rng);
                self.config.mutation.mutate(&mut c1, &mut rng);
                self.config.mutation.mutate(&mut c2, &mut rng);
                next_agents.push(c1);
                if next_agents.len() < pop_size {
                    next_agents.push(c2);
                }
                i += 2;
            }

            // Replace population
            pop.agents = next_agents;
            self.evaluate(&mut pop);
            pop.sort_by_fitness();
        }

        // Record final generation
        fitness_history.push(pop.avg_fitness());
        self.convergence_detector.record(pop.avg_fitness());
        self.species_tracker.record(&pop.agents);

        let best = pop.best().clone();
        let final_species = *self.species_tracker.history.last().unwrap();
        let status = self.convergence_detector.check(&self.species_tracker);

        EvolutionResult {
            generations_run: self.config.max_generations,
            best_fitness: best.fitness,
            best_strategy: best.strategy,
            fitness_history,
            converged: status.converged,
            convergence_reason: status.reason,
            final_species,
        }
    }
}
