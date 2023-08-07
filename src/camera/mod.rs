use crate::vector::Vec3;
use crate::vector::{ Point, Ray };

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point::_new(0.0, 0.0, 0.0);
        let horizontal = Vec3::_new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::_new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::_new(0.0, 0.0, focal_length);

        Camera {
            origin, 
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {

        Ray {
            origin: self.origin,
            dir: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin,
        }
    }
}