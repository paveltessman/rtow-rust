use std::ops::{Add, AddAssign, Mul};
use crate::interval::Interval;

#[derive(Debug)]
pub struct Color {
    pub r : f64,
    pub g : f64,
    pub b : f64
}

impl Color {
    pub fn new(r : f64, g : f64, b : f64) -> Color {
        Color { r, g, b }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
        )
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self * other.r,
            self * other.g,
            self * other.b,
        )
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Color {
        return Color::new(value.x, value.y, value.z);
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other : Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}


use std::io::Write;

use crate::vec3::Vec3;

const INTENSITY : Interval = Interval::new(0.0, 0.999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

pub fn write_color(out : &mut impl Write, pixel_color : Color) {

    let r = linear_to_gamma(pixel_color.r);
    let g = linear_to_gamma(pixel_color.g);
    let b = linear_to_gamma(pixel_color.b);

    let rbyte = (256.0 * INTENSITY.clamp(r)) as usize;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as usize;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap();
}
