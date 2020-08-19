use crate::{
    Camera, CameraIterator, ComputeMeshes, ComputedMesh, Light, Mesh, Ray, SortByTime, Vert, IntersectionCompute,
    WorldIterator,
};
use image::{ImageBuffer, Rgba};
use std::cmp::{max, min};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

#[derive(Clone)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Arc<Light>>,
    pub meshes: Vec<Arc<Mesh>>,
}

pub fn col_at_ray(ray: &Ray, lights: &Vec<Arc<Light>>, computed_meshes: &Vec<Arc<ComputedMesh>>) -> Vert {
    let mut intersections = vec![];
    for mesh in computed_meshes.into_iter() {
        mesh.intersects_with(&ray, &mut intersections);
    }
    intersections.sort_by_time();
    let mut col: Option<Vert> = None;
    for light in lights.into_iter() {
        if let Some(first_intersection) = intersections.first() {
            let compute = IntersectionCompute::new(&ray, first_intersection);
            let shadowed = if light.casts_shadows {
                light.shadowed(compute.over_point.clone(), &computed_meshes)
            } else {
                false
            };
            let light_rgb = light.lighting(&compute, shadowed);
            col = match col {
                Some(prev_light_rgb) => Some(prev_light_rgb * light_rgb),
                None => Some(light_rgb),
            }
        }
    }
    match col {
        Some(col) => col,
        None => Vert::black(),
    }
}

fn render_thread(computed_meshes: Vec<Arc<ComputedMesh>>, lights: Vec<Arc<Light>>, camera_iter: CameraIterator) -> JoinHandle<Vec<u8>> {
    spawn(move || {
        let mut pixels = vec![];
        let iter = WorldIterator {
            computed_meshes,
            lights,
            camera_iter,
        };
        for rgb in iter {
            let rgba: [u8; 4] = rgb.into();
            pixels.append(&mut rgba.to_vec());
        }
        pixels
    })
}


impl World {
    pub fn new() -> World {
        World {
            camera: Camera::default(),
            lights: vec![],
            meshes: vec![],
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(Arc::new(light));
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(Arc::new(mesh));
    }

    pub fn render_to_png(&self, path: &str, number_of_threads: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut pixels = vec![];
        let computed_meshes = self.meshes.compute_meshes();
        let mut threads = vec![];
        let step = max((self.camera.height as f64 / number_of_threads as f64) as usize, 1);
        for start_y in (0..self.camera.height).step_by(step) {
            let computed_meshes = computed_meshes.clone();
            let lights = self.lights.clone();
            let end_y = min(start_y + step, self.camera.height);
            let camera_iter = self.camera.part_iter(start_y, end_y);
            let thread = render_thread(computed_meshes, lights, camera_iter);
            threads.push(thread);
        }
        for thread in threads {
            match thread.join() {
                Ok(mut vec) => pixels.append(&mut vec),
                Err(_) => todo!("error handling"),
            }
        }
        let (width, height) = self.camera.dimensions();
        match ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(width as u32, height as u32, pixels) {
            Some(img) => img.save(path)?,
            None => todo!("error handling"),
        }
        Ok(())
    }
}
