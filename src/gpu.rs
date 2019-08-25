pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(PartialEq, Debug)]
pub enum Collision {
    Collision,
    NoCollision,
}

pub struct Gpu {
    fb: PixelBuffer<bool>,
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            fb: PixelBuffer::new(),
        }
    }

    pub fn write_sprite(&mut self, x: usize, y: usize, sprite_lines: &[u8]) -> Collision {
        let mut collision = Collision::NoCollision;
        for (line, sprite) in sprite_lines.iter().enumerate() {
            let mut fb_pixels = [false; 8];
            self.fb.read_pos(x, y + line, &mut fb_pixels);
            for col in 0..8 {
                let sprite_pixel = (sprite & (0x80 >> col)) != 0;
                fb_pixels[col] ^= sprite_pixel;
                // collision if any pixel transition 1 -> 0
                // collision = (have & new) > 0
                if sprite_pixel == true && fb_pixels[col] == false {
                    collision = Collision::Collision;
                }
            }
            self.fb.write_pos(x, y + line, &fb_pixels);
        }
        collision
    }

    pub fn clear(&mut self) {
        self.fb.write(&[false; WIDTH * HEIGHT]);
    }
}

impl AsRef<[bool]> for Gpu {
    fn as_ref(&self) -> &[bool] {
        self.fb.as_ref()
    }
}

struct PixelBuffer<T: Default + Copy> {
    buf: [T; WIDTH * HEIGHT],
}

impl<T: Default + Copy> PixelBuffer<T> {
    fn new() -> PixelBuffer<T> {
        PixelBuffer {
            buf: [T::default(); WIDTH * HEIGHT],
        }
    }

    fn write(&mut self, pixels: &[T]) {
        self.write_pos(0, 0, pixels);
    }

    fn write_pos(&mut self, x: usize, y: usize, pixels: &[T]) {
        assert!(x < WIDTH && y < HEIGHT);

        let mut index = y * WIDTH + x;
        for &pixel in pixels {
            if index == WIDTH * HEIGHT {
                index = 0;
            }
            self.buf[index] = pixel;
            index += 1;
        }
    }

    fn read(&mut self, buf: &mut [T]) {
        self.read_pos(0, 0, buf);
    }

    fn read_pos(&mut self, x: usize, y: usize, buf: &mut [T]) {
        assert!(x < WIDTH && y < HEIGHT);

        let mut index = y * WIDTH + x;
        for val in buf {
            if index == WIDTH * HEIGHT {
                index = 0;
            }
            *val = self.buf[index];
            index += 1;
        }
    }
}

impl<T: Default + Copy> AsRef<[T]> for PixelBuffer<T> {
    fn as_ref(&self) -> &[T] {
        &self.buf
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pb_write() {
        let pixels: &[u32] = &[1, 2, 3, 4];
        let mut pb = PixelBuffer::<u32>::new();

        pb.write(pixels);
        assert_eq!(pb.buf[0..4], [1, 2, 3, 4]);

        pb.write_pos(0, 1, pixels);
        assert_eq!(pb.buf[WIDTH..WIDTH + 4], [1, 2, 3, 4]);

        pb.write_pos(6, 4, pixels);
        assert_eq!(pb.buf[4 * WIDTH + 6..4 * WIDTH + 6 + 4], [1, 2, 3, 4]);
    }

    #[test]
    fn pb_write_wrapping() {
        let pixels: &[u32] = &[4, 3, 2, 1];
        let mut pb = PixelBuffer::new();

        pb.write_pos(WIDTH - 1, HEIGHT - 1, pixels);
        assert_eq!(pb.buf[WIDTH * HEIGHT - 1], 4);
        assert_eq!(pb.buf[0..3], [3, 2, 1]);
    }

    #[test]
    fn pb_read() {
        let pixels: &[u32] = &[1, 2, 3, 4];
        let mut pb = PixelBuffer::new();
        pb.write(pixels);
        pb.write_pos(7, 7, pixels);

        {
            let mut buf = [0u32; 4];
            pb.read(&mut buf);
            assert_eq!(buf, [1, 2, 3, 4]);
        }
        {
            let mut buf = [0u32; 4];
            pb.read_pos(7, 7, &mut buf);
            assert_eq!(buf, [1, 2, 3, 4]);
        }
    }

    #[test]
    fn gpu_write_sprite() {
        let mut gpu = Gpu::new();
        let sprite: &[u8] = &[0b11110000, 0b00001111];
        // helper
        let fb_to_byte = |gpu: &Gpu, start| {
            gpu.as_ref()
                .iter()
                .skip(start)
                .take(8)
                .fold(0u8, |s, &v| (s << 1) | v as u8)
        };

        // write some sprite
        assert_eq!(gpu.write_sprite(0, 0, &sprite), Collision::NoCollision);
        assert_eq!(fb_to_byte(&gpu, 0), 0b11110000);
        assert_eq!(fb_to_byte(&gpu, WIDTH), 0b00001111);

        // write same sprite -> collision
        assert_eq!(gpu.write_sprite(0, 0, &sprite), Collision::Collision);
        assert_eq!(fb_to_byte(&gpu, 0), 0b00000000);
        assert_eq!(fb_to_byte(&gpu, WIDTH), 0b00000000);
    }
}
