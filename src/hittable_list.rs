use crate::{hittable::{Hittable, HitRecord}, ray::Ray, interval::Interval};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn with_object(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) { self.objects.clear() }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
   /// Checks if a ray intersects with a sphere using the formula *-b + √b * b - 4ac / 2a* 
    fn hit(&mut self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
       let mut temp_record = HitRecord::new();
       let mut hit_anything = false;
       let mut closest_so_far = ray_t.max;

       for object in &mut self.objects {
            if object.hit(ray, Interval::from(ray_t.min, closest_so_far), &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                rec.point = temp_record.point;
                rec.normal = temp_record.normal;
                rec.t = temp_record.t;
                rec.front_face = temp_record.front_face;
            }
       }

       return hit_anything;
    } 
}
