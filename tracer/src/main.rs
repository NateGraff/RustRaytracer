mod data;
mod scene;
mod cast;

use data::*;
use scene::*;
use cast::*;

fn main() {
    let v = Viewport {
        eye: Point{x: 0.0, y: 0.0, z: -10.0},
        eye_width: 10.0,
        eye_height: 10.0,
        img_width: 20,
        img_height: 20,
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
            p: Point{x: 0.0, y: 0.0, z: 0.0},
            c: Color{r: 0.0, g: 0.0, b: 0.0},
        },
        light_color: Color{r: 0.0, g: 0.0, b: 0.0},
    };

    print_bitmap(&cast_viewport(&v, &s));
}
