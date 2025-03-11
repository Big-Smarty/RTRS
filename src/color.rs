use std::convert::Into;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use glam::U8Vec3;
use image;

#[derive(Copy, Clone, Debug)]
pub struct Rgb {
    data: U8Vec3,
}

impl Rgb {
    pub fn new(data: U8Vec3) -> Self {
        Self { data }
    }
}

impl Deref for Rgb {
    type Target = U8Vec3;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Rgb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Self {
            data: U8Vec3::default(),
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " r: {} g: {} b: {} ", self.x, self.y, self.z)
    }
}

impl Into<image::Rgb<u8>> for Rgb {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb(self.to_array())
    }
}
