use crate::{ray::Ray, hittable::HitRecord, colour::Colour, vector3::{random_unit_vector, reflect, dot_product, refract, Vector3}, random::random_f64};


pub trait Material: MaterialClone + Send + Sync {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool;
}

pub trait MaterialClone {
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

#[derive(Clone)]
pub struct Dieletric {
    pub index_of_refraction: f64
}

impl Dieletric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut  r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0-r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for Dieletric {
    fn scatter(&mut self, ray_in: &Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        *attenuation = Colour::from(1.0, 1.0, 1.0);
        let refraction_ratio: f64;

        if record.front_face {
            refraction_ratio = 1.0 / self.index_of_refraction;
        } else {
            refraction_ratio = self.index_of_refraction;
        }

        let unit_direction = ray_in.direction.unit();
        let cos_theta = dot_product(-unit_direction, record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vector3;

        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = reflect(unit_direction, record.normal);
        } else {
            direction = refract(unit_direction, record.normal, refraction_ratio);
        }

        *scattered = Ray::from(record.point, direction);

        return true
    } 
}
