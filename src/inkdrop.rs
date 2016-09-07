extern crate piston_window;
extern crate image as im;
extern crate rand;

use std::collections::HashSet;
use std::vec::Vec;
use super::simplecanvas::*;

pub struct InkDrop {
    boundary : HashSet<(u32, u32)>,
    filled : HashSet<(u32, u32)>,
    width : u32,
    height : u32,
    t : f64
}

impl InkDrop {
    pub fn new(width : u32, height : u32) -> InkDrop {
        InkDrop {
            boundary: [(width/2, height/2)].iter().cloned().collect(),
            filled: HashSet::new(),
            width: width,
            height: height,
            t: 1.0
        }
    }

    fn new_neighbors(x : u32, y : u32, w : u32, h : u32) -> Vec<(u32, u32)> {
        let mut v = Vec::new();
        if x > 0 {
            v.push((x-1, y));
        }
        if x < w-1 {
            v.push((x+1, y));
        }
        if y > 0 {
            v.push((x, y-1));
        }
        if y < h-1 {
            v.push((x, y+1));
        }
        v
    }
}

impl Drawable for InkDrop {
    fn reset(&mut self, width : u32, height : u32) {
        *self = InkDrop::new(width, height);
    }

    fn draw(&mut self, canvas : &mut RgbaImage) {
        println!("Boundary before: {}", self.boundary.len());
        let l = (50.0*self.t) as u8 % 255;
        // randomly fill some of the boundary pixels
        let (w, h) = (self.width, self.height);
        for &(x, y) in &self.boundary.clone() {
            if rand::random() {
                self.filled.insert((x, y));
                canvas.put_pixel(x, y, im::Rgba([l, l/2, 0, 255]));
                self.boundary.extend(InkDrop::new_neighbors(x, y, w, h));
            }
        }
        // remove those from boundary
        self.boundary = self.boundary.difference(&self.filled).cloned().collect();

        self.t += 1.0/self.t;
    }

    fn alive(&self) -> bool {
        self.boundary.len() > 0
    }
}


