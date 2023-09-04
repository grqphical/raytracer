use std::time::Instant;
use pbr::ProgressBar;
use crate::save;
use crate::vector3::{cross_product, random_in_unit_disk};
use crate::viewer::show_image;
use crate::{hittable::{Hittable, HitRecord}, colour::Colour, interval::Interval, ray::Ray, vector3::Vector3, random::random_f64};

/// Calculates the average of a Vector of u128
fn average(numbers: &[u128]) -> u128 {
    let sum: u128 = numbers.iter().sum();
    let count: u128 = numbers.len() as u128;
    sum / count
}

/// Represents a camera in the raytracer
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
    pub vfov: f64,
    pub look_from: Vector3,
    pub look_at: Vector3,
    pub up: Vector3,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vector3,
    pub defocus_disk_v: Vector3,
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
            vfov: 90.0,
            look_from: Vector3::from(0.0, 0.0, -1.0),
            look_at: Vector3::from(0.0, 0.0, 0.0),
            up: Vector3::from(0.0, 1.0, 0.0),
            u: Vector3::new(),
            v: Vector3::new(),
            w: Vector3::new(),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            defocus_disk_u: Vector3::new(),
            defocus_disk_v: Vector3::new(),
        } 
    }
}

impl Camera {
    /// Initalize the camera 
    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = self.look_from; 

        // Camera data
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = (self.look_from - self.look_at).unit();
        self.u = cross_product(self.up, self.w).unit();
        self.v = cross_product(self.w, self.u);

        // Calculate the vectors across the horizontal and down the vertical edges;
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate position of upper-left pixel
        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    
    /// Render the scene
    ///
    /// ## Arguments
    ///
    /// - `world` HittableList of objects in the scene
    pub fn render(&mut self, world: &mut dyn Hittable) {
        let start_time = Instant::now();
        self.init();

        // Represents final pixel data
        let mut data: Vec<u32> = vec![];

        // Vector to store scanline times to be used to calculate average at end
        let mut scanline_durations: Vec<u128> = Vec::new();

        println!("\nStarting Render at {}x{} pixels", self.image_width, self.image_height);

        // Create a progressbar to keep track of the render
        let mut pb = ProgressBar::new(self.image_height as u64);
        pb.message("Rendering scanline: ");

        for j in 0..self.image_height {
            let scanline_start = Instant::now();
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += self.ray_colour(&r, self.depth_limit, world);
                }

                pixel_colour.write_colour_pixels(&mut data, self.samples_per_pixel);
                let scanline_duration = scanline_start.elapsed();
                scanline_durations.push(scanline_duration.as_millis());
            }
            pb.inc();
        }

        let end_time = start_time.elapsed();
        pb.finish_println(&format!("Rendered in {} seconds\n", end_time.as_secs()));
        let average_scanline_time: u128 = average(&scanline_durations);
        println!("Average scanline render time: {average_scanline_time} ms\n");

        print!("\x1B[0m");

        save::save_u32_vector_to_png("render.png", self.image_width as u32, self.image_height as u32, &data).unwrap();

        show_image(&mut data, self.image_width as usize, self.image_height as usize);
    }

    fn ray_colour(&self, r: &Ray, depth_limit: u64, world: &mut dyn Hittable) -> Colour {
        if depth_limit <= 0 { return Colour::new() }

        let mut record = HitRecord::new();

        if world.hit(r, Interval::from(0.001, f64::INFINITY), &mut record) {
           let mut scattered = Ray::new();
           let mut attenuation = Colour::new();
           let mut material = record.material.clone();

           if material.scatter(r, &mut record, &mut attenuation, &mut scattered) { 
               return attenuation * self.ray_colour(&scattered, depth_limit - 1, world);
           } 

           return Colour::new();
        }
        let unit_dir = r.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        return (1.0-a) * Colour::from(1.0, 1.0, 1.0) + a * Colour::from(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let pixel_center = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin: Vector3;

        if self.defocus_angle <= 0.0 {
            ray_origin = self.center;
        } else {
            ray_origin = self.defocus_disk_sample();
        }

        let ray_dir = pixel_sample - ray_origin;

        return Ray::from(ray_origin, ray_dir); 
    }

    fn pixel_sample_square(&self) -> Vector3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();

        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let p = random_in_unit_disk();
        return self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
    }
}
