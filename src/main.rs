mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

use std::io::{self, Write};
use std::time::Instant;

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discrim = half_b * half_b - a * c;
    if discrim < 0.0 {
        -1.0
    } else {
        (-half_b - discrim.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_dir = ray.direction().normalized();
    t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.3, 0.5, 0.8);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let start = Instant::now();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    write!(&mut stdout, "P3\n{} {}\n255\n", image_width, image_height)?;

    for y in (0..image_height).rev() {
        write!(
            &mut stderr,
            "\rGenerating image {:.2}%..",
            (image_height - y) as f64 / image_height as f64 * 100.0
        )?;
        stderr.flush()?;
        for x in 0..image_width {
            let x_offset = x as f64 / image_width as f64;
            let y_offset = y as f64 / image_height as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + x_offset * horizontal + y_offset * vertical - origin,
            );
            let pixel_color = ray_color(ray);
            writeln!(&mut stdout, "{pixel_color}")?;
        }
    }
    writeln!(&mut stderr, "\nGenerated image in {:.2?}", start.elapsed())?;

    Ok(())
}
