mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};

use std::io::{self, Write};
use std::time::Instant;

fn ray_color(ray: Ray) -> Color {
    let unit_dir = ray.direction().normalized();
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
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
        writeln!(&mut stderr, "Line {}/{}", y, image_height)?;
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
    writeln!(&mut stderr, "Generated image in {:?}", start.elapsed())?;

    Ok(())
}
