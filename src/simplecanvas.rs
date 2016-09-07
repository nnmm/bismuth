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
        
        let buf = im::ImageBuffer::from_pixel(width, height, im::Rgba([0, 0, 0, 255]));

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

    fn update(&mut self) {
        // call draw() on each drawable, remove if not alive
        for ref mut dr in self.drawables.iter_mut() {
            dr.draw(&mut self.imbuf);
        }
    }

    fn reset(&mut self, width : u32, height : u32) {
        // call new() on each drawable
        println!("Window resized to ({}, {}).", width, height);
        self.width = width;
        self.height = height;
        self.imbuf = im::ImageBuffer::from_pixel(width, height, im::Rgba([0, 0, 0, 255]));
        self.texture = Texture::from_image(
            &mut self.window.factory,
            &self.imbuf.convert(),
            &TextureSettings::new() ).unwrap();
        for ref mut dr in self.drawables.iter_mut() {
            dr.reset(width, height);
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            if let Event::Render(rargs) = e {
                // resized?
                if self.width == rargs.width && self.height == rargs.height {
                    self.update();
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