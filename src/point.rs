use std::ops::{AddAssign, SubAssign};

use crate::vec::Vec3;

pub struct Point3(Vec3);

impl core::clone::Clone for Point3 {
    fn clone(&self) -> Point3 {
        Point3(self.0.clone())
    }
}

impl core::convert::From<Vec3> for Point3 {
    fn from(vec: Vec3) -> Point3 {
        Point3(vec)
    }
}

impl core::convert::From<&Vec3> for Point3 {
    fn from(e: &Vec3) -> Point3 {
        Point3(e.clone())
    }
}

impl core::ops::AddAssign<&Vec3> for Point3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.0.add_assign(rhs);
    }
}

impl core::ops::Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(mut self, rhs: Vec3) -> Point3 {
        self.add_assign(&rhs);
        self
    }
}

impl core::ops::Add<Vec3> for &Point3 {
    type Output = Point3;

    fn add(self, mut rhs: Vec3) -> Point3 {
        rhs.add_assign(&self.0);
        Point3(rhs)
    }
}

impl core::ops::Add<&Vec3> for Point3 {
    type Output = Point3;

    fn add(mut self, rhs: &Vec3) -> Point3 {
        self.add_assign(rhs);
        self
    }
}

impl core::ops::Add<&Vec3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: &Vec3) -> Point3 {
        Point3(&self.0 + rhs)
    }
}

impl core::ops::Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(mut self, rhs: Point3) -> Vec3 {
        self.0.sub_assign(&rhs.0);
        self.0
    }
}

impl core::ops::Sub<Point3> for &Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Point3) -> Vec3 {
        &self.0 - &rhs.0
    }
}

impl core::ops::Sub<&Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: &Point3) -> Vec3 {
        &self.0 - &rhs.0
    }
}

impl core::ops::Sub<&Point3> for &Point3 {
    type Output = Vec3;

    fn sub(self, rhs: &Point3) -> Vec3 {
        &self.0 - &rhs.0
    }
}

impl Point3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3(Vec3::new(x, y, z))
    }

    pub fn new_origin() -> Point3 {
        Point3(Vec3::new(0.0, 0.0, 0.0))
    }
}

impl Point3 {
    pub fn x(&self) -> f64 {
        self.0.x()
    }

    pub fn y(&self) -> f64 {
        self.0.y()
    }

    pub fn z(&self) -> f64 {
        self.0.z()
    }
}
