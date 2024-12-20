use image::{ImageBuffer, Rgba};
use std::path::Path;

fn read_qoi_image(_filename: &Path) -> ImageBuffer<Rgba<u8>, &[u8]> {
    todo!()
}

fn main() {
    let image = read_qoi_image(Path::new("image.qoi"));
    image.save("image.png").unwrap();
}
