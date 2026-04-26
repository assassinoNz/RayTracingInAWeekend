use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::point::Point3;
use crate::ray::Ray3;
use crate::util::{rand_f64, rand_f64_clamped};

#[repr(C)]
#[repr(align(16))]
pub struct Vec3(f64, f64, f64);

impl core::clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl core::ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Vec3 {
        self.0 = self.0.neg();
        self.1 = self.1.neg();
        self.2 = self.2.neg();
        self
    }
}

impl core::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.clone().neg()
    }
}

impl core::ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl core::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Vec3) -> Vec3 {
        self.sub_assign(&rhs);
        self
    }
}

impl core::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, mut rhs: Vec3) -> Vec3 {
        rhs.0 = self.0 - rhs.0;
        rhs.1 = self.1 - rhs.1;
        rhs.2 = self.2 - rhs.2;
        rhs
    }
}

impl core::ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: &Vec3) -> Vec3 {
        self.sub_assign(rhs);
        self
    }
}

impl core::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl core::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl core::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Vec3 {
        self.add_assign(&rhs);
        self
    }
}

impl core::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, mut rhs: Vec3) -> Vec3 {
        rhs.add_assign(self);
        rhs
    }
}

impl core::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: &Vec3) -> Vec3 {
        self.add_assign(rhs);
        self
    }
}

impl core::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl core::ops::MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl core::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: Vec3) -> Vec3 {
        self.mul_assign(&rhs);
        self
    }
}

impl core::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl core::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl core::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: f64) -> Vec3 {
        self.mul_assign(rhs);
        self
    }
}

impl core::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl core::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

impl core::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: f64) -> Vec3 {
        self.div_assign(rhs);
        self
    }
}

impl core::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn new_rand() -> Vec3 {
        Vec3(rand_f64(), rand_f64(), rand_f64())
    }

    pub fn new_rand_clamped(min: f64, max: f64) -> Vec3 {
        Vec3(
            rand_f64_clamped(min, max),
            rand_f64_clamped(min, max),
            rand_f64_clamped(min, max),
        )
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn len_sq(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.0.abs() < EPSILON && self.1.abs() < EPSILON && self.2.abs() < EPSILON
    }

    pub fn into_unit(self) -> UnitVec3 {
        UnitVec3(&self / self.len())
    }

    pub fn into_ray(self, origin: Point3) -> Ray3 {
        Ray3::new(origin, self)
    }
}

pub type Color3 = Vec3;

impl Color3 {
    pub fn new_black() -> Color3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn new_white() -> Color3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn new_red() -> Color3 {
        Vec3(1.0, 0.0, 0.0)
    }

    pub fn new_green() -> Color3 {
        Vec3(0.0, 1.0, 0.0)
    }

    pub fn new_blue() -> Color3 {
        Vec3(0.0, 0.0, 1.0)
    }
}

impl Color3 {
    pub fn r(&self) -> f64 {
        self.x()
    }

    pub fn g(&self) -> f64 {
        self.y()
    }

    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn into_gamma_corrected(mut self) -> Color3 {
        self.0 = if self.0 > 0.0 { self.0.sqrt() } else { 0.0 };

        self.1 = if self.1 > 0.0 { self.1.sqrt() } else { 0.0 };

        self.2 = if self.2 > 0.0 { self.2.sqrt() } else { 0.0 };

        self
    }
}

pub struct UnitVec3(Vec3);

impl core::ops::Deref for UnitVec3 {
    type Target = Vec3;

    fn deref(&self) -> &Vec3 {
        &self.0
    }
}

impl core::ops::Neg for UnitVec3 {
    type Output = UnitVec3;

    fn neg(mut self) -> UnitVec3 {
        self.0.0 = self.0.0.neg();
        self.0.1 = self.0.1.neg();
        self.0.2 = self.0.2.neg();
        self
    }
}

impl core::ops::Neg for &UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> UnitVec3 {
        unsafe { UnitVec3::new_unchecked(self.as_vec().neg()) }
    }
}

impl UnitVec3 {
    pub unsafe fn new_unchecked(vec: Vec3) -> Self {
        Self(vec)
    }

    pub fn new_rand() -> UnitVec3 {
        loop {
            let rand_clamped_vec = Vec3::new_rand_clamped(-1.0, 1.0);
            let len_sq = rand_clamped_vec.len_sq();
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return UnitVec3(rand_clamped_vec / len_sq.sqrt());
            }
        }
    }
}

impl UnitVec3 {
    pub fn as_vec(&self) -> &Vec3 {
        &self.0
    }

    pub fn into_ray(self, origin: Point3) -> Ray3 {
        Ray3::new(origin, self.0)
    }

    pub fn reflect(&self, vec: &Vec3) -> Vec3 {
        vec - (&self.0 * 2.0 * self.dot(vec))
    }

    pub fn refract(&self, vec: &Vec3, ni_div_nt: f64) -> Vec3 {
        let cos_theta = (-self.dot(vec)).min(1.0);
        let r_out_perp = (&self.0 + (vec * cos_theta)) * ni_div_nt;
        let r_out_parallel = -vec * (1.0 - r_out_perp.len_sq()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}
