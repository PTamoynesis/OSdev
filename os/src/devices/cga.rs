/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: This module provides functions for doing output on the CGA text ║
   ║         screen. It also supports a text cursor position stored in the   ║
   ║         hardware using ports.                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use spin::Mutex;
use crate::kernel::cpu as cpu;

/// Global CGA instance, used for screen output in the whole kernel.
/// Usage: let mut cga = cga::CGA.lock();
///        cga.print_byte(b'X');
pub static CGA: Mutex<CGA> = Mutex::new(CGA::new());

/// All 16 CGA colors.
#[repr(u8)] // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

pub const CGA_STD_ATTR: u8 = (Color::Black as u8) << 4 | (Color::Green as u8);

const CGA_BASE_ADDR: *mut u8 = 0xb8000 as *mut u8;
const CGA_ROWS: usize = 25;
const CGA_COLUMNS: usize = 80;

const CGA_INDEX_PORT: u16 = 0x3d4; // select register
const CGA_DATA_PORT: u16 = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8 = 14;  // cursor high byte
const CGA_LOW_BYTE_CMD: u8 = 15;   // cursor low byte

pub struct CGA {
    index_port: cpu::IoPort,
    data_port: cpu::IoPort
}

impl CGA {
    /// Create a new CGA instance.
    const fn new() -> CGA {
        CGA {
            index_port: cpu::IoPort::new(CGA_INDEX_PORT),
            data_port: cpu::IoPort::new(CGA_DATA_PORT)
        }
    }

    /// Clear CGA screen and set cursor position to (0, 0).
    pub fn clear(&mut self) {
        for x in 0..80 {
            for y in 0..25 {
                self.show(x,y,' ',CGA_STD_ATTR);
            }
        }
        self.setpos(0,0);
        /* Hier muss Code eingefuegt werden */
    }

    /// Display the `character` at the given position `x`,`y` with attribute `attrib`.
    pub fn show(&mut self, x: usize, y: usize, character: char, attrib: u8) {
        if x > CGA_COLUMNS || y > CGA_ROWS {
            return;
        }

        let pos = (y * CGA_COLUMNS + x) * 2;

        // Write character and attribute to the screen buffer.
        //
        // Unsafe because we are writing directly to memory using a pointer.
        // We ensure that the pointer is valid by using CGA_BASE_ADDR
        // and checking the bounds of x and y.
        unsafe {
            CGA_BASE_ADDR.offset(pos as isize).write(character as u8);
            CGA_BASE_ADDR.offset((pos + 1) as isize).write(attrib);
        }
    }

    /// Return cursor position `x`,`y`
    pub fn getpos(&mut self) -> (usize, usize) {
        let mut pos: u16 = 0;
        /* Hier muss Code eingefuegt werden */
        unsafe {
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            pos = (self.data_port.inb() as u16) << 8;
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            pos |= self.data_port.inb() as u16;
        }
        let x = (pos % CGA_COLUMNS as u16) as usize;
        let y = (pos / CGA_COLUMNS as u16) as usize;

        (x, y)
        // Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen
    }

    /// Set cursor position `x`,`y` 
    pub fn setpos(&mut self, x: usize, y: usize) {
        let mut offset = y * CGA_COLUMNS + x;
        let high_byte = (offset >> 8) as u8;
        let low_byte = (offset & 0xFF) as u8;
        unsafe {
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            self.data_port.outb(high_byte);
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            self.data_port.outb(low_byte);
        }
        /* Hier muss Code eingefuegt werden */
    }

    /// Print byte `b` at actual position cursor position `x`,`y`
    pub fn print_byte(&mut self, b: u8) {
        let (mut x, mut y) = self.getpos();

        if y == CGA_ROWS - 1 && x == CGA_COLUMNS - 1{
            self.scrollup();
            y -= 1;
        }

        self.show(x, y, b as char, CGA_STD_ATTR);

        x += 1;
        if x >= CGA_COLUMNS {
            x = 0;
            y += 1;
        }
        self.setpos(x, y);
        /* Hier muss Code eingefuegt werden */
    }

    /// Scroll text lines by one to the top.
    pub fn scrollup(&mut self) {
        let mut character = 0 as u8;
        let mut attrib = 0 as u8;
        let mut pos = 0 as usize;
        let mut destpos = 0 as usize;
        for n in 1..80 {
            for m in 0..25{
                pos = (n * CGA_COLUMNS + m) * 2;
                destpos = ((n-1)*CGA_COLUMNS + m) * 2;
                unsafe {
                    character = CGA_BASE_ADDR.offset(pos as isize).read();
                    attrib = CGA_BASE_ADDR.offset((pos + 1) as isize).read();

                    CGA_BASE_ADDR.offset(destpos as isize).write(character);
                    CGA_BASE_ADDR.offset((destpos + 1) as isize).write(attrib);
                }
            }
        }
        for x in 0..CGA_COLUMNS {
            self.show(x,CGA_ROWS-1,'\0', CGA_STD_ATTR);
        }
        self.setpos(0,CGA_ROWS-1);
    }
        /* Hier muss Code eingefuegt werden */

    /// Helper function returning an attribute byte for the given parameters `bg`, `fg`, and `blink`
    pub fn attribute(&mut self, bg: Color, fg: Color, blink: bool) -> u8 {
        /* Hier muss Code eingefuegt werden */
        let attrib = (fg as u8) << 4 | (bg as u8);
        //technically with this probably but qemu doesnt emulate blinking
            // | (blink as u8) << 7;
        attrib// Platzhalter, entfernen und durch sinnvollen Rueckgabewert ersetzen
    }
}