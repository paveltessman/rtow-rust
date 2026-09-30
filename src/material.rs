use crate::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

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
    albedo : Color,
    fuzz : f64,
}

impl Metal {
    pub fn new(albedo: Color, mut fuzz : f64) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0
        }
        return Metal { albedo, fuzz };
    }
}

impl Material for Metal {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {

        let mut reflected  = Vec3::reflect(r_in.direction, rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        let scattered  = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if dot(scattered.direction, rec.normal) <= 0.0 {
            return None;
        }
        return Some((attenuation, scattered));
    }
}

pub struct Dielectric {
    refraction_index : f64,
}

impl Dielectric {
    pub fn new(refraction_index : f64) -> Dielectric {
        return Dielectric { refraction_index };
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face { 1.0/self.refraction_index } else { self.refraction_index};
        let unit_direction = r_in.direction.unit_vector();
        let refracted = Vec3::refract(unit_direction, rec.normal, ri);
        let scattered = Ray::new(rec.p, refracted);

        return Some((attenuation, scattered));
    }
}
