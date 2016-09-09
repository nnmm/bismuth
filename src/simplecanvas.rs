extern crate piston_window;
extern crate image as im;

use self::im::ConvertBuffer;
use self::piston_window::*;
use std::vec::Vec;

pub struct SimpleCanvas {
    window : PistonWindow,
    width : u32,
    height : u32,
    imbuf : im::ImageBuffer<im::Rgba<u8>, Vec<u8>>,
    texture : G2dTexture<'static>,
    drawables : Vec<Box<Drawable>>
}

pub type RgbaImage = im::ImageBuffer<im::Rgba<u8>, Vec<u8>>;

impl SimpleCanvas {
    pub fn new(windowname : &str) -> SimpleCanvas {
        let (width, height) = (800, 800);
        let mut window: PistonWindow =
            WindowSettings::new(windowname, (width, height))
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .build()
            .unwrap();
        window.set_bench_mode(true);
        
        let buf = im::ImageBuffer::from_pixel(width, height, im::Rgba([0, 0, 0, 0]));

        let texture = Texture::from_image(
            &mut window.factory,
            &buf,
            &TextureSettings::new() ).unwrap();

        SimpleCanvas {
            window: window,
            width: width,
            height: height,
            imbuf: buf,
            texture: texture,
            drawables: Vec::new()
        }
    }

    pub fn add(&mut self, thing : Box<Drawable>) {
        self.drawables.push(thing);
    }

    fn update(&mut self, steps : u32) {
        // call draw() on each drawable, remove if not alive
        for ref mut dr in self.drawables.iter_mut() {
            for _ in 0..steps {
                dr.draw(&mut self.imbuf);
            }
        }
    }

    fn reset(&mut self, width : u32, height : u32) {
        // println!("Window resized to ({}, {}).", width, height);
        self.width = width;
        self.height = height;
        // create new texture
        self.imbuf = im::ImageBuffer::from_pixel(width, height, im::Rgba([0, 0, 0, 0]));
        self.texture = Texture::from_image(
            &mut self.window.factory,
            &self.imbuf.convert(),
            &TextureSettings::new() ).unwrap();
        // call reset() on each drawable
        for ref mut dr in self.drawables.iter_mut() {
            dr.reset(width, height);
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            if let Event::Render(rargs) = e {
                // resized?
                if self.width == rargs.width && self.height == rargs.height {
                    self.update(2);
                    self.texture.update(&mut self.window.encoder, &self.imbuf).unwrap();
                } else {
                    self.reset(rargs.width, rargs.height);
                }
                let tex = &self.texture;
                self.window.draw_2d(&e, |c, g| {
                    clear([1.0; 4], g);
                    image(tex, c.transform, g);
                });
            }
        }
    }
}


pub trait Drawable {
    fn reset(&mut self, u32, u32);
    fn draw(&mut self, &mut RgbaImage);
    fn alive(&self) -> bool;
}

pub fn hsv_to_rgb(h : f64, s : f64, v : f64) -> im::Rgba<u8> {
    let c = v*s;
    let hprime = h*6.0;
    let x = c*(1.0 - (hprime % 2.0 - 1.0).abs());
    let m = v - c;
    let (r1, b1, g1) = match hprime {
        hp if 0.0 <= hp && hp < 1.0 => (c, x, 0.0),
        hp if 1.0 <= hp && hp < 2.0 => (x, c, 0.0),
        hp if 2.0 <= hp && hp < 3.0 => (0.0, c, x),
        hp if 3.0 <= hp && hp < 4.0 => (0.0, x, c),
        hp if 4.0 <= hp && hp < 5.0 => (x, 0.0, c),
        hp if 5.0 <= hp && hp < 6.0 => (c, 0.0, x),
        _ => panic!("Invalid h' {}", hprime)
    };
    im::Rgba {
        data: [((r1 + m) * 255.0) as u8, ((b1 + m) * 255.0) as u8, ((g1 + m) * 255.0) as u8, 255u8]
    }
}