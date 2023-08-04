use crate::vector::Vec3;
use crate::vector::{ Point, Ray };

pub struct HitRecord {
    pub p:Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face:bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::_dot(r.dir, *outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else { *outward_normal * -1.0 };
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, red: &mut HitRecord) -> bool;
}



pub struct Sphere {
    cen: Point,
    r: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.cen;
        let a = r.dir._length_squared();
        let half_b = Vec3::_dot(oc, r.dir);
        let c = oc._length_squared() - self.r*self.r;
        let discriminant = half_b*half_b - a*c;
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / (a);

        if root < t_min || root > t_max {
            root = (-half_b - sqrtd) / (a);
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.t = root;
        rec.p = r._at(rec.t);
        let outward_normal = (rec.p - self.cen) / self.r;
        rec.set_face_normal(r, &outward_normal);
        
        return true; 
    }
}