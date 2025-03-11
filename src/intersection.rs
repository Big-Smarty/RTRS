use glam::Vec3A;

use crate::color::Rgb;

pub enum IntersectionType {
    FrontFace(Intersection),
    BackFace(Intersection),
}

impl IntersectionType {
    pub fn get(&self) -> &Intersection {
        match self {
            Self::FrontFace(x) => x,
            Self::BackFace(y) => y,
        }
    }
}

pub struct Intersection {
    position: Vec3A,
    normal: Vec3A,
    distance: f32,
    albedo: Rgb,
}

impl Intersection {
    pub fn new(position: Vec3A, normal: Vec3A, distance: f32, albedo: Rgb) -> Self {
        Self {
            position,
            normal,
            distance,
            albedo,
        }
    }
    pub fn position(&self) -> Vec3A {
        self.position
    }
    pub fn normal(&self) -> Vec3A {
        self.normal
    }
    pub fn distance(&self) -> f32 {
        self.distance
    }
    pub fn albedo(&self) -> Rgb {
        self.albedo
    }
}

pub fn closest(x: IntersectionType, y: IntersectionType) -> IntersectionType {
    if x.get().distance() >= y.get().distance() {
        return y;
    }
    return x;
}
pub fn closest_optional(
    x: Option<IntersectionType>,
    y: Option<IntersectionType>,
) -> Option<IntersectionType> {
    if x.is_some() && y.is_some() {
        return Some(closest(x.unwrap(), y.unwrap()));
    }
    if x.is_some() {
        return x;
    }
    if y.is_some() {
        return y;
    }
    None
}
