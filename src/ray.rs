use std::default::Default;

use glam::*;

use crate::color::Rgb;
use crate::hittable::{Hittable, Intersectable};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self { origin, direction }
    }

    pub fn get_color(&self, intersectable: Intersectable) -> Rgb {
        match intersectable {
            Intersectable::Sphere(sphere) => {
                let unit_direction = self.direction.normalize();
                match sphere.intersection(self) {
                    Some(i) => {
                        return Rgb::new(i.get().normal().map(|i| i * 255.0).as_u8vec3());
                    }
                    None => {
                        let a = (unit_direction + Vec3A::new(1.0, 1.0, 1.0)) * 0.5;
                        return Rgb::new(
                            ((Vec3A::new(1.0, 1.0, 1.0) - a) + a * Vec3A::new(0.5, 0.7, 1.0))
                                .map(|i| i * 255.0)
                                .as_u8vec3(),
                        );
                    }
                }
            }
        }
    }

    pub fn origin(&self) -> &Vec3A {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3A {
        &self.direction
    }

    pub fn at(&self, d: f32) -> Vec3A {
        self.origin + self.direction * d
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vec3A::default(),
            direction: Vec3A::default().with_z(-1.0),
        }
    }
}
