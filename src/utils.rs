use crate::particle::Particle;

use macroquad::prelude::*;
use ::rand::Rng;

pub const G: f32 = 6.67 * 10e-11;

pub enum RadiusOption {
    Constant(f32),
    NonConstant,
}

pub fn generate_particles<const W: usize, const H: usize, R: Rng + ?Sized>(size: usize, rng: &mut R, radius: RadiusOption, color: Color) -> Vec<Particle> {
    (0..size)
        .map(|_| {
            Particle::new(
                rng.gen_range(0..W) as f32,
                rng.gen_range(0..H) as f32,
                match radius {
                    RadiusOption::Constant(r) => r,
                    RadiusOption::NonConstant => rng.gen(),
                },
                0.0,
                0.0,
                color,
            )
        })
        .collect()
}
