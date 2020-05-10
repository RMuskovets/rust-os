#[allow(dead_code)]

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Color {
    pub fn lighter(self: &mut Color) -> Color {
        match self {
            Color::Black     => Color::DarkGray,
            Color::Blue      => Color::LightBlue,
            Color::Green     => Color::LightGreen,
            Color::Cyan      => Color::LightCyan,
            Color::Red       => Color::LightRed,
            Color::Magenta   => Color::Pink,
            Color::Brown     => Color::Yellow,
            Color::DarkGray  => Color::LightGray,
            Color::LightGray => Color::White,
            _                => *self
        }
    }

    pub fn darker(self: &mut Color) -> Color {
        match self {
            Color::DarkGray   => Color::Black,
            Color::LightGray  => Color::DarkGray,
            Color::LightBlue  => Color::Blue,
            Color::LightGreen => Color::Green,
            Color::LightCyan  => Color::Cyan,
            Color::LightRed   => Color::Red,
            Color::Pink       => Color::Magenta,
            Color::Yellow     => Color::Brown,
            Color::White      => Color::LightGray,
            _                 => *self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(fg: Color, bg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Char {
    ascii: u8,
    color: ColorCode
}

const WIDTH : usize = 80;
const HEIGHT: usize = 25;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Char; WIDTH]; HEIGHT]
}

pub struct Writer {
    pub   col: usize,
    pub color: ColorCode,
    pub   buf: &'static mut Buffer
}

impl Writer {
    pub fn putc(&mut self, b: u8) {
        match b {
            b'\n' => self.nl(),
            b => {
                if self.col >= WIDTH {
                    self.nl();
                }
                let row = HEIGHT - 1;
                let col = self.col;
                let color = self.color;
                self.buf.chars[row][col] = Char {
                    ascii: b,
                    color
                };
            }
        }
    }

    pub fn nl(&mut self) {
        for row in 1..HEIGHT {
            for col in 1..WIDTH {
                let c = self.buf.chars[row][col];
                self.buf.chars[row-1][col] = c;
            }
        }
        self.clear_row(HEIGHT - 1);
        self.col = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            ascii: b' ',
            color: self.color
        };
        for col in 0..WIDTH {
            self.buf.chars[row][col] = blank;
        }
    }

    pub fn puts(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                0x20..0x7e | b'\n' => self.putc(b),
                _ => self.putc(0xfe)
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}