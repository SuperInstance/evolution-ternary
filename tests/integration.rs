//! Integration and unit tests.

#[cfg(test)]
mod tests {
    use evolution_ternary::*;

    // === Population tests ===

    #[test]
    fn test_agent_new_valid() {
        let agent = TernaryAgent::new(vec![0, 1, 2, 1, 0]);
        assert!(agent.is_some());
        assert_eq!(agent.unwrap().strategy, vec![0, 1, 2, 1, 0]);
    }

    #[test]
    fn test_agent_new_invalid() {
        let agent = TernaryAgent::new(vec![0, 1, 3]);
        assert!(agent.is_none());
    }

    #[test]
    fn test_agent_new_empty() {
        let agent = TernaryAgent::new(vec![]);
        assert!(agent.is_some());
        assert!(agent.unwrap().is_empty());
    }

    #[test]
    fn test_population_new_valid() {
        let agents = vec![
            TernaryAgent::new_unchecked(vec![0, 1, 2]),
            TernaryAgent::new_unchecked(vec![1, 2, 0]),
        ];
        let pop = TernaryPopulation::new(agents);
        assert!(pop.is_some());
        assert_eq!(pop.unwrap().size(), 2);
    }

    #[test]
    fn test_population_new_mismatched_lengths() {
        let agents = vec![
            TernaryAgent::new_unchecked(vec![0, 1, 2]),
            TernaryAgent::new_unchecked(vec![1, 2]),
        ];
        let pop = TernaryPopulation::new(agents);
        assert!(pop.is_none());
    }

    #[test]
    fn test_population_random() {
        let pop = TernaryPopulation::random(20, 10);
        assert_eq!(pop.size(), 20);
        assert_eq!(pop.strategy_len, 10);
        for agent in &pop.agents {
            for &v in &agent.strategy {
                assert!(v <= 2);
            }
        }
    }

    #[test]
    fn test_sort_by_fitness() {
        let agents = vec![
            TernaryAgent { strategy: vec![0, 0], fitness: 1.0 },
            TernaryAgent { strategy: vec![2, 2], fitness: 5.0 },
            TernaryAgent { strategy: vec![1, 1], fitness: 3.0 },
        ];
        let mut pop = TernaryPopulation::new(agents).unwrap();
        pop.sort_by_fitness();
        assert_eq!(pop.agents[0].fitness, 5.0);
        assert_eq!(pop.agents[1].fitness, 3.0);
        assert_eq!(pop.agents[2].fitness, 1.0);
    }

    #[test]
    fn test_best_worst() {
        let agents = vec![
            TernaryAgent { strategy: vec![0], fitness: 1.0 },
            TernaryAgent { strategy: vec![1], fitness: 5.0 },
            TernaryAgent { strategy: vec![2], fitness: 3.0 },
        ];
        let pop = TernaryPopulation::new(agents).unwrap();
        assert_eq!(pop.best().fitness, 5.0);
        assert_eq!(pop.worst().fitness, 1.0);
    }

    #[test]
    fn test_avg_fitness() {
        let agents = vec![
            TernaryAgent { strategy: vec![0], fitness: 2.0 },
            TernaryAgent { strategy: vec![1], fitness: 4.0 },
        ];
        let pop = TernaryPopulation::new(agents).unwrap();
        assert!((pop.avg_fitness() - 3.0).abs() < 1e-10);
    }

    // === Fitness tests ===

    #[test]
    fn test_simple_environment() {
        let env = Environment::simple(3);
        assert_eq!(env.evaluate(&[0, 0, 0]), 0.0);
        assert_eq!(env.evaluate(&[1, 1, 1]), 3.0);
        assert_eq!(env.evaluate(&[2, 2, 2]), 6.0);
    }

    #[test]
    fn test_diagonal_environment() {
        let env = Environment::diagonal(3);
        // Position 0 prefers 0, position 1 prefers 1, position 2 prefers 2
        let perfect = env.evaluate(&[0, 1, 2]);
        let wrong = env.evaluate(&[2, 0, 1]);
        assert!(perfect > wrong);
    }

    #[test]
    fn test_zeros_environment() {
        let env = Environment::zeros(3);
        assert!(env.evaluate(&[0, 0, 0]) > env.evaluate(&[1, 1, 1]));
    }

    #[test]
    fn test_multi_environment_sum() {
        let fn_ = FitnessFunction::multi_sum(vec![
            Environment::simple(2),
            Environment::zeros(2),
        ]);
        // Agent with [0,0]: simple=0+0=0, zeros=2+2=4, total=4
        assert_eq!(fn_.evaluate(&[0, 0]), 4.0);
    }

    #[test]
    fn test_multi_environment_max() {
        let fn_ = FitnessFunction::multi_max(vec![
            Environment::simple(2),
            Environment::zeros(2),
        ]);
        // Agent [2,2]: simple=4, zeros=1, max=4
        assert_eq!(fn_.evaluate(&[2, 2]), 4.0);
    }

