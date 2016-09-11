extern crate piston_window;
extern crate image as im;
extern crate rand;

use std::collections::HashSet;
use std::vec::Vec;
use super::simplecanvas::*;

pub struct InkDrop {
    boundary: HashSet<(u32, u32)>,
    filled: HashSet<(u32, u32)>,
    colorize: Box<Fn(f64) -> Color>,
    cycles: u32,
    t: f64
}


impl InkDrop {
    pub fn new(start: HashSet<(u32, u32)>, colorize: Box<Fn(f64) -> Color>, cycles: u32) -> InkDrop {
        let boundary = start.clone();
        InkDrop {
            boundary: start,
            filled: HashSet::new(),
            colorize: colorize,
            cycles: cycles,
            t: 1.0
        }
    }

    fn new_neighbors(x: u32, y: u32, w: u32, h: u32) -> Vec<(u32, u32)> {
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
    fn draw(&mut self, canvas: &mut RgbaImage) {
        // println!("Boundary before: {}", self.boundary.len());

        let col = (self.colorize)(self.t);
        // let col = hsv_to_rgb(hue, 1.0, 1.0 - value);
        // let col = hsv_to_rgb(hue, value, 1.0);
        // randomly fill some of the boundary pixels
        let (w, h) = (canvas.width(), canvas.height());
        for &(x, y) in &self.boundary.clone() {
            if rand::random() {
                self.filled.insert((x, y));
                canvas.put_pixel(x, y, col);
                self.boundary.extend(InkDrop::new_neighbors(x, y, w, h));
            }
        }
        // remove those from boundary
        self.boundary = self.boundary.difference(&self.filled).cloned().collect();

        self.t += 1.0/self.t;
    }

    fn alive(&self) -> bool {
        //let cycles = (self.t + 1.0/self.t)/self.value_speed;
        self.boundary.len() > 0// && (cycles as u32) < self.cycles
    }
}


