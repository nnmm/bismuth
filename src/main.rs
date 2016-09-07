extern crate bismuth;

use bismuth::inkdrop::*;
use bismuth::simplecanvas::*;


fn main() {
    let mut can = SimpleCanvas::new("Test");
    let klecks = InkDrop::new(800, 800);
    can.add(Box::new(klecks));
    can.run();
}
