use crate::vector::Vec3;
use crate::vector::{ Point, Ray };

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p:Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face:bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::_dot(r.dir, *outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}



pub struct Sphere {
    pub cen: Point,
    pub r: f64,
}

impl Sphere {
    pub fn new(point:Point, radius:f64) -> Sphere {
        return Sphere {
            cen: point,
            r: radius,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.cen;
        let a = ray.dir._length_squared();
        let half_b = Vec3::_dot(oc, ray.dir);
        let c = oc._length_squared() - self.r*self.r;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / (a);

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / (a);
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray._at(rec.t);
        let outward_normal = (rec.p - self.cen) / self.r;
        rec.set_face_normal(ray, &outward_normal);
        
        return true; 
    }
}