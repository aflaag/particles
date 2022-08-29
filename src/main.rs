use particles::utils::*;

use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SIZE: usize = 200;

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

    let mut red_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, RED);
    let mut green_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, GREEN);
    let mut yellow_particles = generate_particles::<WIDTH, HEIGHT, SIZE, _>(&mut rng, 3.0, YELLOW);

    let mut max_dist = 80.0;
    let mut k = 0.7;

    let mut slider1 = -0.32;
    let mut slider2 = -0.17;
    let mut slider3 = 0.34;
    let mut slider4 = -0.1;
    let mut slider5 = -0.34;
    let mut slider6 = 0.15;
    let mut slider7 = -0.2;
    let mut slider8 = 0.0;
    let mut slider9 = 0.0;

    loop {
        clear_background(BLACK);
        
        widgets::Window::new(hash!(), Vec2::new(100., 100.), Vec2::new(500., 250.))
            .label("Parameters")
            .movable(true)
            .ui(&mut *root_ui(), |ui| {
                ui.slider(hash!(), "Max Distance", 0f32..WIDTH as f32, &mut max_dist);
                ui.slider(hash!(), "K", 0f32..1f32, &mut k);

                ui.slider(hash!(), "G-G", -1f32..1f32, &mut slider1);
                ui.slider(hash!(), "G-R", -1f32..1f32, &mut slider2);
                ui.slider(hash!(), "G-Y", -1f32..1f32, &mut slider3);
                ui.slider(hash!(), "R-R", -1f32..1f32, &mut slider4);
                ui.slider(hash!(), "R-G", -1f32..1f32, &mut slider5);
                ui.slider(hash!(), "Y-Y", -1f32..1f32, &mut slider6);
                ui.slider(hash!(), "Y-G", -1f32..1f32, &mut slider7);
                ui.slider(hash!(), "R-Y", -1f32..1f32, &mut slider8);
                ui.slider(hash!(), "Y-R", -1f32..1f32, &mut slider9);
            });

        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &green_particles, max_dist, k, slider1);
        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &red_particles, max_dist, k, slider2);
        green_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(green_particles, &yellow_particles, max_dist, k, slider3);
        red_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(red_particles, &red_particles, max_dist, k, slider4);
        red_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(red_particles, &green_particles, max_dist, k, slider5);
        yellow_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(yellow_particles, &yellow_particles, max_dist, k, slider6);
        yellow_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(yellow_particles, &green_particles, max_dist, k, slider7);
        red_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(red_particles, &yellow_particles, max_dist, k, slider8);
        yellow_particles = perform_interaction::<WIDTH, HEIGHT, SIZE>(yellow_particles, &red_particles, max_dist, k, slider9);

        red_particles.iter().for_each(|p| p.draw());
        green_particles.iter().for_each(|p| p.draw());
        yellow_particles.iter().for_each(|p| p.draw());

        next_frame().await
    }
}
