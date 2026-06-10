use nalgebra as na;
use rand::{Rng, RngExt};

use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eaten_foods: usize,
    pub(crate) brain: Brain,
    pub(crate) eye: Eye,
}

impl Animal {
    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn Rng) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn random(rng: &mut dyn Rng) -> Self {
        let eye = Eye::default();

        let brain = Brain::random(rng, &eye);

        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eaten_foods: 0,
            eye,
            brain,
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn Rng) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
            eye,
            brain,
            eaten_foods: 0,
        }
    }

    pub fn position(&self) -> nalgebra::OPoint<f32, nalgebra::Const<2>> {
        self.position
    }

    pub fn rotation(&self) -> nalgebra::Rotation<f32, 2> {
        self.rotation
    }
}
