use std::default::Default;
use std::fmt::Display;

use apecs::*;
use glam::Vec3A;

#[derive(Edges)]
pub struct Camera {
    position: Vec3A,
    direction: Vec3A,
    focal_length: f32,
}

impl Camera {
    pub fn new(position: Vec3A, direction: Vec3A, focal_length: f32) -> Self {
        Self {
            position,
            direction,
            focal_length,
        }
    }
    pub fn position(&self) -> &Vec3A {
        &self.position
    }
    pub fn direction(&self) -> &Vec3A {
        &self.direction
    }
    pub fn focal_length(&self) -> &f32 {
        &self.focal_length
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos: {}\ndir: {}\nfl: {}",
            self.position(),
            self.direction(),
            self.focal_length()
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3A::default(),
            direction: Vec3A::default().with_z(-1.0),
            focal_length: 1.0,
        }
    }
}
