use std::io::Write;

use camera::Camera;
use colour::Colour;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vector3::Vector3;

use crate::{material::{Dieletric, Material}, random::{random_f64, random_f64_in_range}};

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
mod save;

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
    let material_ground = Box::new(Lambertian::new(Colour::from(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere { center: Vector3::from(0.0, -1000.0, 0.0), radius: 1000.0, material: material_ground}));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vector3::from(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

            if (center - Vector3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Colour::random() * Colour::random();
                    sphere_material = Box::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere {center, radius: 0.2, material: sphere_material}));
                } else if choose_mat < 0.8 {
                    let albedo = Colour::random_in_range(0.5..1.0);
                    let fuzz = random_f64_in_range(0.0..0.5);
                            
                    sphere_material = Box::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere {center, radius: 0.2, material: sphere_material}));
                } else {
                    sphere_material = Box::new(Dieletric::new(1.5));
                    world.add(Box::new(Sphere {center, radius: 0.2, material: sphere_material}));
                }
            }
        }
    }

    let material1 = Box::new(Dieletric::new(1.5));
    world.add(Box::new(Sphere {center: Vector3::from(0.0, 1.0, 0.0), radius: 1.0, material: material1}));

    let material2 = Box::new(Lambertian::new(Colour::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere {center: Vector3::from(-4.0, 1.0, 0.0), radius: 1.0, material: material2}));

    let material3 = Box::new(Metal::new(Colour::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere {center: Vector3::from(4.0, 1.0, 0.0), radius: 1.0, material: material3}));
     

    let mut cam: Camera = Camera {..Default::default()};
    
    // Render settings
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = width;
    cam.samples_per_pixel = 500;
    cam.depth_limit = 50;

    cam.vfov = 20.0;
    cam.look_from = Vector3::from(13.0, 2.0, 3.0);
    cam.look_at = Vector3::from(0.0, 0.0, 0.0);
    cam.up = Vector3::from(0.0, 1.0, 0.0);
    
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    
    // Render the scene
    cam.render(&mut world);
}
