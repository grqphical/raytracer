use crate::{ray::Ray, hittable::HitRecord, colour::Colour, vector3::{random_unit_vector, reflect, dot_product}};

pub trait Material: MaterialClone {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool;
}

trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T where T: 'static + Material + Clone {
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::from(record.point, scatter_direction);
        *attenuation = self.albedo; 
        return true;
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo:Colour, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let reflected = reflect(ray_in.direction.unit(), record.normal);

        *scattered = Ray::from(record.point, reflected + self.fuzz * random_unit_vector());
        *attenuation = self.albedo;
        return dot_product(scattered.direction, record.normal) > 0.0;
    }
}
