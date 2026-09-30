mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod util;
mod material;

use crate::hittable_list::HittableList;
use crate::vec3::{Point3};
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::Lambertian;
use crate::color::Color;

fn main() {

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Color::new(0.5, 0.5, 0.5))));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Color::new(0.5, 0.5, 0.5))));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
