static MACHEPS: f64 = 0.00001;
use std::f64::consts::PI;
mod linear_algebra;
mod obj;
mod renderer;
use linear_algebra::*;
use obj::*;
use renderer::*;
use std::time::Instant;

pub fn render(world: &World, path: &str, number_of_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let instant = Instant::now();
    world.render_to_png(path, number_of_threads)?;
    let seconds = instant.elapsed().as_millis() as f64 / 1000.0;
    let (width, height) = world.camera.dimensions();
    println!("Rendered {} ({}x{}) using {} threads in {:.3}s", path, width, height, number_of_threads, seconds);
    Ok(())
}

pub fn spinning_plane() {
    let mut rads = 0.0;
    for i in 0..192 {
        let mut world = World::new();
        world.camera.fov = PI / 2.8;
        world.camera.set_dimensions(512, 512);
        world.camera.from = Vert::new(0.0, 0.0, -8.0);
        world.camera.to = Vert::new(0.0, 0.0, 0.0);
        let mut light = Light::new();
        light.position = Vert::new(-5.0, 8.0, -8.0);
        let mut mesh = Mesh::load_obj("objs/plane.obj").unwrap();
        mesh.rotation.y = PI / 2.0 + rads;
        mesh.scale = Vert::new(2.0, 2.0, 2.0);
        world.add_light(light);
        world.add_mesh(mesh);
        let path = format!("imgs/spinning_plane_{:04}.png", i);
        render(&world, &path, 16).unwrap();
        rads += PI * 2.0 / 192.0;
    }
}
