use crate::particle::Particle;

use macroquad::prelude::*;
use ::rand::Rng;

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
                    rng.gen_range(0f32..W as f32),
                    rng.gen_range(0f32..H as f32),
                    radius,
                    0.0,
                    0.0,
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
            .iter_mut()
            .for_each(|p1| {
                let mut grav_force = Vec2::ZERO;

                for p2 in other_particles.particles {
                    let first = p1.dist_x(&p2);
                    let second = p1.dist_y(&p2);

                    let distance = (first * first + second * second).sqrt();

                    if distance > 0.0 && distance < max_dist {
                        grav_force += g / distance * Vec2::new(first, second);
                    }
                }

                p1.vx = (p1.vx + grav_force.x) * k;
                p1.vy = (p1.vy + grav_force.y) * k;
                p1.x += p1.vx;
                p1.y += p1.vy;

                if p1.x < 0.0 || p1.x > W as f32 {
                    p1.vx *= -1.0;
                }

                if p1.y < 0.0 || p1.y > H as f32 {
                    p1.vy *= -1.0;
                }
        });
    }
}

impl<const W: usize, const H: usize, const S: usize> From<[Particle; S]> for Particles<W, H, S> {
    fn from(particles: [Particle; S]) -> Self {
        Self { particles }
    }
}
