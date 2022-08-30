use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Particle {
    pub pos: Vec2,
    pub radius: f32,
    pub v: Vec2,
    pub color: Color,
}

impl Particle {
    pub fn new(pos: Vec2, radius: f32, v: Vec2, color: Color) -> Self {
        Self { pos, radius, v, color }
    }

    pub fn distance_from(&self, other: &Self) -> Vec2 {
        self.pos - other.pos
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
