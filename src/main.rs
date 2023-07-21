use vector::Vec3;

use crate::vector::Colour;
use crate::vector::Ray;
mod vector;

fn ray_colour(r:Ray) -> Colour {
    let mut t = hit_sphere(vector::Point::_new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0
    {
        let n:Vec3 = vector::Vec3::unit_vector(r._at(t) - vector::Vec3::_new(0.0, 0.0, -1.0));
        return Colour::_new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction:Vec3 = vector::Vec3::unit_vector(r.dir);
    t = 0.5*(unit_direction.y + 1.0);   
    return Colour::_new(1.0, 1.0, 1.0)*(1.0-t) + Colour::_new(0.5, 0.7, 1.0)*t;
}

fn hit_sphere(center:vector::Point, radius:f64, r:&Ray) -> f64
{
    let oc = r.origin - center;
    let a = vector::Vec3::_dot(r.dir, r.dir);
    let b = 2.0 * vector::Vec3::_dot(oc, r.dir);
    let c = vector::Vec3::_dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0*a);
    }

}

fn main() {
    //Image
    const ASPECT_RATIO:f64 = 16.0/9.0;
    const IMAGE_WIDTH:i32 = 400;
    const IMAGE_HEIGHT:i32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as i32;

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
            let pixel_colour = ray_colour(r);
            vector::Colour::write_color(pixel_colour);
        }
        j-=1;
    }
}
