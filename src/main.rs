use particles::particles::Particles;

use macroquad::{prelude::*, ui::{widgets, root_ui}, hash};
use ::rand::Rng;

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
    let max_dimension = WIDTH.min(HEIGHT) as f32;

    let mut rng = ::rand::thread_rng();

    let mut red_particles = Particles::<WIDTH, HEIGHT, SIZE>::from_random(&mut rng, 3.0, RED);
    let mut green_particles = Particles::<WIDTH, HEIGHT, SIZE>::from_random(&mut rng, 3.0, GREEN);
    let mut yellow_particles = Particles::<WIDTH, HEIGHT, SIZE>::from_random(&mut rng, 3.0, YELLOW);

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

    let pos = Vec2::new(100., 100.);
    let size = Vec2::new(470., 280.);

    let rand_k_pos = Some(Vec2::new(100., 2.));
    let rand_max_dist_pos = Some(Vec2::new(200., 2.));

    loop {
        clear_background(BLACK);
        
        widgets::Window::new(hash!(), pos, size)
            .label("Parameters")
            .movable(true)
            .ui(&mut *root_ui(), |ui| {
                if ui.button(None, "Randomize!") {
                    slider1 = rng.gen_range(-1f32..1f32);
                    slider2 = rng.gen_range(-1f32..1f32);
                    slider3 = rng.gen_range(-1f32..1f32);
                    slider4 = rng.gen_range(-1f32..1f32);
                    slider5 = rng.gen_range(-1f32..1f32);
                    slider6 = rng.gen_range(-1f32..1f32);
                    slider7 = rng.gen_range(-1f32..1f32);
                    slider8 = rng.gen_range(-1f32..1f32);
                    slider9 = rng.gen_range(-1f32..1f32);
                }

                if ui.button(rand_k_pos, "Randomize K") {
                    k = rng.gen_range(0f32..1f32);
                }

                if ui.button(rand_max_dist_pos, "Randomize Max Dist (not recommended)") {
                    max_dist = rng.gen_range(0f32..max_dimension);
                }

                ui.slider(hash!(), "Max Dist", 0f32..max_dimension, &mut max_dist);
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

        green_particles.self_interact(max_dist, k, slider1);
        green_particles.interact_with(&red_particles, max_dist, k, slider2);
        green_particles.interact_with(&yellow_particles, max_dist, k, slider3);
        red_particles.self_interact(max_dist, k, slider4);
        red_particles.interact_with(&green_particles, max_dist, k, slider5);
        yellow_particles.self_interact(max_dist, k, slider6);
        yellow_particles.interact_with(&green_particles, max_dist, k, slider7);
        red_particles.interact_with(&yellow_particles, max_dist, k, slider8);
        yellow_particles.interact_with(&red_particles, max_dist, k, slider9);

        red_particles.draw();
        green_particles.draw();
        yellow_particles.draw();

        next_frame().await
    }
}
