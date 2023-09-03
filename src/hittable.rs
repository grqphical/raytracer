use crate::{vector3::{Vector3, dot_product}, ray::Ray, interval::Interval, material::{Material, Lambertian}, colour::Colour};

/// Represents a hit from a ray
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    /// Sets the records normal based on whether it is a front face or back face
    ///
    /// ## Arguments
    ///
    /// - `ray` The ray which was cast
    /// - `outward_normal` The normal of the face that was hit
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = dot_product(ray.direction, outward_normal) < 0.0;
        if self.front_face  {          
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
    
    /// Constructs an empty hit record
    pub fn new() -> Self {
        Self {
            point: Vector3::new(),
            normal: Vector3::new(),
            t: 0.0,
            front_face: false,
            material: Box::new(Lambertian::new(Colour::new())),
        }
    }
}

pub trait Hittable {
    fn hit(&mut self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
