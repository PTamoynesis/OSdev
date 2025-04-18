use crate::devices::cga as cga;
use crate::devices::cga::CGA;
// shortcut for cga
use crate::devices::cga_print; // used to import code needed by println! 
use crate::devices::key as key; // shortcut for key
use crate::devices::keyboard as keyboard;
use crate::devices::keyboard::KEYBOARD;
// shortcut for keyboard


pub fn run() {
    let mut keyboard = keyboard::KEYBOARD.lock();
    let mut cga = CGA.lock();
    cga.clear();
    keyboard.set_repeat_rate(10,2);

    loop {
        let mut key = keyboard.key_hit();

        if key.valid(){
            if key.get_ascii() == 0{
                cga.print_byte('-' as u8);
            } else if key.get_ascii() == 1 {
                cga.print_byte(key.get_ascii());
            } else {
                println!("Scancode: {}", key.get_scancode());
            }
        }
    }
    // keyboard.key_hit();
    /* Hier muss Code einfgeügt werden */

    // 'key_hit' aufrufen und Zeichen ausgeben
}
