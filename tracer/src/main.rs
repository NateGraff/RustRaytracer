#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

mod data;
mod cast;

use data::*;
use cast::*;

fn main() {
    let v = Viewport {
        eye: Point{x: 0.0, y: 0.0, z: -10.0},
        eye_width: 10.0,
        eye_height: 10.0,
        img_width: 400,
        img_height: 400,
    };

    let s = Scene {
        spheres: vec![Sphere{
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 2.0,
            c: Color{r: 0.0, g: 0.0, b: 0.0},
            f: Finish {
                ambient: 0.0,
                diffuse: 0.0,
                specular: 0.0,
                roughness: 0.0,
            }
        }],
        light_source: Light{
            p: Point{x: 100.0, y: 100.0, z: -100.0},
            c: Color{r: 0.0, g: 0.0, b: 0.0},
        },
        light_color: Color{r: 0.0, g: 0.0, b: 0.0},
    };

    let bitmap = cast_viewport(&v, &s);
    save_bmp(&bitmap, &v);
}

fn save_bmp(bitmap: &Vec<Vec<Color>>, v: &Viewport) {
    let mut img = Image::new(v.img_width, v.img_height);

    for (x, y) in img.coordinates() {
        let xp = x as usize;
        let yp = y as usize;

        let r = f64::floor(bitmap[xp][yp].r * 255.0) as u8;
        let g = f64::floor(bitmap[xp][yp].g * 255.0) as u8;
        let b = f64::floor(bitmap[xp][yp].b * 255.0) as u8;

        img.set_pixel(x, y, px!(r, g, b));
    }

    let _ = img.save("img.bmp");
}
