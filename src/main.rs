use std::fs;
use colour::Colour;
use pbr::ProgressBar;

mod vector3;
mod colour;
mod ray;

// Define constants related to the render output
const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const FILE_NAME: &str = "output.ppm";


fn main() {
    // Represents the final PPM data
    let mut data = String::new();

    // Create a progressbar to keep track of the render
    let mut pb = ProgressBar::new(HEIGHT as u64);
    pb.message("Scanlines remaining: ");

    // Push the header to the final data
    data.push_str(&format!("P3\n{WIDTH} {HEIGHT}\n255\n"));

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            // Generate a simple rainbow gradient using the UV coordinate
            let pixel_colour = Colour::from(i as f64 / (WIDTH as f64 - 1.0), j as f64 / (WIDTH as f64 - 1.0), 0.0);
            let mut pixel_data = String::new();

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
