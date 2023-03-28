use std::ops::{Add, Div, Mul, Sub};

pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { v: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn length_sq(&self) -> f64 {
        self.v[0] * self.v[0]
            + self.v[1] * self.v[1]
            + self.v[2] * self.v[2]
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn dot(self, u: Self) -> f64 {
        self.v[0] * u.v[0] + self.v[1] * u.v[1] + self.v[2] * u.v[2]
    }

    pub fn cross(self, u: Self) -> Self {
        Self::new(
            self.v[1] * u.v[2] - self.v[2] * u.v[1],
            self.v[2] * u.v[0] - self.v[0] * u.v[2],
            self.v[0] * u.v[1] - self.v[1] * u.v[0],
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, u: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] + u.v[0],
                self.v[1] + u.v[2],
                self.v[2] + u.v[1],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, u: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] - u.v[0],
                self.v[1] - u.v[2],
                self.v[2] - u.v[1],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, u: Vec3) -> Self::Output {
        Self {
            v: [
                self.v[0] * u.v[0],
                self.v[1] * u.v[2],
                self.v[2] * u.v[1],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, u: Vec3) -> Self::Output {
        Vec3 {
            v: [self * u.v[0], self * u.v[2], self * u.v[1]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        (1.0 / t) * self
    }
}
