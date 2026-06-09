use nalgebra as na;
use rand::{Rng, RngExt};

#[derive(Debug)]
pub struct Simulation {
    world: World,
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}

#[derive(Debug)]
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
}

#[derive(Debug)]
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}

impl World {
    pub fn random(rng: &mut dyn Rng) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self {
            position: rng.random(),
            rotation: rng.random(),
            speed: 0.002,
        }
    }

    pub fn position(&self) -> nalgebra::OPoint<f32, nalgebra::Const<2>> {
        self.position
    }

    pub fn rotation(&self) -> nalgebra::Rotation<f32, 2> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn Rng) -> Self {
        Self {
            position: rng.random(),
        }
    }

    pub fn position(&self) -> nalgebra::OPoint<f32, nalgebra::Const<2>> {
        self.position
    }
}
