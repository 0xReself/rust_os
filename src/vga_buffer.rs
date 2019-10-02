#[allow(dead_code)]
/*
 * To enable copy semantics make it printable and comparable
 */ 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/*
 * Set the type of the enum to u8 (u4 would be enough but rust doesn't have it)
 */
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

/*
 * To enable copy semantics make it printable and comparable
 */ 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/*
 * To ensure that the Data Layout is always a u8
 */
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /*
     * Constructor for a ColorCode
     * @param{foreground} the foreground Color form the Enum
     * @param{background} the background Color from the Enum
     * @return{ColorCode} returns the New Color code that is 1byte big
     * the first 4 bytes the froeground the other 4 bytes background
     */
    fn new(foreground: Color, background: Color) -> ColorCode {
        return ColorCode((background as u8) << 4 | (foreground as u8));
    }
}

/*
 * To enable copy semantics make it printable and comparable
 */ 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/*
 * To use the c struct layout so it is guarantees the correct field ordering
 */
#[repr(C)]
/*
 * A Char on the Screen with a Ascii Code and a ColorCode
 */
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/*
 * The Size of the normal VGA Display that we can draw into 
 */
const VGA_BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER_WIDTH: usize = 80;

/*
 * To be sure the compiler dont optimize the writes to the buffer
 */
use volatile::Volatile;

/*
 * To ensure that the Data Layout is always a u8
 */
#[repr(transparent)]
/*
 * A Buffer with an "2D" Array that is height*width and then always a ScreenChar
 */
struct Buffer {
    /*ScreenChar as Type, WIDTH 1st Demension, HEIGHT 2nd Demension*/
    chars: [[Volatile<ScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}

/*
 * The Writer will always write to the last line and goes to the next one when the old
 * one is full or a \n is there. Then a ColorCode for the current Color and a referenz 
 * to the VGA BUFFER that is 'static so it is running the whole program long
 */
pub struct Writer {
    colum_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /*
     * Write a single Char to the Screen and skips to the next line when a \n accured
     * @param{byte} the byte that needs to be written to the VGA-Buffer
     */
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte  => {
                if self.colum_position >= VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = VGA_BUFFER_HEIGHT - 1;
                let col = self.colum_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.colum_position += 1;
            },
        }
    }

    /*
     * Write a String to the VGA Buffer
     * @param{s} string that needs to be written to the buffer 
     */
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                /*printable ASCII byte or newline*/
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                /*not in range*/
                _                   => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {}
}

pub fn print_something() {
    let mut writer = Writer {
        colum_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut * (0xb8000 as *mut Buffer) },
    };

    let string: &'static str = "ello wörld!";

    writer.write_byte(b'H');
    writer.write_string(&string);
}