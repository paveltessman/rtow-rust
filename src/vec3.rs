use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::util::{random_f64, random_f64_range};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Vec3 {
    pub fn new(x : f64, y : f64, z : f64) -> Vec3 {
        Vec3 {x, y, z}
    }
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_f64(), random_f64(), random_f64());
    }

    pub fn random_range(min : f64, max : f64) -> Vec3 {
        let vec = Vec3::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
            );
        return vec;
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lensq = p.len_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal : Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        }
        return -on_unit_sphere;
    }

    pub fn reflect(v : Vec3, n : Vec3) -> Vec3 {
        return v - 2.0*dot(v, n) * n;
    }

    pub fn refract(uv : Vec3, n : Vec3, etai_over_etal : f64) -> Vec3 {
        let cos_theta = dot(-uv, n).min(1.0);
        let r_out_perp = etai_over_etal * (uv + cos_theta*n);
        let r_out_parallel = -(((1.0 - r_out_perp.len_squared()).abs()).sqrt())*n;
        return r_out_perp + r_out_parallel;

    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        const S : f64 = 1e-8;
        return self.x.abs() < S && self.y.abs() < S && self.z.abs() < S;
    }
}

pub fn dot(a : Vec3, b: Vec3) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self * other.x,
            self * other.y,
            self * other.z,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(
            self.x / other,
            self.y / other,
            self.z / other,
        )
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

pub type Point3=Vec3;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v+w, Vec3::new(5.0, 7.0, 9.0))
    }

    #[test]
    fn test_sub() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v-w, Vec3::new(-3.0, -3.0, -3.0))
    }

    #[test]
    fn test_f64mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(10.0*v, Vec3::new(10.0, 20.0, 30.0))
    }

    #[test]
    fn test_divf64() {
        let v = Vec3::new(6.0, 9.0, 12.0);
        assert_eq!(v/3.0, Vec3::new(2.0, 3.0, 4.0))
    }

    #[test]
    fn test_ops() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v+w, Vec3::new(5.0, 7.0, 9.0));

        // Reuse
        assert_eq!(10.0*v, Vec3::new(10.0, 20.0, 30.0))
    }
}
