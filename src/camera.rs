use std::sync::{mpsc, Mutex, Arc};
use std::io::Write;
use std::thread::{self, JoinHandle};
use std::time::Instant;
use crate::hittable_list::HittableList;
use crate::save;
use crate::vector3::{cross_product, random_in_unit_disk};
use crate::viewer::show_image;
use crate::{hittable::{Hittable, HitRecord}, colour::Colour, interval::Interval, ray::Ray, vector3::Vector3, random::random_f64};

/// Represents a scanline being transfered between threads
/// id is which row of the image the scanline is from
#[derive(Clone)]
struct ScanlineResult {
    id: usize,
    scanline: Vec<u32>,
}

/// Represents a camera in the raytracer
#[derive(Copy, Clone)]
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
    pub fn render(&mut self, world: HittableList) {
        let start_time = Instant::now();
        self.init();

        // Represents final pixel data
        // I used a 2D Vector because I wanted to ensure there wouldn't be any race conditions and each thread will know where to
        // output it's data
        let mut data: Vec<Vec<u32>> = vec![Vec::new(); self.image_height as usize];

        // List of thread handles that we can loop over and join
        let mut handles: Vec<JoinHandle<_>> = vec![];

        println!("\nStarting Render at {}x{} pixels with {} samples", self.image_width, self.image_height, self.samples_per_pixel);
        // Create a channel to send pixel data between threads
        let (pixel_tx, pixel_rx) = mpsc::channel::<ScanlineResult>();

        for j in 0..self.image_height {
            // Clone variables in order to be used in different threads
            let camera_clone = self.clone();
            let mut world_clone = world.clone();
            let pixel_transmitter = Arc::new(Mutex::new(pixel_tx.clone()));
            let mut colour_data: Vec<u32> = vec![];

            let handle = thread::spawn(move || {
                colour_data.clear();
                for i in 0..camera_clone.image_width { 
                    let mut pixel_colour = Colour::new();
                    for _ in 0..camera_clone.samples_per_pixel {
                        let r = camera_clone.get_ray(i, j);
                        pixel_colour += camera_clone.ray_colour(&r, camera_clone.depth_limit, &mut world_clone);
                    }
                    
                    // Write the pixel data to the temporary colour data buffer
                    pixel_colour.write_colour_pixels(&mut colour_data, camera_clone.samples_per_pixel);             
               }
                // Send scanline back to main thread
                pixel_transmitter.lock().unwrap().send(ScanlineResult { id: j as usize, scanline: colour_data }).unwrap();
            });

            handles.push(handle);
        }

        let mut current_complete_count = 1;
        for handle in handles {
            handle.join().unwrap();

            // Recieve pixel data
            let result = pixel_rx.recv().unwrap();
            print!("\rScanlines Completed: {}/{}", current_complete_count, self.image_height);
            std::io::stdout().flush().unwrap();
            current_complete_count += 1;

            // Append it to the 2D array used to represent the image
            for colour in result.scanline {
                data[result.id].push(colour);
            }
        }

        let end_time = start_time.elapsed();
        let mut final_data: Vec<u32> = vec![];

        // Convert 2D array into 1D array
        for mut row in data {
            final_data.append(&mut row);
        }

        println!("\nRendered in {} seconds", end_time.as_secs());
        print!("\x1B[0m");

        save::save_u32_vector_to_png("render.png", self.image_width as u32, self.image_height as u32, &final_data).unwrap();
        println!("Saved as 'render.png'");

        show_image(&mut final_data, self.image_width as usize, self.image_height as usize);
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
