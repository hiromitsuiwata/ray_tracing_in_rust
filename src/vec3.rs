use std::{f64::consts::PI, ops::*};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.x()
    }

    pub fn g(&self) -> f64 {
        self.y()
    }

    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.x() * t, self.y() * t, self.z() * t],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 {
            e: [self.x() / t, self.y() / t, self.z() / t],
        }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn color(r: f64, g: f64, b: f64) -> Vec3 {
    Vec3 { e: [r, g, b] }
}

pub fn origin() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

fn random_f64(min: f64, max: f64) -> f64 {
    let scale = max - min;
    let mut rng = rand::thread_rng();
    scale * rng.gen::<f64>() + min
}

fn random_vec(min: f64, max: f64) -> Vec3 {
    let scale = max - min;
    // 乱数
    let mut rng = rand::thread_rng();
    let r1 = scale * rng.gen::<f64>() + min;
    let r2 = scale * rng.gen::<f64>() + min;
    let r3 = scale * rng.gen::<f64>() + min;
    Vec3::new(r1, r2, r3)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let r = random_vec(-1.0, 1.0);
        if r.length_squared() >= 1.0 {
            continue;
        } else {
            return r;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    // 緯度
    let a = random_f64(0.0, 2.0 * PI);
    // 高さ
    let z = random_f64(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}
