use crate::particle::Particle;

use macroquad::prelude::*;
use ::rand::Rng;

pub fn generate_particles<const W: usize, const H: usize, const S: usize, R: Rng + ?Sized>(rng: &mut R, radius: f32, color: Color) -> [Particle; S] {
    let mut particles = [Particle::default(); S];

    particles
        .iter_mut()
        .for_each(|p| {
            *p = Particle::new(
                rng.gen_range(0..W) as f32,
                rng.gen_range(0..H) as f32,
                radius,
                0.0,
                0.0,
                color,
            )
        });

    particles
}

pub fn perform_interaction<const W: usize, const H: usize, const S: usize>(
    mut particles1: [Particle; S],
    particles2: &[Particle; S],
    max_dist: f32,
    k: f32,
    g: f32
) -> [Particle; S] {
    particles1
        .iter_mut()
        .for_each(|p1| {
            let mut grav_force = Vec2::ZERO;

            for p2 in particles2 {
                let first = p1.dist_x(p2);
                let second = p1.dist_y(p2);

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

    particles1
}
