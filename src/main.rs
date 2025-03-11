#![feature(trivial_bounds)]

#[macro_use]
extern crate log;

use std::fs::File;

use apecs::*;
use chrono::prelude::*;
use glam::Vec3A;
use image::DynamicImage;
use simplelog::*;

mod camera;
mod color;
mod context;
mod hittable;
mod intersection;
mod primitives;
mod ray;
mod render;
mod scene;
mod viewport;

use render::render;
use scene::init_scene;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create(format!("{}-rtrs.log", Utc::now())).unwrap(),
        ),
    ])
    .unwrap();

    let mut world = World::default();
    world.add_resource(camera::Camera::new(
        Vec3A::splat(0.0),
        Vec3A::new(0.0, 0.0, -1.0),
        1.0,
    ));

    world.add_resource(viewport::Viewport::new(16.0 / 9.0));
    world.add_resource(DynamicImage::new_rgb8(1920, 1080));
    world.add_resource(context::Context::<context::vulkan_context::VulkanContext>::default());
    world.add_subgraph(
        graph!(init_scene)
            .with_barrier()
            .with_subgraph(graph!(render)),
    );
    world.tick().unwrap();
    world.tick().unwrap();
}
