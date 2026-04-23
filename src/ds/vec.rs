use core::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::ds::point::Point3;
use crate::ds::ray::Ray3;

#[repr(C)]
#[repr(align(16))]
pub struct Vec3(f64, f64, f64);

impl core::clone::Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3(self.0.clone(), self.1.clone(), self.2.clone())
    }
}

impl core::default::Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
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
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl core::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Self) -> Vec3 {
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

    pub fn into_unit(self) -> UnitVec3 {
        UnitVec3(&self / self.len())
    }

    pub fn into_ray(self, origin: Point3) -> Ray3 {
        Ray3::new(origin, self)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }
}

pub type Color3 = Vec3;

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
}

impl Color3 {
    pub fn black() -> Color3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color3 {
        Vec3(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color3 {
        Vec3(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color3 {
        Vec3(0.0, 0.0, 1.0)
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
    pub fn as_vec(&self) -> &Vec3 {
        &self.0
    }
}

impl UnitVec3 {
    pub unsafe fn new_unchecked(vec: Vec3) -> Self {
        Self(vec)
    }
}
