use std::f64::INFINITY;
use std::f64::consts::PI;
use std::rc::Rc;

use vector::Vec3;
use hittable::{ HitRecord, Sphere, Hittable };


use crate::hittables::HittableList;
use crate::vector::Colour;
use crate::vector::Point;
use crate::vector::Ray;
mod vector;
mod hittable;
mod hittables;

//Utility Functions
fn _degrees_to_radians(degrees:f64) -> f64 {
    degrees * PI / 180.0
}

fn ray_colour(r:&Ray, world: &dyn Hittable) -> Colour {
    let mut rec = HitRecord { p: (Point::_new(0.0, 0.0, 0.0)), normal: (Vec3::_new(0.0, 0.0, 0.0)), t: (0.0), front_face: (false) };
    if world.hit(r, 0.0, INFINITY, &mut rec)
    {
        return (rec.normal + Colour::_new(1.0,1.0,1.0)) * 0.5;
    }
    let unit_direction:Vec3 = vector::Vec3::unit_vector(r.dir);
    let t = 0.5*(unit_direction.y + 1.0);   
    return (Colour::_new(1.0, 1.0, 1.0)*(1.0-t)) + (Colour::_new(0.5, 0.7, 1.0)*t);
}

fn _hit_sphere(center:vector::Point, radius:f64, r:&Ray) -> f64
{
    let oc = r.origin - center;
    let a = r.dir._length_squared();
    let half_b = vector::Vec3::_dot(oc, r.dir);
    let c = oc._length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / (a);
    }
}

fn main() {
    //Image
    const ASPECT_RATIO:f64 = 16.0/9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as i32;

    //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::_new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::_new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vector::Point::_new(0.0, 0.0, 0.0);
    let horizontal = vector::Vec3::_new(viewport_width, 0.0, 0.0);
    let vertical = vector::Vec3::_new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vector::Vec3::_new(0.0, 0.0, focal_length);

    //Render`
    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = IMAGE_HEIGHT-1;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH-1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT-1) as f64;
            let r:Ray = Ray { origin: (origin), dir: (lower_left_corner + horizontal*u + vertical*v - origin) };
            let pixel_colour = ray_colour(&r, &world);
            vector::Colour::write_color(pixel_colour);
        }
        j-=1;
    }
    eprintln!("\nDone.\n");
}
