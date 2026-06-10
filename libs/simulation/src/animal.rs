use nalgebra as na;
use rand::{Rng, RngExt};

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
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
