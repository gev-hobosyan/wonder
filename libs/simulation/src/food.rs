use nalgebra as na;
use rand::{Rng, RngExt};

#[derive(Debug)]
pub struct Food {
    pub(crate) position: na::Point2<f32>,
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
