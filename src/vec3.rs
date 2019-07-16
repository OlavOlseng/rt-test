use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, Neg };

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3{
        Vec3 {x, y, z}
    }

    pub fn zeroes() -> Vec3 {
        Vec3 { 
            x: 0.0, 
            y: 0.0, 
            z: 0.0
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.0, 
            y: 1.0, 
            z: 1.0
        }
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn length(&self) -> f32 {
        (self * self).sum().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        (self * self).sum()
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let k: f32 = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }
}

fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    (v1 * v2).sum()
}

fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z*v2.y,
        y: -v1.x * v2.z + v1.z*v2.x,
        z: v1.x * v2.y - v1.y*v2.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z
        }
    }
}

impl AddAssign for Vec3 {
    
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x; 
        self.y += rhs.y; 
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z
        }
    }
}

impl SubAssign for Vec3 {

    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs:Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl MulAssign for Vec3 {

    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3{
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.y
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3{
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn add() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } + Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: 3.0,
                y: 1.0,
                z: 4.0
            }
        );
    }

    #[test]
    fn add_assign() {
        let mut x = Vec3::new(0.0, 0.0, 0.0);
        let y = Vec3::new(1.0, 2.0, 3.0);
        x += y;
        assert_eq!(
            x,
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            super::cross(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 2.0
                },
                &Vec3 {
                    x: 2.0,
                    y: 1.0,
                    z: 2.0
                }),
            Vec3 {
                x: -2.0,
                y: 2.0,
                z: 1.0
            }
        )
    }

    #[test]
    fn dot() {
        assert_eq!(
            super::dot(
                &Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 2.0
                },
                &Vec3 {
                    x: 2.0,
                    y: 1.0,
                    z: 2.0
                }),
            6.0
        );
    }

    #[test]
    fn length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.length(), 3.0);
        assert_eq!(u.length(), 1.0);
    }

    #[test]
    fn squared_length() {
        let v = Vec3 {
            x: -2.0,
            y: -2.0,
            z: -1.0,
        };
        let u = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        assert_eq!(v.squared_length(), 9.0);
        assert_eq!(u.squared_length(), 1.0);
    }

    #[test]
    fn mul() {
        assert_eq!(
            3.0 * Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Vec3 {
                x: 3.0,
                y: 6.0,
                z: 9.0
            }
        );
    }

    #[test]
    fn hadamard() {
        let lhs = Vec3::new(1.0, 1.0, 1.0);
        let rhs = Vec3::new(2.0, 3.0, 4.0);
        println!("{:?}", lhs * rhs);
        assert_eq!(lhs * rhs, Vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Vec3 {
                x: 1.0,
                y: -2.0,
                z: 3.0
            },
            Vec3 {
                x: -1.0,
                y: 2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: 2.0
            } - Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0
            },
            Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0
            }
        );
    }
}