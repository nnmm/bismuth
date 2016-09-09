extern crate bismuth;
extern crate image as im;

use bismuth::inkdrop::*;
use bismuth::simplecanvas::*;
// use bismuth::hsvtest::*;


fn main() {
    let mut can = SimpleCanvas::new("Test");
    let item = InkDrop::new(800, 800);
    // let item = HsvTest;
    can.add(Box::new(item));
    can.run();
}

