use std::fs;
use std::time::{Duration, Instant};

use pbr::ProgressBar;

use crate::vector3::random_unit_vector;
use crate::{hittable::{Hittable, HitRecord}, colour::Colour, interval::Interval, ray::Ray, vector3::{Vector3, random_on_hemisphere}, random::random_f64};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub image_height: i64,
    pub center: Vector3,
    pub pixel00_loc: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
    pub samples_per_pixel: i64,
    pub depth_limit: u64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Vector3::new(),
            pixel00_loc: Vector3::new(),
            pixel_delta_u: Vector3::new(),
            pixel_delta_v: Vector3::new(),
            samples_per_pixel: 10,
            depth_limit: 10,
        }
    }
}

impl Camera {
    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = Vector3::from(0.0, 0.0, 0.0);

        let viewport_height = 2.0;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Camera data
        let focal_length = 1.0;
        let camera_center = Vector3::new();

        // Calculate the vectors across the horizontal and down the vertical edges;
        let viewport_u = Vector3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::from(0.0, -viewport_height as f64, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate position of upper-left pixel
        let viewport_upper_left = camera_center - Vector3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &mut dyn Hittable) {
        let start_time = Instant::now();
        self.init();

        // Represents the final PPM data
        let mut data = String::new();

        // Create a progressbar to keep track of the render
        let mut pb = ProgressBar::new(self.image_height as u64);        

        // Push the header to the final data
        data.push_str(&format!("P3\n{} {}\n255\n", self.image_width, self.image_height));

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.depth_limit, world);
                }

                let mut pixel_data = String::new();
                pixel_colour.write_colour(&mut pixel_data, self.samples_per_pixel);


                // Write the pixel data to the output string
                data.push_str(&pixel_data); 
            }
            pb.inc();
        }
        let end_time = start_time.elapsed();
        pb.finish_println(&format!("Rendered in {} seconds", end_time.as_secs()));

        // Write the output string to the file
        fs::write("output.ppm", data).expect("Error writing to file");
    }

    fn ray_colour(&self, r: &Ray, depth_limit: u64, world: &mut dyn Hittable) -> Colour {
        if depth_limit <= 0 { return Colour::new() }

        let mut record = HitRecord::new();

        if world.hit(r, Interval::from(0.001, f64::INFINITY), &mut record) {
            let direction = record.normal + random_unit_vector();
            return 0.5 * self.ray_colour(&Ray::from(record.point, direction), depth_limit - 1, world);
        }

        let unit_dir = r.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        return (1.0-a) * Colour::from(1.0, 1.0, 1.0) + a * Colour::from(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_dir = pixel_sample - ray_origin;

        return Ray::from(ray_origin, ray_dir); 
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
