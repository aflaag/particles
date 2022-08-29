use particles::utils::*;

use macroquad::prelude::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SIZE: usize = 200;

const MAX_DIST: f32 = 80.0;
const K: f32 = 0.7;

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
    let mut red_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, RED);
    let mut green_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, GREEN);
    let mut yellow_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, YELLOW);

    loop {
        // clear the screen
        clear_background(BLACK);
        
        // perform interactions
        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &green_particles, MAX_DIST, K, -0.32);
        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &red_particles, MAX_DIST, K, -0.17);
        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &yellow_particles, MAX_DIST, K, 0.34);
        red_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(red_particles, &red_particles, MAX_DIST, K, -0.1);
        red_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(red_particles, &green_particles, MAX_DIST, K, -0.34);
        yellow_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(yellow_particles, &yellow_particles, MAX_DIST, K, 0.15);
        yellow_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(yellow_particles, &green_particles, MAX_DIST, K, -0.2);

        // draw particles
        red_particles.iter().for_each(|p| p.draw());
        green_particles.iter().for_each(|p| p.draw());
        yellow_particles.iter().for_each(|p| p.draw());

        next_frame().await
    }
}
