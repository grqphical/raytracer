use image::ImageBuffer;
use image::Rgb;

pub fn save_u32_vector_to_png(filename: &str, width: u32, height: u32, data: &Vec<u32>) -> Result<(), image::ImageError> {
    // Create an ImageBuffer with RGBA format
    let mut img = ImageBuffer::new(width, height);

    // Iterate over the data and set the pixel values
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Convert the u32 value to RGBA format
        let rgba = Rgb([
            ((data[(y * width + x) as usize] >> 16) & 0xFF) as u8,
            ((data[(y * width + x) as usize] >> 8) & 0xFF) as u8,
            (data[(y * width + x) as usize] & 0xFF) as u8,
        ]);

        *pixel = rgba;
    }

    // Save the ImageBuffer to a PNG file
    img.save(filename)?;

    Ok(())
} 
