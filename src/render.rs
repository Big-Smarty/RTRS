use std::time::SystemTime;

use apecs::*;
use glam::Vec3A;
use image::{DynamicImage, GenericImageView, RgbImage};

use crate::camera::Camera;
use crate::color::Rgb;
use crate::primitives::sphere::Sphere;
use crate::ray::Ray;
use crate::viewport::Viewport;

pub fn render(
    (camera, viewport, mut image, q_spheres): (
        ViewMut<Camera>,
        ViewMut<Viewport>,
        ViewMut<DynamicImage>,
        Query<(&Vec3A, &f32, &Rgb)>,
    ),
) -> Result<(), GraphError> {
    let now = SystemTime::now();
    let mut spheres: Vec<Sphere> = Vec::new();
    for qs in q_spheres.query().iter_mut() {
        spheres.push(Sphere::new(**qs.0, **qs.1, **qs.2));
    }
    *image = image::DynamicImage::ImageRgb8(RgbImage::from_par_fn(
        image.dimensions().0,
        image.dimensions().1,
        |x, y| {
            let mut p = Rgb::default();
            for s in spheres.iter() {
                let pixel_center = viewport.root_pixel(
                    *camera.position(),
                    *camera.focal_length(),
                    image.dimensions(),
                ) + (viewport.delta_u(image.dimensions().0) * x as f32)
                    + (viewport.delta_v(image.dimensions().1) * y as f32);
                let ray_direction = pixel_center - camera.position();
                let ray = Ray::new(*camera.position(), ray_direction);
                p = ray
                    .get_color(crate::hittable::Intersectable::Sphere(*s))
                    .into()
            }
            p.into()
        },
    ));
    debug!("Render time: {}ms", now.elapsed().unwrap().as_millis());
    image.save("test.png").unwrap();
    ok()
}
