use rand::{thread_rng, Rng};

pub fn update(image: &mut Image) -> bool {
    let mut rng = thread_rng();
    let x0: i32 = rng.gen_range(0, 500);
    let x1: i32 = rng.gen_range(0, 500);
    let y0: i32 = rng.gen_range(0, 500);
    let y1: i32 = rng.gen_range(0, 500);
    let r: u8 = rng.gen_range(0, 255);
    let g: u8 = rng.gen_range(0, 255);
    let b: u8 = rng.gen_range(0, 255);
    image.draw_line((x0, y0), (x1, y1), (r, g, b));
    true
}

pub struct Image {
    data: Vec<u8>,
    w: usize,
    _h: usize,
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Image {
            data: vec![0u8; 3 * w * h],
            w,
            _h: h,
        }
    }

    pub fn draw_line(&mut self, from: (i32, i32), to: (i32, i32), color: (u8, u8, u8)) {
        let (mut x0, mut y0) = from;
        let (mut x1, mut y1) = to;

        let steep = (x0 - x1).abs() < (y0 - y1).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror = (dy as f32 / dx as f32).abs();
        let mut error = 0.0;
        let mut y = y0;

        for x in x0..=x1 {
            if steep {
                self.set((y as usize, x as usize), color);
            } else {
                self.set((x as usize, y as usize), color);
            }

            error += derror;
            if error > 0.5 {
                y += if y1 > y0 { 1 } else { -1 };
                error -= 1.0;
            }
        }
    }

    pub fn set(&mut self, pixel: (usize, usize), color: (u8, u8, u8)) {
        let index = (pixel.0 + self.w * pixel.1) * 3;
        self.data[index] = color.0;
        self.data[index + 1] = color.1;
        self.data[index + 2] = color.2;
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }
}
