use crate::vector3::Vector3;

pub type Colour = Vector3;

impl Colour {
    /// Writes the pixel data to a string
    /// 
    /// ## Arguments
    /// - `str` Mutable reference to String that will recieve the pixel data
    pub fn write_colour(&self, str: &mut String) {
        let data = format!("{} {} {}\n", (self.x * 255.999) as i64, (self.y * 255.999) as i64, (self.z * 255.999) as i64);
        str.push_str(&data)
    }
}