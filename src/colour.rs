use crate::{vector3::Vector3, interval::Interval};

pub type Colour = Vector3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    return linear_component.sqrt();
}

impl Colour {
    /// Writes the pixel data to a string
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
}
