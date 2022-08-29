use macroquad::prelude::*;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, radius: f32, vx: f32, vy: f32, color: Color) -> Self {
        Self { x, y, radius, vx, vy, color }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }
}
