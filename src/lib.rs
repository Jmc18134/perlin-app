use wasm_bindgen::prelude::*;

const P: [u8; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + x * (b - a)
}

fn grad2(h: u8, x: f32, y: f32) -> f32 {
    match h & 0x7 {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        _ => 0.0,
    }
}

#[wasm_bindgen]
pub fn perlin2d(x: f32, y: f32, octaves: u32, persistence: f32, xrepeat: u32, yrepeat: u32) -> f32 {
    let cx = x as u32 & 255;
    let cy = y as u32 & 255;
    let ox = x.fract();
    let oy = y.fract();

    let xfade = fade(ox);
    let yfade = fade(oy);

    let i1 = cx as usize;
    let i2 = cy as usize;
    let mut xinc = cx as u32 + 1;
    if xrepeat > 0 {
        xinc %= xrepeat;
    }

    let xinc = xinc as usize;

    let mut yinc = cy as u32 + 1;
    if yrepeat > 0 {
        yinc %= yrepeat;
    }

    let yinc = yinc as usize;

    let aa = P[P[i1] as usize + i2];
    let ab = P[P[i1] as usize + yinc];
    let ba = P[P[xinc] as usize + i2];
    let bb = P[P[xinc] as usize + yinc];

    let x1 = lerp(grad2(aa, ox, oy), grad2(ba, ox - 1.0, oy), xfade);
    let x2 = lerp(
        grad2(ab, ox, oy - 1.0),
        grad2(bb, ox - 1.0, oy - 1.0),
        xfade,
    );

    (lerp(x1, x2, yfade) + 1.0) / 2.0
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[wasm_bindgen]
pub struct PerlinGen {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl PerlinGen {
    pub fn new(width: u32, height: u32) -> PerlinGen {
        // Backing array of RGBA values
        let pixels = vec![0; (4 * width * height) as usize];
        PerlinGen {
            width,
            height,
            pixels,
        }
    }

    pub fn fill_self(&mut self, octaves: u32, persistence: f32, xrepeat: u32, yrepeat: u32) {
        let mut pixels = self.pixels.chunks_mut(4);
        for i in 0..self.width {
            for j in 0..self.height {
                // Get the perlin noise value at the coords of this pixel
                // Output is an f32 between 0 and 1, so map it to an RGB value
                let val = (255.0 * perlin2d(i as f32, j as f32, octaves, persistence, xrepeat, yrepeat)) as u8;

                let pixel = pixels.next().unwrap();
                pixel[0] = val;
                pixel[1] = val;
                pixel[2] = val;
                pixel[3] = 255;
            }
        }
    }

    pub fn get_pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}
