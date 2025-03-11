#![no_std]

use spirv_std::glam::{UVec3, Vec3};
use spirv_std::{glam, image, spirv};

#[spirv(compute(threads(64)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(descriptor_set = 0, binding = 0)] img: &image::Image2d,
) {
}
