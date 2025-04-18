use crate::devices::cga;
use crate::devices::cga::{Color, CGA};
// shortcut for cga
use crate::devices::cga_print;
use crate::devices::kprint::{kprint, Writer};
// used to import code needed by println!

pub fn run () {
    let mut cga = CGA.lock();
    cga.clear();
    // print!("kaka poo poo ");
    // for n in 0..120 {
    //     cga.print_byte('A' as u8);
    // }

    // only for testing purposes
    // let attribute = cga.attribute(Color::Pink,Color::Red,true);
    // cga.show(10,10,'f',attribute);
    // cga.setpos(20,20);
    // kprintln!("{:?}",cga.getpos());
    // cga.setpos(5,9);
    // kprintln!("{:?}",cga.getpos());
    // cga.scrollup();

    /* Hier muss Code eingefuegt werden */
}
