use std::io::{self, Write};
use std::time::Instant;

const IMG_HEIGHT: usize = 256;
const IMG_WIDTH: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let start = Instant::now();

    write!(&mut stdout, "P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT)?;

    for y in (0..IMG_HEIGHT).rev() {
        writeln!(&mut stderr, "Line {}/{}", y, IMG_HEIGHT)?;
        for x in 0..IMG_WIDTH {
            writeln!(&mut stdout, "{} {} {}", x, y, 63)?;
        }
    }
    writeln!(&mut stderr, "Took {:?}", start.elapsed())?;

    Ok(())
}
