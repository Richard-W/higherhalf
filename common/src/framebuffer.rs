use r_efi::efi::protocols::graphics_output::BltPixel;
use font8x8::UnicodeFonts;
use font8x8::unicode::BasicFonts;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Framebuffer {
    buffer: *mut BltPixel,
    width: usize,
    height: usize,
    pitch: usize,
}

impl Framebuffer {
    pub unsafe fn new(buffer: *mut BltPixel, width: usize, height: usize, pitch: usize) -> Self {
        Framebuffer {
            buffer: buffer,
            width: width,
            height: height,
            pitch: pitch,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn fill(&mut self, red: u8, green: u8, blue: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = unsafe {
                    &mut *self.buffer.offset((y * self.pitch + x) as isize)
                };
                pixel.red = red;
                pixel.green = green;
                pixel.blue = blue;
            }
        }
    }

    pub fn shift_vertical(&mut self, rows: usize) {
        for y in 0..(self.height - rows) {
            for x in 0..self.width {
                unsafe {
                    self.buffer.offset((y * self.pitch + x) as isize).write(*self.buffer.offset(((y + rows) * self.pitch + x) as isize));
                }
            }
        }
        for y in (self.height - rows)..self.height {
            for x in 0..self.width {
                unsafe {
                    self.buffer.offset((y * self.pitch + x) as isize).write(BltPixel { blue: 0, green: 0, red: 0, reserved: 0 });
                }
            }
        }
    }

    /// Draw a character in the framebuffer
    ///
    /// (y, x) describe the pixel coordinate of the upper left corner
    /// of the character.
    pub fn draw_char(&mut self, character: char, y: usize, x: usize) {
        assert!(y + 8 < self.height);
        assert!(x + 8 < self.width);
        let glyph = match BasicFonts::new().get(character) {
            Some(x) => x,
            None => [0xff; 8],
        };

        for glyph_row in 0..8 {
            let glyph_row_data = glyph[glyph_row];
            for glyph_col in 0..8 {
                let pixel = unsafe {
                    &mut *self.buffer.offset(((y + glyph_row) * self.pitch + x + glyph_col) as isize)
                };
                if (glyph_row_data & (1 << glyph_col)) != 0 {
                    pixel.red = 0xff;
                    pixel.green = 0xff;
                    pixel.blue = 0xff;
                }
                else {
                    pixel.red = 0x00;
                    pixel.green = 0x00;
                    pixel.blue = 0x00;
                }
            }
        }
    }
}

