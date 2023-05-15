pub enum Format {
    Grayscale,
    RGB,
    RGBA,
}

pub struct TGAColor {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
    bytespp: i32,
}

impl TGAColor {
    pub fn new(b: u8, g: u8, r: u8, a: u8, bpp: Format) -> Self {
        let bytespp = enum2num(bpp);
        TGAColor {
            b: b,
            g: g,
            r: r,
            a: a,
            bytespp: bytespp,
        }
    }
}

pub struct TGAImage {
    width: i32,
    height: i32,
    bytespp: i32,
    data : Vec<u8>
}

impl TGAImage {
    pub fn new(w: i32, h: i32, bpp: Format) -> Self {
        let bytespp = enum2num(bpp);
        let nbytes: usize = (w * h * bytespp).try_into().unwrap_or(0);
        let data = vec![0; nbytes];
        TGAImage {
            width: w,
            height: h,
            bytespp: bytespp,
            data: data,
        }
    }

}

fn enum2num(bpp: Format) -> i32 {
    match bpp {
        Format::Grayscale => 1,
        Format::RGB => 3,
        Format::RGBA => 4,
    }
}
