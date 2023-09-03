use crate::{Vector3, ray::Ray, vector3::dot_product, hittable::{Hittable, HitRecord}, interval::Interval};

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64
}


impl Hittable for Sphere {
    /// Checks if a ray intersects with a sphere using the formula *-b + √b * b - 4ac / 2a* 
    fn hit(&mut self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = dot_product(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 { return false; }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        return true;
    }
}
