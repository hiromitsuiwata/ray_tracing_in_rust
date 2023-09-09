use std::{f32::consts::PI, ops::*};

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.x()
    }

    pub fn g(&self) -> f32 {
        self.y()
    }

    pub fn b(&self) -> f32 {
        self.z()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f32 {
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

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.x() * other.x(),
                self.y() * other.y(),
                self.z() * other.z(),
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.x() * t, self.y() * t, self.z() * t],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.x() / t, self.y() / t, self.z() / t],
        }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn color(r: f32, g: f32, b: f32) -> Vec3 {
    Vec3 { e: [r, g, b] }
}

/// 原点
pub fn origin() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// ランダムな小数
pub fn random_f32(min: f32, max: f32) -> f32 {
    let scale = max - min;
    let mut rng = rand::thread_rng();
    scale * rng.gen::<f32>() + min
}

/// 単位球の中心から球面上を向くランダムなベクトル
pub fn random_unit_vector() -> Vec3 {
    // 緯度
    let a = random_f32(0.0, 2.0 * PI);
    // 高さ
    let z = random_f32(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// 原点からxy平面上に原点中心で配置された単位円の円周上の点を向くランダムなベクトル
pub fn random_in_unit_disk() -> Vec3 {
    let theta = random_f32(0.0, 2.0 * PI);
    Vec3::new(theta.cos(), theta.sin(), 0.0)
}

pub fn random_color() -> Vec3 {
    Vec3::new(
        random_f32(0.0, 1.0),
        random_f32(0.0, 1.0),
        random_f32(0.0, 1.0),
    )
}

pub fn random_color_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_f32(min, max),
        random_f32(min, max),
        random_f32(min, max),
    )
}

/// 金属マテリアルの反射
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

/// 誘電体マテリアルの屈折
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_parallel = (uv + n * cos_theta) * etai_over_etat;
    let r_out_perp = -n * (1.0 - r_out_parallel.length_squared());
    r_out_parallel + r_out_perp
}
