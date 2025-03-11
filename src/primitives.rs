use glam::Vec3A;

use crate::color::Rgb;
use crate::hittable::Hittable;

pub mod sphere;

pub trait Primitive: Hittable {
    fn position(&self) -> Vec3A;
    fn scale(&self) -> Vec3A;
    fn albedo(&self) -> Rgb;
}
