use std::f64::INFINITY;
use std::f64::consts::PI;
use std::rc::Rc;
use rand::prelude::*;

use vector::Vec3;
use hittable::{ HitRecord, Sphere, Hittable };
use camera::Camera;

use crate::hittables::HittableList;
use crate::vector::Colour;
use crate::vector::Point;
use crate::vector::Ray;
mod vector;
mod hittable;
mod hittables;
mod camera;

//Utility Functions
fn _degrees_to_radians(degrees:f64) -> f64 {
    degrees * PI / 180.0
}

fn rand_double() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(0.0..1.0);
    return y;
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
    const SAMPLES_PER_PIXEL:i32 = 100;

    //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::_new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::_new(0.0, -100.5, -1.0), 100.0)));

    //Camera
    let cam = Camera::new();

    //Render`
    println!("P3\n {IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let mut j = IMAGE_HEIGHT-1;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::_new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand_double()) / ((IMAGE_WIDTH-1) as f64);
                let v = (j as f64 + rand_double()) / ((IMAGE_HEIGHT-1) as f64);
                let r = cam.get_ray(u, v);
                pixel_colour = pixel_colour + ray_colour(&r, &world);
            }
            Vec3::write_color(pixel_colour, SAMPLES_PER_PIXEL);
        }
        j-=1;
    }
    eprintln!("\nDone.\n");
}
