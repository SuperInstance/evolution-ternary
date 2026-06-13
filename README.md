# evolution-ternary

**Evolutionary dynamics on ternary strategy spaces** — agents carry strategy vectors over the alphabet {0, 1, 2}, with tournament/roulette/rank selection, single-point/uniform crossover, point mutation, fitness-proportionate evaluation across multiple environments, speciation, and convergence detection.

## Why It Matters

Genetic algorithms (GAs) are optimization methods inspired by natural selection: maintain a population of candidate solutions, evaluate fitness, select parents, recombine, mutate, and repeat. Traditional GAs use binary {0, 1} or real-valued chromosomes. This crate uses **ternary {0, 1, 2}** encoding, which offers:

1. **Expressiveness**: Three states (underweight, neutral, overweight) vs. binary's two (present/absent). This maps naturally to neural network weight quantization ({−1, 0, +1}).
2. **Smoother landscape**: Ternary provides a middle ground — mutation can "step" through neutral states rather than always flipping polarity.
3. **Alignment with ternary computing**: The output strategy vectors are directly usable as weights for ternary neural networks (BitNet b1.58 architecture).

The crate also implements **speciation** (population structuring by similarity, protecting innovative strategies from premature elimination) and **convergence detection** (identifying fitness plateaus and species stabilization).

## How It Works

### Genetic Algorithm Loop

```
Initialize population (random ternary strategies)
    │
    ▼
┌─▶ Evaluate fitness (per environment)
│       │
│   Select parents (tournament / roulette / rank)
│       │
│   Crossover (single-point / uniform)
│       │
│   Mutate (point mutation at rate μ)
│       │
│   Speciate (cluster by similarity)
│       │
│   Convergence check ──▶ stop if plateau
│       │
└───────┘ (next generation)
```

### Selection Methods

**Tournament Selection** (size k): Pick k random agents, return the best. Selection pressure increases with k.

$$P_{\text{select}}(\text{best of } k) = 1$$

Expected complexity: O(k) per selection, O(n·k) for n selections.

**Roulette Wheel**: Probability proportional to fitness. For fitness values f_i:

$$P(i) = \frac{f_i}{\sum_j f_j}$$

Requires non-negative fitness (negative values are shifted). Fails when all fitness = 0 (falls back to uniform).

**Rank-Based** (pressure s ∈ [1, 2]): Probability based on rank, not raw fitness:

$$P(\text{rank } r) = \frac{2 - s + 2(s-1)\frac{N - r}{N - 1}}{N}$$

This decouples selection pressure from fitness magnitude — more robust to scaling issues.

### Crossover Operators

**Single-Point**: Choose cut position c, swap tails. Produces two children from two parents. Preserves building blocks on each side of the cut.

**Uniform**: For each position, randomly pick from parent A or B with 50% probability. Maximum mixing — destroys building blocks but explores more.

### Point Mutation

Each position has probability μ of mutation. The new value is chosen uniformly from the other two ternary values:

$$P(v' | v) = \begin{cases} \frac{\mu}{2} & \text{if } v' \neq v \\ 1 - \mu & \text{if } v' = v \end{cases}$$

Typical mutation rate: μ = 0.01 (1%).

### Fitness Evaluation

Each environment provides a weight matrix `w[position][value]`. Fitness is the weighted sum:

$$F(\mathbf{s}) = \sum_{i=0}^{L-1} w_i[s_i]$$

Multi-environment fitness combines scores via sum, max, or average — enabling multi-objective optimization.

### Speciation

Agents are clustered by strategy similarity (number of matching positions). Within each species, evolution proceeds independently before merging — this protects novel strategies from being eliminated by dominant ones before they can be refined.

### Convergence Detection

The system signals convergence when:
- Fitness plateau: best fitness has not improved for N consecutive generations
- Species stabilization: species membership hasn't changed for N generations

### Complexity Analysis

| Operation | Time Complexity |
|-----------|----------------|
| Fitness evaluation | O(n × L) where n = pop size, L = strategy length |
| Tournament selection | O(n × k) |
| Roulette selection | O(n²) (cumulative distribution) |
| Rank selection | O(n log n) (sort) + O(n²) (selection) |
| Single-point crossover | O(L) per pair |
| Uniform crossover | O(L) per pair |
| Point mutation | O(L) per agent |
| Speciation | O(n² × L) (all pairs similarity) |

## Quick Start

```rust
use evolution_ternary::*;

let mut engine = EvolutionEngine::new(
    EvolutionConfig {
        population_size: 100,
        strategy_len: 20,
        mutation: Mutation::default_rates(),
        selection: SelectionMethod::tournament(),
        ..Default::default()
    }
);

let result: EvolutionResult = engine.run(500, |strategy| {
    // Fitness: reward strategies that match a target pattern
    let target = [2, 1, 0, 2, 1];
    strategy.iter().zip(target.iter())
        .map(|(&v, &t)| if v == t { 1.0 } else { 0.0 })
        .sum()
});
```

## API

### Core Types
- `TernaryAgent` — Strategy vector (Vec<u8> where each element ∈ {0,1,2}) + fitness
- `TernaryPopulation` — Collection of agents with statistics (best, worst, avg_fitness)
- `Environment` — Per-position fitness weights `w[position][value]`
- `FitnessFunction` — Single or multi-environment evaluation

### Genetic Operators
- `SelectionMethod` — Tournament{size}, RouletteWheel, RankBased{pressure}
- `Mutation` — point_rate + crossover config (SinglePoint / Uniform)
- `CrossoverMethod` — SinglePoint, Uniform

### Engine
- `EvolutionEngine` — Runs the GA loop with `EvolutionConfig`
- `EvolutionResult` — Per-generation best/avg fitness, convergence status
- `SpeciesTracker` — Clusters agents by similarity
- `ConvergenceDetector` — Detects fitness plateaus

### RNG
- `SimpleRng` — xorshift64 PRNG, no external dependencies

## Architecture Notes

The ternary strategy space connects to the γ + η = C conservation framework:

- **γ** (gamma) = the active population's genetic diversity (Shannon entropy of strategies)
- **η** (eta) = the selection pressure (information gained per generation)
- **C** (constant) = log₃(L) — the maximum entropy of a length-L ternary string

As evolution proceeds, γ decreases (population converges) while η increases (information accumulates). The sum γ + η approaches C at convergence — the system has extracted all available information from the fitness landscape.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## References

1. Holland, J.H. (1992). *Adaptation in Natural and Artificial Systems.* MIT Press. — Original genetic algorithm.
2. Goldberg, D.E. (1989). *Genetic Algorithms in Search, Optimization, and Machine Learning.* Addison-Wesley.
3. Mitchell, M. (1996). *An Introduction to Genetic Algorithms.* MIT Press.
4. Wang, H., et al. (2023). "BitNet: Scaling 1-bit Transformers for Large Language Models." *arXiv:2310.11453.* — Ternary weight quantization.
5. Luke, S. (2013). *Essentials of Metaheuristics,* 2nd ed. [cs.gmu.edu/~sean/book/metaheuristics](http://cs.gmu.edu/~sean/book/metaheuristics/)

## License

MIT
