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

    pub fn set(&mut self, x: i32, y: i32, c: TGAColor) -> bool {
        if self.data.len() == 0 || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        for i in 0..self.bytespp {
            let pos = (i + (x + y * self.width) * self.bytespp) as usize;
            self.data[pos] = c.b;
            self.data[pos+1] = c.g;
            self.data[pos+2] = c.r;
            self.data[pos+3] = c.a;
        }
        return true;
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.data.len() == 0 {
            return false;
        }

        let bytes_per_line: usize = (self.width * self.bytespp).try_into().unwrap_or(0);
        if bytes_per_line == 0 {
            return false;
        }

        let mut line: Vec<u8> = vec![0; bytes_per_line];
        let half: usize = (self.height >> 1).try_into().unwrap_or(0);
        if half == 0 {
            return false; // image size is reasonably big
        }

        for i in 0..half {
            let height = self.height as usize;
            let l1 = i * bytes_per_line;
            let l2 = (height-1-i) * bytes_per_line;
            
            // line <- data+l1
            for j in 0..bytes_per_line {
                line[j] = self.data[j + l1];
                self.data[j + l1] = self.data[j + l2];
                self.data[j + l2] = line[j];
            }
        }
        return true;
    }

    /*
    pub fn write_tga_file(&self, filename: &str) -> bool {
        let developer_area_ref: [u8; 4] = [0, 0, 0, 0];
        let extension_area_ref: [u8; 4] = [0, 0, 0, 0];
        let footer: [u8; 17] = ['T','R','U','E','V','I','S','I','O','N','-','X','F','I','L','E','.'];
        let datatypecode = get_datatype_code(&self.bytespp);
        let header: TGAHeader = TGAHeader::new(0, 0, datatypecode, 0, 0, 0, 0, 0, 
            self.width, self.height, self.bytespp << 3, 0x20);

        // TODO: dump data
    }
    */
}

/*
struct TGAHeader {
    idlength: u8,
    colormaptype: u8,
    datatypecode: u8,
    colormaporigin: u16,
    colormaplength: u16,
    colormapdepth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bitsperpixel: u8,
    imagedescriptor: u8,
}

impl TGAHeader {
    pub fn new(idlength: u8, colormaptype: u8, datatypecode: u8, colormaporigin: u16, colormaplength: u16,
        colormapdepth: u8, x_origin: u16, y_origin: u16, width: u16, height: u16, bitsperpixel: u8, imagedescriptor: u8) {
        TGAHeader {
            idlength: idlength,
            colormaptype: colormaptype,
            datatypecode: datatypecode,
            colormaporigin: colormaporigin,
            colormaplength: colormaplength,
            colormapdepth: colormapdepth,
            x_origin: x_origin,
            y_origin: y_origin,
            width: width,
            height: height,
            bitsperpixel: bitsperpixel,
            imagedescriptor: imagedescriptor,
        }
    }
}
*/

/*
 * util
 */
fn enum2num(bpp: Format) -> i32 {
    match bpp {
        Format::Grayscale => 1,
        Format::RGB => 3,
        Format::RGBA => 4,
    }
}
