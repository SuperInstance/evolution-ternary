# evolution-ternary

Evolutionary dynamics on ternary strategy spaces — natural selection as sort, mutation as autofill, fitness landscapes as heatmaps.

## Concept

This crate models evolutionary dynamics where each agent carries a **ternary strategy vector** — a sequence of values from the alphabet {0, 1, 2}. The design maps directly onto the **SuperInstance Spreadsheet** paradigm:

| Spreadsheet Concept | Evolutionary Analog |
|---|---|
| **Sort** (descending by column) | **Natural Selection** — agents are ranked by fitness; the fittest survive |
| **Autofill** (extrapolate pattern) | **Mutation** — patterns in strategy vectors are extended or perturbed |
| **Conditional Format** (color by value) | **Fitness Heatmap** — strategies are colored by their fitness contribution |

## Features

- **`TernaryPopulation`** — N agents, each with a ternary strategy vector of configurable length
- **`FitnessFunction`** — configurable per-environment fitness, supports multi-environment combination (sum, max, avg)
- **`Selection`** — tournament, roulette wheel, and rank-based selection methods
- **`Mutation`** — point mutation (flip ternary values), crossover (single-point and uniform), and autofill mutation (pattern extrapolation)
- **`EvolutionEngine`** — runs generational loops with selection + mutation + elitism, tracks fitness over time
- **`SpeciesTracker`** — classifies agents into species (Explorer, Diplomat, Warrior, Hybrid) based on strategy composition
- **`ConvergenceDetector`** — detects fitness plateaus and species stabilization

## Quick Start

```rust
use evolution_ternary::*;

// Create a random population: 30 agents, strategy length 8
let pop = TernaryPopulation::random_seeded(30, 8, 42);

// Define a fitness function
let fitness = FitnessFunction::single(Environment::simple(8));

// Configure and run evolution
let config = EvolutionConfig {
    selection: SelectionMethod::tournament(),
    mutation: Mutation::default_rates(),
    max_generations: 100,
    elitism: 2,
    seed: 42,
};

let mut engine = EvolutionEngine::new(config, fitness);
let result = engine.run(pop);

println!("Generations: {}", result.generations_run);
println!("Best fitness: {:.3}", result.best_fitness);
println!("Best strategy: {:?}", result.best_strategy);
println!("Converged: {} ({:?})", result.converged, result.convergence_reason);
```

## Species Ecology

Agents are classified by their dominant strategy value:

- **Explorer** — dominated by 0s (cautious, risk-averse)
- **Diplomat** — dominated by 1s (moderate, balanced)
- **Warrior** — dominated by 2s (aggressive, risk-seeking)
- **Hybrid** — no dominant value (mixed strategy)

The `SpeciesTracker` records species composition each generation, enabling analysis of ecological dynamics and diversity.

## The Spreadsheet Analogy

### Sort = Natural Selection

In a spreadsheet, sorting by a column ranks rows by that value. In evolution, selection ranks agents by fitness — the fittest rise to the top and survive to the next generation.

### Autofill = Mutation

Spreadsheet autofill detects a pattern (0, 1, 2, ...) and extends it. Similarly, `autofill_mutate` extrapolates ternary patterns in strategy vectors. Point mutation is the stochastic analog — random perturbation that introduces novelty.

### Conditional Format = Fitness Heatmap

Conditional formatting colors cells by value, making patterns visible at a glance. The fitness landscape is the heatmap: each position in the strategy vector contributes a fitness value, and the overall pattern reveals peaks and valleys in the adaptive landscape.

## Properties

- **Pure Rust** — no unsafe code, no external dependencies
- **Deterministic** — seeded RNG for reproducible experiments
- **Configurable** — selection method, mutation rates, crossover strategy, elitism, convergence criteria

## License

MIT
