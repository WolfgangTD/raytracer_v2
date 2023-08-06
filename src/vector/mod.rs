use std::ops::{Sub, Add, Mul, Div, Neg};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs:Vec3) -> Vec3 {
        Vec3{ x:self.x + rhs.x, y:self.y + rhs.y, z:self.z + rhs.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs:Vec3) -> Vec3 {
        Vec3{ x:self.x - rhs.x, y:self.y - rhs.y, z:self.z - rhs.z}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs:f64) -> Vec3 {
        Vec3{ x:self.x * rhs, y:self.y * rhs, z:self.z * rhs}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs:f64) -> Vec3 {
        Vec3{ x:self.x * 1.0/rhs, y:self.y * 1.0/rhs, z:self.z * 1.0/rhs}
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{ x:self.x * -1.0, y: self.y * -1.0, z: self.z * -1.0}
    }
}


impl Vec3 {
    pub fn _new(x:f64, y:f64, z:f64) -> Vec3 {
        return Vec3 { x: (x), y: (y), z: (z) }
    }
    pub fn _length_squared(self) -> f64 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }
    pub fn _length(self) -> f64 {
        return self._length_squared().sqrt();
    }
    pub fn _dot(u: Vec3, v: Vec3) -> f64 {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }
    pub fn _cross(u: &Vec3, v: &Vec3) -> Vec3 {
        return Vec3 { 
            x: (u.y * v.z - u.z * v.y), 
            y: (u.z * v.x  - u.x * v.z), 
            z: (u.x * v.y - u.y * v.x) 
        };
    }
    pub fn unit_vector(v: Vec3) -> Vec3{
        return v / v._length();
    }
    pub fn write_color(pixel_colour:Colour) {
        println!("{} {} {}", (255.999 * pixel_colour.x) as i32, (255.999 * pixel_colour.y) as i32, (255.999 * pixel_colour.z) as i32);
    }
}

pub type Colour = Vec3;
pub type Point = Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin:Point,
    pub dir:Vec3
}

impl Ray {
    pub fn _at(self, t:f64) -> Point {
        let out:Point = self.origin + self.dir*t;
        return out;
    }
}