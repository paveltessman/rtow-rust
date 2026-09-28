use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, dot};
use crate::ray::{Ray};

pub struct Sphere {
    pub center : Point3,
    pub radius : f64,
}

impl Sphere {
    pub fn new(center : Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.direction.len_squared();
        let h = dot(r.direction, oc);
        let c = oc.len_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let t = root;
        let p = r.at(root);
        let normal = (p - self.center) / self.radius;

        return Some(HitRecord { p, normal, t });
    }
}
