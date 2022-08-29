use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn dist_x(&self, other: &Self) -> f32 {
        self.x - other.x
    }

    pub fn dist_y(&self, other: &Self) -> f32 {
        self.y - other.y
    }

    pub fn draw(&self) {
        draw_circle(self.x + 250.0, self.y + 250.0, self.radius, self.color);
    }
}
