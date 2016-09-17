extern crate bismuth;
extern crate image as im;

// use std::collections::HashSet;
use bismuth::inkmap::*;
use bismuth::simplecanvas::*;
// use bismuth::hsvtest::*;


fn main() {
    let init_cb = &init;
    let frame_cb = &run;
    let mut can = SimpleCanvas::new("Bismuth", init_cb, frame_cb);

    can.run();
}

fn init(width: u32, height: u32) -> Vec<Box<Drawable>> {
    let cb = Box::new(|t| {
        let hue_speed = 100.0;
        let value_speed = 5.0;
        let hue = (t%hue_speed)/hue_speed;
        let value = (t%value_speed)/value_speed;
        hsv_to_rgb(1.0 - hue, value, 1.0)
    });
    let mut result : Vec<Box<Drawable>> = Vec::new();
    let start_set = [((width/2, height/2), true)].iter().cloned().collect();
    let item = InkMap::new(start_set, cb, 300);
    // let item = HsvTest;
    result.push(Box::new(item));

    // there has to be a better way ...
/*    let start_set1 : HashSet<(u32, u32)> = (0..width).map(|i| (i, 0)).collect();
    let start_set2 : HashSet<(u32, u32)> = (0..width).map(|i| (i, height-1)).collect();
    let start_set3 : HashSet<(u32, u32)> = start_set1.union(&start_set2).cloned().collect();
    let start_set4 : HashSet<(u32, u32)> = (0..height).map(|i| (0, i)).collect();
    let start_set5 : HashSet<(u32, u32)> = start_set3.union(&start_set4).cloned().collect();
    let start_set6 : HashSet<(u32, u32)> = (0..height).map(|i| (width-1, i)).collect();
    let start_set7 : HashSet<(u32, u32)> = start_set5.union(&start_set6).cloned().collect();
    let start_set8 : HashSet<(u32, u32)> = (0..height).map(|i| (0, i)).collect();
    let start_set9 : HashSet<(u32, u32)> = start_set7.union(&start_set8).cloned().collect();
    let item2 = InkDrop::new(start_set9, ..., 100);
    result.push(Box::new(item2));*/

    result
}

fn run(_: u32, _: u32, _: u32) -> Vec<Box<Drawable>> {
    let result : Vec<Box<Drawable>> = Vec::new();
    result
}

