use crate::vec3::Vec3;
use std::fmt::Display;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {} {}",
            (self.x() * 255.999) as u8,
            (self.y() * 255.999) as u8,
            (self.z() * 255.999) as u8
        ))
    }
}
