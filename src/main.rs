use particles::utils::*;

use macroquad::prelude::*;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn generate_window() -> Conf {
    Conf {
        window_title: "Particles".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(generate_window)]
async fn main() {
    let mut rng = ::rand::thread_rng();

    // generate particles
    // TODO: add mass maybe? very cool
    let mut particles = generate_particles::<WIDTH, HEIGHT, _>(20, &mut rng, RadiusOption::Constant(5.0), WHITE);

    loop {
        // clear the screen
        clear_background(BLACK);
        
        let particles_clone = particles.clone();

        for p1 in &mut particles {
            let mut grav_force = Vec2::ZERO;

            for p2 in &particles_clone {
                let first = p1.dist_x(p2);
                let second = p1.dist_y(p2);

                let distance = first * first + second * second;

                if distance > 0.0 {
                    let part = G / distance;

                    grav_force += part * Vec2::new(first, second);
                }
            }

            p1.x = grav_force.x;
            p1.y = grav_force.y;
        }

        // draw particles
        particles.iter().for_each(|p| p.draw());

        next_frame().await
    }
}
