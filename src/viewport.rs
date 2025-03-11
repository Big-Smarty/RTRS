use std::default::Default;
use std::fmt::Display;

use apecs::*;
use glam::Vec3A;

#[derive(Edges)]
pub struct Viewport {
    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            width: aspect_ratio * 2.0,
            height: 2.0,
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn u(&self) -> Vec3A {
        Vec3A::new(self.width, 0.0, 0.0)
    }

    pub fn v(&self) -> Vec3A {
        Vec3A::new(0.0, -self.height, 0.0)
    }

    pub fn delta_u(&self, image_width: u32) -> Vec3A {
        self.u() / image_width as f32
    }

    pub fn delta_v(&self, image_height: u32) -> Vec3A {
        self.v() / image_height as f32
    }

    pub fn upper_left(&self, camera_position: Vec3A, focal_length: f32) -> Vec3A {
        camera_position - Vec3A::new(0.0, 0.0, focal_length) - self.u() / 2.0 - self.v() / 2.0
    }

    pub fn root_pixel(
        &self,
        camera_position: Vec3A,
        focal_length: f32,
        image_dimensions: (u32, u32),
    ) -> Vec3A {
        self.upper_left(camera_position, focal_length)
            + Vec3A::new(0.5, 0.5, 0.5)
                * (self.delta_u(image_dimensions.0) + self.delta_v(image_dimensions.1))
    }
}

impl Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width: {} height: {}", self.width(), self.height())
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 16.0 / 9.0 * 2.0,
            height: 2.0,
        }
    }
}