    // === Selection tests ===

    #[test]
    fn test_tournament_prefers_fitter() {
        use population::SimpleRng;
        let agents = vec![
            TernaryAgent { strategy: vec![0], fitness: 0.1 },
            TernaryAgent { strategy: vec![1], fitness: 0.2 },
            TernaryAgent { strategy: vec![2], fitness: 100.0 },
        ];
        let pop = TernaryPopulation::new(agents).unwrap();
        let mut rng = SimpleRng::from_seed(42);
        let sel = SelectionMethod::Tournament { size: 3 };
        let parents = sel.select(&pop, 20, &mut rng);
        // Tournament should heavily favor the fittest agent
        let best_count = parents.iter().filter(|p| p.fitness == 100.0).count();
        assert!(best_count >= 10, "Expected most parents to be the fittest, got {}/20", best_count);
    }

    #[test]
    fn test_roulette_wheel() {
        use population::SimpleRng;
        let agents: Vec<TernaryAgent> = (0..10)
            .map(|i| TernaryAgent { strategy: vec![i as u8 % 3], fitness: i as f64 })
            .collect();
        let pop = TernaryPopulation::new(agents).unwrap();
        let mut rng = SimpleRng::from_seed(42);
        let sel = SelectionMethod::RouletteWheel;
        let parents = sel.select(&pop, 20, &mut rng);
        assert_eq!(parents.len(), 20);
    }

    #[test]
    fn test_rank_based() {
        use population::SimpleRng;
        let agents: Vec<TernaryAgent> = (0..5)
            .map(|i| TernaryAgent { strategy: vec![i as u8 % 3], fitness: (5 - i) as f64 })
            .collect();
        let pop = TernaryPopulation::new(agents).unwrap();
        let mut rng = SimpleRng::from_seed(42);
        let sel = SelectionMethod::rank_based();
        let parents = sel.select(&pop, 10, &mut rng);
        assert_eq!(parents.len(), 10);
    }

    // === Mutation tests ===

