pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn len_sq(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn magnitude(&self) -> f64 {
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

    pub fn unit_vec(&self) -> Vec3 {
        self / self.magnitude()
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3(self.0, self.1, self.2)
    }
}

impl core::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl core::ops::Sub<Self> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl core::ops::Sub<Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        &self - &rhs
    }
}

impl core::ops::Sub<&Self> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Self) -> Vec3 {
        &self - rhs
    }
}

impl core::ops::Add<Self> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl core::ops::Add<Self> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        &self + &rhs
    }
}

impl core::ops::Add<&Self> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Self) -> Vec3 {
        &self + rhs
    }
}

impl core::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        self + &rhs
    }
}

impl core::ops::AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl core::ops::Mul<Self> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl core::ops::Mul<Self> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        &self * &rhs
    }
}

impl core::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl core::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        &self * rhs
    }
}

impl core::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl core::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl core::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        &self / rhs
    }
}

impl core::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

pub type Color3 = Vec3;

impl Color3 {
    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

pub type Point3 = Vec3;

impl Point3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}