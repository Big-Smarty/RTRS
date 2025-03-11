use std::fmt::Display;

use glam::Vec3A;

use crate::color::Rgb;
use crate::hittable::Hittable;
use crate::intersection;
use crate::primitives::Primitive;

#[derive(Copy, Clone)]
pub struct Sphere((Vec3A, f32, Rgb));

impl Sphere {
    pub fn new(position: Vec3A, radius: f32, albedo: Rgb) -> Self {
        Self((position, radius, albedo))
    }
    pub fn get(&self) -> (Vec3A, f32, Rgb) {
        self.0
    }
    pub fn radius(&self) -> f32 {
        self.0.1
    }
}

impl Primitive for Sphere {
    fn position(&self) -> Vec3A {
        self.0.0
    }

    fn scale(&self) -> Vec3A {
        Vec3A::splat(self.radius())
    }

    fn albedo(&self) -> Rgb {
        self.0.2
    }
}

impl Hittable for Sphere {
    fn intersects(&self, ray: &crate::ray::Ray) -> bool {
        let oc = self.position() - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius().powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 { false } else { true }
    }

    fn intersection(&self, ray: &crate::ray::Ray) -> Option<intersection::IntersectionType> {
        let oc = self.position() - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius().powi(2);
        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let distance = (h - discriminant.sqrt()) / a;
            return Some(intersection::IntersectionType::FrontFace(
                intersection::Intersection::new(
                    ray.at(distance),
                    ray.at(distance) - self.position(),
                    distance,
                    self.albedo(),
                ),
            ));
        }
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "pos: {0}\nrad: {1}\nalbedo: {2}",
            self.get().0,
            self.get().1,
            self.get().2,
        )
    }
}
