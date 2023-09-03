use camera::Camera;
use hittable_list::HittableList;
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

fn main() {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere{ center: Vector3::from(0.0, 0.0, -1.0), radius: 0.5}));
    world.add(Box::new(Sphere{ center: Vector3::from(0.0, -100.5, -1.0), radius: 100.0}));

    let mut cam: Camera = Camera {..Default::default()};

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&mut world);
}