    #[test]
    fn test_point_mutation_changes_values() {
        use population::SimpleRng;
        let mut agent = TernaryAgent::new_unchecked(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let mut rng = SimpleRng::from_seed(42);
        let mutation = Mutation { point_rate: 1.0, crossover: mutation::CrossoverMethod::SinglePoint, crossover_rate: 0.0 };
        mutation.point_mutate(&mut agent, &mut rng);
        // With 100% rate, all values should change (from 0)
        assert!(agent.strategy.iter().any(|&v| v != 0));
        // All values still ternary
        assert!(agent.strategy.iter().all(|&v| v <= 2));
    }

    #[test]
    fn test_crossover_single_point() {
        use population::SimpleRng;
        let p1 = TernaryAgent::new_unchecked(vec![0, 0, 0, 0, 0]);
        let p2 = TernaryAgent::new_unchecked(vec![2, 2, 2, 2, 2]);
        let mut rng = SimpleRng::from_seed(42);
        let mutation = Mutation { point_rate: 0.0, crossover: mutation::CrossoverMethod::SinglePoint, crossover_rate: 1.0 };
        let (c1, c2) = mutation.crossover(&p1, &p2, &mut rng);
        // Children should be different from parents (mixed)
        assert_ne!(c1.strategy, p1.strategy);
        assert_ne!(c2.strategy, p2.strategy);
    }

    #[test]
    fn test_crossover_uniform() {
        use population::SimpleRng;
        let p1 = TernaryAgent::new_unchecked(vec![0, 0, 0, 0]);
        let p2 = TernaryAgent::new_unchecked(vec![2, 2, 2, 2]);
        let mut rng = SimpleRng::from_seed(42);
        let mutation = Mutation { point_rate: 0.0, crossover: mutation::CrossoverMethod::Uniform, crossover_rate: 1.0 };
        let (c1, c2) = mutation.crossover(&p1, &p2, &mut rng);
        assert!(c1.strategy.iter().all(|&v| v <= 2));
        assert!(c2.strategy.iter().all(|&v| v <= 2));
    }

    // === Species tests ===

    #[test]
    fn test_species_explorer() {
        let agent = TernaryAgent::new_unchecked(vec![0, 0, 0, 0, 0, 1]);
        assert_eq!(Species::classify(&agent), Species::Explorer);
    }

    #[test]
    fn test_species_diplomat() {
        let agent = TernaryAgent::new_unchecked(vec![1, 1, 1, 1, 0, 2]);
        assert_eq!(Species::classify(&agent), Species::Diplomat);
    }

    #[test]
    fn test_species_warrior() {
        let agent = TernaryAgent::new_unchecked(vec![2, 2, 2, 2, 2, 1]);
        assert_eq!(Species::classify(&agent), Species::Warrior);
    }

    #[test]
    fn test_species_hybrid() {
        let agent = TernaryAgent::new_unchecked(vec![0, 1, 2, 0, 1, 2]);
        assert_eq!(Species::classify(&agent), Species::Hybrid);
    }

    #[test]
    fn test_species_tracker() {
        let agents = vec![
            TernaryAgent::new_unchecked(vec![0, 0, 0, 0]),
            TernaryAgent::new_unchecked(vec![1, 1, 1, 1]),
            TernaryAgent::new_unchecked(vec![2, 2, 2, 2]),
        ];
        let mut tracker = SpeciesTracker::new();
        let counts = tracker.record(&agents);
        assert_eq!(counts, (1, 1, 1, 0));
        assert_eq!(tracker.diversity(), 3);
    }

    #[test]
    fn test_species_stability() {
        let agents = vec![
            TernaryAgent::new_unchecked(vec![0, 0, 0, 0]),
        ];
        let mut tracker = SpeciesTracker::new();
        for _ in 0..5 {
            tracker.record(&agents);
        }
        assert!(tracker.is_stable(3));
        assert!(!tracker.is_stable(6));
    }

    // === Convergence tests ===

    #[test]
    fn test_convergence_detector_plateau() {
        let mut detector = ConvergenceDetector::new(5, 0.001);
        for _ in 0..10 {
            detector.record(10.0);
        }
        let tracker = SpeciesTracker::new();
        let status = detector.check(&tracker);
        assert!(status.converged);
        assert!(status.plateau_generations >= 5);
    }

    #[test]
    fn test_convergence_detector_improving() {
        let mut detector = ConvergenceDetector::new(5, 0.001);
        for i in 0..10 {
            detector.record(i as f64 * 10.0);
        }
        let tracker = SpeciesTracker::new();
        let status = detector.check(&tracker);
        assert!(!status.converged);
    }

    // === Engine integration tests ===

    #[test]
    fn test_engine_runs() {
        let pop = TernaryPopulation::random_seeded(20, 8, 42);
        let config = EvolutionConfig::default_config();
        let fitness_fn = FitnessFunction::single(Environment::simple(8));
        let mut engine = EvolutionEngine::new(config, fitness_fn);
        let result = engine.run(pop);
        assert!(result.generations_run > 0);
        assert!(result.best_fitness > 0.0);
        assert_eq!(result.best_strategy.len(), 8);
        assert!(!result.fitness_history.is_empty());
    }

    #[test]
    fn test_engine_converges_to_high_fitness() {
        let pop = TernaryPopulation::random_seeded(30, 6, 123);
        let config = EvolutionConfig {
            mutation: Mutation {
                point_rate: 0.05,
                crossover: mutation::CrossoverMethod::SinglePoint,
                crossover_rate: 0.8,
            },
            max_generations: 200,
            elitism: 4,
            seed: 42,
            ..EvolutionConfig::default_config()
        };
        let fitness_fn = FitnessFunction::single(Environment::simple(6));
        let mut engine = EvolutionEngine::new(config, fitness_fn);
        let result = engine.run(pop);
        // With strong selection pressure, should converge close to optimal (6 * 2.0 = 12.0)
        assert!(result.best_fitness >= 8.0, "Best fitness was {}, expected >= 8.0", result.best_fitness);
    }

    #[test]
    fn test_engine_multi_environment() {
        let pop = TernaryPopulation::random_seeded(25, 5, 99);
        let config = EvolutionConfig::default_config();
        let fitness_fn = FitnessFunction::multi_sum(vec![
            Environment::simple(5),
            Environment::diagonal(5),
        ]);
        let mut engine = EvolutionEngine::new(config, fitness_fn);
        let result = engine.run(pop);
        assert!(result.generations_run > 0);
        assert!(result.best_fitness > 0.0);
    }

    #[test]
    fn test_deterministic_with_seed() {
        let pop1 = TernaryPopulation::random_seeded(15, 4, 77);
        let pop2 = TernaryPopulation::random_seeded(15, 4, 77);
        assert_eq!(pop1.agents, pop2.agents);

        let config = EvolutionConfig::default_config();
        let fitness_fn = FitnessFunction::single(Environment::simple(4));

        let mut e1 = EvolutionEngine::new(config.clone(), fitness_fn.clone());
        let mut e2 = EvolutionEngine::new(config, fitness_fn);

        let r1 = e1.run(pop1);
        let r2 = e2.run(pop2);
        assert_eq!(r1.best_strategy, r2.best_strategy);
        assert_eq!(r1.best_fitness, r2.best_fitness);
    }

    #[test]
    fn test_seeded_reproducibility() {
        let a = TernaryPopulation::random_seeded(10, 5, 42);
        let b = TernaryPopulation::random_seeded(10, 5, 42);
        for (x, y) in a.agents.iter().zip(b.agents.iter()) {
            assert_eq!(x.strategy, y.strategy);
        }
    }
}
