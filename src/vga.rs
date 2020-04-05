//! VGA driver
extern crate volatile;
use self::volatile::Volatile;
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

/// Print out the message on the VGA console.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Print out the message on the VGA console.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

lazy_static! {
    /// Global VGA console writer.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// VGA consoler writer.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

/// VGA console color enum.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightSyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1; // bottom row
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{println, serial_print, serial_println};
    #[test_case]
    fn println() {
        serial_print!("vga::println... ");
        println!("here is the output from println");
        serial_println!("[ok]");
    }
    #[test_case]
    fn println_many() {
        serial_print!("vga::println_many... ");
        for _ in 0..200 {
            println!("let's println a lot");
        }
        serial_println!("[ok]");
    }
    #[test_case]
    fn println_output() {
        use super::*;
        use x86_64::instructions::interrupts;
        serial_print!("vga::println_output... ");
        let s = "Some test string that fits on a single line";
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writeln!(writer, "\n{}", s).expect("write failed");
            for (i, c) in s.chars().enumerate() {
                let got = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
                assert_eq!(char::from(got.ascii_character), c);
            }
        });
        serial_println!("[ok]");
    }
    #[test_case]
    fn println_multilines() {
        use super::*;
        use x86_64::instructions::interrupts;
        serial_print!("vga::println_multilines... ");
        let s = "Some test string that fits on a single line";
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writeln!(writer).expect("reset write failed");
            for i in 0..10 {
                writeln!(writer, "{} #{}", s, i).expect("write failed");
            }
            for j in 0..10 {
                for (i, c) in s.chars().enumerate() {
                    let got = writer.buffer.chars[BUFFER_HEIGHT - (2 + j)][i].read();
                    assert_eq!(char::from(got.ascii_character), c);
                }
                let got = writer.buffer.chars[BUFFER_HEIGHT - (2 + j)][s.len() + 2].read();
                assert_eq!(
                    char::from(got.ascii_character),
                    char::from((0x39 - j) as u8), // 0x39 is ASCII '9'
                );
            }
        });
        serial_println!("[ok]");
    }
}
