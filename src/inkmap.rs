extern crate piston_window;
extern crate image as im;
extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use super::simplecanvas::*;

pub struct InkMap {
    boundary: HashMap<(u32, u32), bool>,
    colorize: Box<Fn(f64) -> Color>,
    cycles: u32,
    t: f64
}


impl InkMap {
    pub fn new(start: HashMap<(u32, u32), bool>, colorize: Box<Fn(f64) -> Color>, cycles: u32) -> InkMap {
        InkMap {
            boundary: start,
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

impl Drawable for InkMap {
    fn draw(&mut self, canvas: &mut RgbaImage) {
        // println!("Boundary before: {}", self.boundary.len());

        let col = (self.colorize)(self.t);
        // let col = hsv_to_rgb(hue, 1.0, 1.0 - value);
        // let col = hsv_to_rgb(hue, value, 1.0);
        // randomly fill some of the boundary pixels
        let (w, h) = (canvas.width(), canvas.height());
        for (&(x, y), &active) in &self.boundary.clone() {
            if active && rand::random() {
                // disable pixel
                self.boundary.insert((x, y), false);
                canvas.put_pixel(x, y, col);
                if x > 0 {
                    self.boundary.entry((x-1, y)).or_insert(true);
                }
                if x < w-1 {
                    self.boundary.entry((x+1, y)).or_insert(true);
                }
                if y > 0 {
                    self.boundary.entry((x, y-1)).or_insert(true);
                }
                if y < h-1 {
                    self.boundary.entry((x, y+1)).or_insert(true);
                }
                // self.boundary.extend(InkMap::new_neighbors(x, y, w, h));
            }
        }

        self.t += 1.0/self.t;
    }

    fn alive(&self) -> bool {
        //let cycles = (self.t + 1.0/self.t)/self.value_speed;
        self.boundary.len() > 0// && (cycles as u32) < self.cycles
    }
}


