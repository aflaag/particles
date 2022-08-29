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
    let mut particles = generate_particles::<WIDTH, HEIGHT, _>(2, &mut rng, RadiusOption::Constant(5.0), WHITE);

    loop {
        clear_background(BLACK);

        // draw particles
        particles.iter().for_each(|p| p.draw());

        next_frame().await
    }
}
