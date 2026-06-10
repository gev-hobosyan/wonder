use lib_genetic_algorithm as ga;
use lib_genetic_algorithm::Individual;

use crate::*;

#[derive(Debug)]
pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> AnimalIndividual {
        Self {
            fitness: animal.eaten_foods as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn Rng) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl Individual for AnimalIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}
