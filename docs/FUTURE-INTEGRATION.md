# Future Integration: evolution-ternary

## Current State
Evolutionary dynamics on ternary strategy spaces — natural selection as sort, mutation as autofill, fitness landscapes as heatmaps. Models ternary agents with strategy vectors from {0, 1, 2}, supporting tournament/roulette/rank selection, point mutation, crossover, autofill mutation, and species tracking (Explorer, Diplomat, Warrior, Hybrid).

## Integration Opportunities

### With room populations
Every room's cell population evolves via evolution-ternary. The `EvolutionEngine` runs generational loops: selection (which cells survive), mutation (how they change), and elitism (preserving the best). The room's tick cycle triggers one generation per N ticks. Cells that perform well persist; cells that perform poorly are replaced. Natural selection IS the room's optimization algorithm.

### With superinstance-spreadsheet
The "Evolve" button in the ternary spreadsheet triggers evolution-ternary's `EvolutionEngine`. Each row (a ternary strategy) is evaluated across columns (environments), fitness is computed, and the next generation is produced via selection + mutation. The spreadsheet IS the fitness landscape.

### With conservation-matrix-rs
Evolution must respect conservation laws. evolution-ternary's `ConvergenceDetector` should incorporate conservation checks: has the population's avoidance ratio drifted from the conserved value? Are all strategy species still present? Evolution within conservation constraints.

## Dormant Ideas Now Unlockable
The "sort = natural selection, autofill = mutation" metaphor was clever but lacked a runtime. Now ternary-cell provides the runtime: cells evolve within rooms, rooms evolve within the fleet. The metaphor becomes the mechanism.

## Potential in Mature Systems
Evolution is continuous. Every room evolves. The fleet evolves. New strategies emerge, old strategies die, and the overall fitness of the fleet increases over time. Natural selection at every scale — cell, room, fleet.

## Cross-Pollination Ideas
- **strategy-ecology**: Species classification for evolution-ternary's agents
- **lotka-volterra-agents**: Competitive dynamics between evolved populations
- **population-scaling**: How evolution changes with population size
- **superinstance-spreadsheet**: The UI for watching evolution happen

## Dependencies for Next Steps
- Integration with ternary-cell tick cycle for generational evolution
- Conservation constraints during evolution
- Cross-room strategy transfer via strategy-transfer
