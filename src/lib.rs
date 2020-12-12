#![feature(test)]

extern crate test;

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

// A linear congruential generator, using gcc's constants
struct LCG {
    m: u64,
    a: u64,
    c: u64,
    state: u64,
}

impl LCG {
    fn new(seed: u64) -> LCG {
        LCG {
            m: 0x80000000,
            a: 1103515245,
            c: 12345,
            state: seed,
        }
    }

    fn next_int(&mut self) -> u64 {
        self.state = (self.a * self.state + self.c) % self.m;
        self.state
    }

    fn next_float(&mut self) -> f32 {
        self.next_int() as f32 / (self.m as f32 - 1.0)
    }
}

fn lerp(a: f32, b: f32, x: f32) -> f32 {
    a + x * (b - a)
}

fn grad(h: u8, x: f32, y: f32, z: f32) -> f32 {
    match h & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => 0.0,
    }
}

pub fn perlin3d(x: f32, y: f32, z: f32) -> f32 {
    // Get the unit cube that the point lies in
    let cx = x as usize & 255;
    let cy = y as usize & 255;
    let cz = z as usize & 255;
    // Get the position of the point within the cube
    let xf = x.fract();
    let yf = y.fract();
    let zf = z.fract();

    let xfade = fade(xf);
    let yfade = fade(yf);
    let zfade = fade(zf);

    let aaa = P[P[P[cx] as usize + cy] as usize + cz];
    let aba = P[P[P[cx] as usize + cy + 1] as usize + cz];
    let aab = P[P[P[cx] as usize + cy] as usize + cz + 1];
    let abb = P[P[P[cx] as usize + cy + 1] as usize + cz + 1];
    let baa = P[P[P[cx + 1] as usize + cy] as usize + cz];
    let bba = P[P[P[cx + 1] as usize + cy + 1] as usize + cz];
    let bab = P[P[P[cx + 1] as usize + cy] as usize + cz + 1];
    let bbb = P[P[P[cx + 1] as usize + cy + 1] as usize + cz + 1];

    let x1 = lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1.0, yf, zf), xfade);
    let x2 = lerp(
        grad(aba, xf, yf - 1.0, zf),
        grad(bba, xf - 1.0, yf - 1.0, zf),
        xfade,
    );
    let y1 = lerp(x1, x2, yfade);

    let x1 = lerp(
        grad(aab, xf, yf, zf - 1.0),
        grad(bab, xf - 1.0, yf, zf - 1.0),
        xfade,
    );
    let x2 = lerp(
        grad(abb, xf, yf - 1.0, zf - 1.0),
        grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
        xfade,
    );
    let y2 = lerp(x1, x2, yfade);

    (lerp(y1, y2, zfade) + 1.0) / 2.0
}

pub fn octave_perlin(x: f32, y: f32, z: f32, octaves: u32, persistence: f32) -> f32 {
    let mut total = 0.0;
    let mut freq = 1.0;
    let mut amp = 1.0;
    let mut max_val = 0.0;

    for _ in 0..octaves {
        total += perlin3d(x * freq, y * freq, z * freq) * amp;
        max_val += amp;
        amp *= persistence;
        freq *= 2.0;
    }

    total / max_val
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[wasm_bindgen]
pub struct PerlinGen {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
    gradients: [u8; 512],
}

impl PerlinGen {
    fn shuffle_gradients(gradients: &mut [u8], seed: u64) {
        let mut rng = LCG::new(seed);
        // The array is repeated twice to avoid overflow - we just shuffle the
        // First half and copy it over to the second
        let mut n = 256;
        while n != 0 {
            let i = (rng.next_float() * n as f32).floor() as usize;
            n -= 1;
            gradients.swap(i, n);
        }
        gradients.copy_within(..256, 256);
    }
}

#[wasm_bindgen]
impl PerlinGen {
    pub fn new(width: u32, height: u32, seed: u32) -> PerlinGen {
        // Backing array of RGBA values
        let pixels = vec![0; (4 * width * height) as usize];

        // Precalculate the gradients
        let mut gradients: [u8; 512] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67,
            68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
            90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
            109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
            126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
            143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
            160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
            177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
            194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210,
            211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
            228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
            245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
            54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
            76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
            98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115,
            116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
            133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
            150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
            167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183,
            184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
            201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217,
            218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234,
            235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251,
            252, 253, 254, 255,
        ];

        PerlinGen::shuffle_gradients(&mut gradients, seed as u64);

        // Generate
        PerlinGen {
            width,
            height,
            pixels,
            gradients,
        }
    }

    pub fn fill_self(&mut self, octaves: u32, persistence: f32) {
        let mut pixels = self.pixels.chunks_mut(4);
        for i in 0..self.width {
            for j in 0..self.height {
                // Perlin noise is 0 at whole numbers - we need to modify the inputs
                // I haven't been able to find a good answer for the 'right' way to do this
                let x = (i as f32 / self.width as f32) * 10.0;
                let y = (j as f32 / self.height as f32) * 10.0;
                let result = octave_perlin(x, y, 0.0, octaves, persistence);
                let val = (255.0 * result) as u8;

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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_perlin_fill() {
        let mut img = PerlinGen::new(1000, 1000, 42);
        img.fill_self(2, 1.0);
    }

    #[bench]
    fn bench_600x600_perlin_fill(b: &mut Bencher) {
        let mut img = PerlinGen::new(1000, 1000, 42);
        b.iter(|| {
            img.fill_self(2, 1.0);
        });
    }
}
