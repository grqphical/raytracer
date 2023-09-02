use std::fs;
use colour::Colour;
use pbr::ProgressBar;
use ray::Ray;
use vector3::Vector3;

mod vector3;
mod colour;
mod ray;

fn ray_colour(r: &Ray) -> Colour {
    let unit_direction = r.direction.unit();
    let a = 0.5 *(unit_direction.y + 1.0);
    return (1.0-a) * Colour::from(1.0, 1.0, 1.0) + a * Colour::from(0.5, 0.7, 1.0);
}

// Define constants related to the render output
const WIDTH: usize = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;

const FILE_NAME: &str = "output.ppm";


fn main() {
    // Calculate the image height based on the aspect ratio
    let mut image_height = (WIDTH as f64 / ASPECT_RATIO) as i64;
    if image_height < 1 {
        image_height = 1;
    }

    let viewport_width: f64 = VIEWPORT_HEIGHT * (WIDTH as f64 / image_height as f64);

    // Camera data
    let focal_length = 1.0;
    let camera_center = Vector3::new();

    // Calculate the vectors across the horizontal and down the vertical edges;
    let viewport_u = Vector3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::from(0.0, -VIEWPORT_HEIGHT as f64, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / WIDTH as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate position of upper-left pixel
    let viewport_upper_left = camera_center - Vector3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Represents the final PPM data
    let mut data = String::new();

    // Create a progressbar to keep track of the render
    let mut pb = ProgressBar::new(image_height as u64);
    pb.message("Scanlines remaining: ");

    // Push the header to the final data
    data.push_str(&format!("P3\n{WIDTH} {image_height}\n255\n"));

    for j in 0..image_height {
        for i in 0..WIDTH {
            let pixel_center = pixel00_location + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from(camera_center, ray_direction);
             
            let mut pixel_data = String::new();
            let pixel_colour = ray_colour(&r);

            pixel_colour.write_colour(&mut pixel_data);

            // Write the pixel data to the output string
            data.push_str(&pixel_data);
        }
        pb.inc();
    }
    pb.finish_print("Rendered");

    // Write the output string to the file
    fs::write(FILE_NAME, data).expect("Error writing to file");
}
