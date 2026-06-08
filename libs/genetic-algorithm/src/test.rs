use std::collections::BTreeMap;

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    Chromosome, CrossoverMethod, GaussianMutation, GeneticAlgorithm, Individual, MutationMethod,
    RandomWheelSelection, SelectionMethod, UniformCrossover,
};

#[derive(Clone, Debug, PartialEq)]
enum TestIndividual {
    WithFitness { fitness: f32 },
    WithCromosome { chromosome: Chromosome },
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}

impl TestIndividual {
    pub fn new(fitness: f32) -> TestIndividual {
        Self::WithFitness { fitness }
    }
}

impl Individual for TestIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithCromosome { chromosome }
    }

    fn fitness(&self) -> f32 {
        match self {
            TestIndividual::WithFitness { fitness } => *fitness,
            TestIndividual::WithCromosome { chromosome } => chromosome.iter().sum(),
        }
    }

    fn chromosome(&self) -> &crate::Chromosome {
        match self {
            Self::WithFitness { .. } => {
                panic!("not supported for fitness type")
            }
            Self::WithCromosome { chromosome } => chromosome,
        }
    }
}

#[test]
pub fn random_wheel_selection() {
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let population = vec![
        TestIndividual::new(1.0),
        TestIndividual::new(2.0),
        TestIndividual::new(4.0),
        TestIndividual::new(3.0),
    ];

    let mut actual_histogram = BTreeMap::new();

    for _ in 0..1000 {
        let fitness = RandomWheelSelection.select(&mut rng, &population).fitness() as i32;

        *actual_histogram.entry(fitness).or_insert(0) += 1;
    }

    let expected_histogram = BTreeMap::from_iter([(1, 102), (2, 198), (3, 278), (4, 422)]);

    assert_eq!(expected_histogram, actual_histogram);
}

#[test]
pub fn uniform_crossover() {
    let mut rng = ChaCha8Rng::from_seed(Default::default());
    let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
    let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();
    let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

    let diff_a = child
        .iter()
        .zip(parent_a)
        .filter(|(child, parent)| *child != parent)
        .count();

    let diff_b = child
        .iter()
        .zip(parent_b)
        .filter(|(child, parent)| *child != parent)
        .count();

    assert_eq!(diff_a, 49);
    assert_eq!(diff_b, 51);
}

mod gaussian_mutation {
    use crate::{GaussianMutation, MutationMethod};

    use super::*;

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod add_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change() {
                let actual_val = actual(0.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual_val.as_slice(), expected_val.as_slice());
            }
        }

        mod add_non_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change() {
                let actual_val = actual(0.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual_val.as_slice(), expected_val.as_slice());
            }
        }
    }

    mod given_non_zero_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod add_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change() {
                let actual_val = actual(0.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual_val.as_slice(), expected_val.as_slice());
            }
        }

        mod add_non_zero_coeff {
            use super::*;

            #[test]
            fn slightly_changes() {
                let actual_val = actual(3.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                let diff = expected_val
                    .iter()
                    .zip(actual_val)
                    .filter(|(expected, actual)| *expected != actual)
                    .count();

                assert_ne!(diff, 0);
            }
        }
    }

    mod given_max_chance {
        use approx::assert_relative_eq;

        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod add_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change() {
                let actual_val = actual(0.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                assert_relative_eq!(actual_val.as_slice(), expected_val.as_slice());
            }
        }

        mod add_non_zero_coeff {
            use super::*;

            #[test]
            fn all_change() {
                let actual_val = actual(3.0);
                let expected_val = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                let diff = expected_val
                    .iter()
                    .zip(actual_val)
                    .filter(|(expected, actual)| *expected != actual)
                    .count();

                assert_ne!(diff, 0);
            }
        }
    }
}

#[test]
fn genetic_algorithm() {
    fn individual(genes: &[f32]) -> TestIndividual {
        TestIndividual::create(genes.iter().cloned().collect())
    }

    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let ga = GeneticAlgorithm::new(
        RandomWheelSelection,
        UniformCrossover,
        GaussianMutation::new(0.5, 0.5),
    );

    let mut population = vec![
        individual(&[0.0, 0.0, 0.0]),
        individual(&[1.0, 1.0, 1.0]),
        individual(&[1.0, 2.0, 1.0]),
        individual(&[1.0, 2.0, 4.0]),
    ];

    for _ in 0..10 {
        population = ga.evolve(&mut rng, &population);
    }

    let expected_population = vec![
        individual(&[0.4476949, 2.0648358, 4.3058133]),
        individual(&[1.2126867, 1.5538777, 2.886911]),
        individual(&[1.0617678, 2.265739, 4.428764]),
        individual(&[0.95909685, 2.4618788, 4.024733]),
    ];

    assert_eq!(population, expected_population);
}
