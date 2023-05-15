mod tgaimage;
use tgaimage::{TGAColor, TGAImage, Format};

fn main() {
    let white: TGAColor = TGAColor::new(255, 255, 255, 255, Format::RGBA);
    let red: TGAColor = TGAColor::new(255, 0, 0, 255, Format::RGBA);

    let mut image = TGAImage::new(100, 100, Format::RGB);
    image.set(52, 51, red);
    image.flip_vertically();
    //image.write_tga_file("output.tga");
}
