use crate::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        return Lambertian { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }
}

pub struct Metal {
    albedo : Color
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
        let reflected  = Vec3::reflect(r_in.direction, rec.normal);
        let scattered  = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }

}
