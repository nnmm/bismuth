use super::simplecanvas::*;

pub struct HsvTest;

impl Drawable for HsvTest {
    fn draw(&mut self, canvas : &mut RgbaImage) {
        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                let h = x as f64/canvas.width() as f64;
                let v = 1.0 - y as f64/canvas.height() as f64;
                // println!("(h, s, v) = {:?}", (h, 1.0, v));
                canvas.put_pixel(x, y, hsv_to_rgb(h, 1.0, v));
            }
        }
    }

    fn alive(&self) -> bool {
        true
    }
}