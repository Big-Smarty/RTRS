use crate::intersection;
use crate::primitives::sphere::Sphere;
use crate::ray;

pub trait Hittable {
    fn intersects(&self, ray: &ray::Ray) -> bool;
    fn intersection(&self, ray: &ray::Ray) -> Option<intersection::IntersectionType>;
}

pub enum Intersectable {
    Sphere(Sphere),
}
