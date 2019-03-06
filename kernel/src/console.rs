use super::*;

pub struct Console {
    framebuffer: Framebuffer,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
}

impl Console {
    pub fn new(framebuffer: Framebuffer) -> Self {
        let width = (framebuffer.width() / 8) - 1;
        let height = (framebuffer.height() / 8) - 1;
        Console {
            framebuffer: framebuffer,
            row: 0,
            col: 0,
            height: height,
            width: width,
        }
    }

    fn scroll(&mut self) {
        self.framebuffer.shift_vertical(8);
    }

    pub fn write_char(&mut self, ch: char) {
        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            let y = self.row * 8;
            let x = self.col * 8;

            self.framebuffer.draw_char(ch, y, x);
            self.col += 1;
        }

        if self.col >= self.width {
            self.row += 1;
            self.col = 0;
        }

        if self.row >= self.height {
            self.scroll();
            self.row = self.height - 1;
        }
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.bytes() {
            self.write_char(ch as _);
        }
        Ok(())
    }
}

