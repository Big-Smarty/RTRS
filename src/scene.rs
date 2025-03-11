use apecs::*;
use glam::*;

use crate::{color::Rgb, primitives::sphere::Sphere};

pub fn init_scene(mut entities: ViewMut<Entities>) -> Result<(), GraphError> {
    debug!("Scene is initialized");
    let e = entities.create();
    e.with_bundle(
        Sphere::new(
            Vec3A::new(0.0, 0.0, -1.0),
            0.5,
            Rgb::new(U8Vec3::splat(255)),
        )
        .get(),
    );
    ok()
}
