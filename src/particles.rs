use crate::particle::Particle;

use macroquad::prelude::*;
use ::rand::Rng;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

#[derive(Debug, Clone, Copy)]
pub struct Particles<const W: usize, const H: usize, const S: usize> {
    particles: [Particle; S],
}

impl<const W: usize, const H: usize, const S: usize> Particles<W, H, S> {
    pub fn from_random<R: Rng + ?Sized>(rng: &mut R, radius: f32, color: Color) -> Self {
        let mut particles = [Particle::default(); S];

        particles
            .iter_mut()
            .for_each(|p| {
                *p = Particle::new(
                    Vec2::new(rng.gen_range(0f32..W as f32), rng.gen_range(0f32..H as f32)),
                    radius,
                    Vec2::ZERO,
                    color,
                )
            });

        Self { particles }
    }

    pub fn draw(&self) {
        self.particles.iter().for_each(|p| p.draw())
    }

    pub fn self_interact(&mut self, max_dist: f32, k: f32, g: f32) {
        self.interact_with(&Self::from(self.particles), max_dist, k, g)
    }

    pub fn interact_with(&mut self, other_particles: &Self, max_dist: f32, k: f32, g: f32) {
        self.particles
            .par_iter_mut()
            .for_each(|p1| {
                other_particles
                    .particles
                    .iter()
                    .for_each(|p2| {
                        let d = p1.distance_from(p2);

                        let distance = (d.x * d.x + d.y * d.y).sqrt();

                        if distance > 0.0 && distance < max_dist {
                            p1.v += g / distance * d;
                        }
                    });

                p1.v *= k;

                p1.pos += p1.v;

                if p1.pos.x < 0.0 || p1.pos.x > W as f32 {
                    p1.v.x *= -1.0;
                }

                if p1.pos.y < 0.0 || p1.pos.y > H as f32 {
                    p1.v.y *= -1.0;
                }
        });
    }
}

impl<const W: usize, const H: usize, const S: usize> From<[Particle; S]> for Particles<W, H, S> {
    fn from(particles: [Particle; S]) -> Self {
        Self { particles }
    }
}
