use std::io::Write;

use camera::Camera;
use colour::Colour;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vector3::Vector3;

mod vector3;
mod colour;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod interval;
mod camera;
mod random;
mod viewer;
mod material;

fn main() {
    let mut width_input = String::new();

    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[37m");
    print!("How wide do you want your image?: ");

    std::io::stdout().flush().unwrap();
        
    std::io::stdin().read_line(&mut width_input).unwrap();

    let width: i64;

    match width_input.trim().parse::<i64>() {
        Ok(val) => width = val,
        Err(err) => {
            eprintln!("ERROR: {}", err.to_string());
            std::process::exit(1);
        }
    }

    let mut world = HittableList::new();
    
    // Generate some basic materials
    let material_ground = Box::new(Lambertian::new(Colour::from(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Colour::from(0.7, 0.3, 0.3)));
    let material_left = Box::new(Metal::new(Colour::from(0.8, 0.8, 0.8), 0.3));
    let material_right = Box::new(Metal::new(Colour::from(0.8, 0.6, 0.2), 0.0));
    
    // Add some objects to our scene
    world.add(Box::new(Sphere{ center: Vector3::from(0.0, -100.5, -1.0), radius: 100.0, material: material_ground}));
    world.add(Box::new(Sphere{ center: Vector3::from(0.0, 0.0, -1.0), radius: 0.5, material: material_center}));
    world.add(Box::new(Sphere{ center: Vector3::from(-1.0, 0.0, -1.0), radius: 0.5, material: material_left}));
    world.add(Box::new(Sphere{ center: Vector3::from(1.0, 0.0, -1.0), radius: 0.5, material: material_right}));

    let mut cam: Camera = Camera {..Default::default()};
    
    // Render settings
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = 100;
    cam.depth_limit = 50;
    
    // Render the scene
    cam.render(&mut world);
}
