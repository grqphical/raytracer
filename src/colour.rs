use crate::{vector3::Vector3, interval::Interval};

pub type Colour = Vector3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    return linear_component.sqrt();
}

impl Colour {
    /// Writes the pixel data to a string in the PPM P3 format
    /// 
    /// ## Arguments
    /// - `str` Mutable reference to String that will recieve the pixel data
    /// - `samples` How many samples per pixel the raytracer should anti-alias
    pub fn write_colour(&self, str: &mut String, samples: i64) {
        let (mut r, mut g, mut b) = (self.x , self.y, self.z);

        let scale: f64 = 1.0 / samples as f64;
        r *= scale;
        b *= scale;
        g *= scale;

        r = linear_to_gamma(r);
        b = linear_to_gamma(b);
        g = linear_to_gamma(g);

        let intensity = Interval::from(0.000, 0.999);
        let output = format!("{} {} {}\n", (256.0 * intensity.clamp(r)) as i64, (256.0 * intensity.clamp(g)) as i64, (256.0 * intensity.clamp(b)) as i64);

        str.push_str(&output);
    }
    
    /// Writes the raw pixel data as a Vector of U32
    ///
    /// ## Arguments
    /// - `output` A reference to the vector where you want to output the values
    /// = `samples` How many samples you want in the anti_aliasing
    pub fn write_colour_pixels(&self, output: &mut Vec<u32>, samples: i64) {
        let (mut r, mut g, mut b) = (self.x , self.y, self.z);

        let scale: f64 = 1.0 / samples as f64;
        r *= scale;
        b *= scale;
        g *= scale;

        r = linear_to_gamma(r);
        b = linear_to_gamma(b);
        g = linear_to_gamma(g);

        let intensity = Interval::from(0.000, 0.999);
        let rgba_value = ((256.0 * intensity.clamp(r)) as u32) << 16 | ((256.0 * intensity.clamp(g)) as u32) << 8 | (256.0 * intensity.clamp(b)) as u32;
        output.push(rgba_value);
    }
    
    /// Creates a colour from an RGB value
    ///
    /// ## Arguments
    /// - `r` Red component
    /// - `g` Green component
    /// - `b` Blue component
    pub fn from_rgb(r: u8, g: u8, b:u8) -> Self {
        let red = r as f64 / 256.0;
        let green = g as f64 / 256.0;
        let blue = b as f64 / 256.0;

        Self {
            x: red,
            y: green,
            z: blue
        }
    }
}
