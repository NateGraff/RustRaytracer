use data::*;

pub struct Viewport {
    pub eye: Point,
    pub eye_width: f64,
    pub eye_height: f64,
    pub img_width: u32,
    pub img_height: u32,
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub light_source: Light,
    pub light_color: Color,
}

pub fn cast_vector(r: &Ray, s: &Scene) -> Color {
    for sphere in &s.spheres {
        match sphere.intersection(r) {
            Some(p) =>
                {
                    let normal = sphere.normal_at(&p);
                    let to_light = p.vector_to(&s.light_source.p).normalize();
                    let brightness = (normal.dot(&to_light) + 1.0)/ 2.0;

                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    match sphere.intersection(&Ray{p: p, dir: to_light}) {
                        Some(_) =>
                            {
                                r = 0.0;
                                g = 0.0;
                                b = 0.0;
                            },
                        None =>
                            {
                                r = sphere.c.r * brightness;
                                g = sphere.c.g * brightness;
                                b = sphere.c.b * brightness;
                            }
                    }

                    return Color{r: r, g: g, b: b};
                }
            None =>
                continue
        }
    }
    Color{r: 1.0, g: 1.0, b: 1.0}
}

pub fn cast_viewport(view: &Viewport, s: &Scene) -> Vec<Vec<Color>> {
    let mut bitmap = Vec::new();
    for y in 0..view.img_height {
        let mut line = Vec::new();
        for x in 0..view.img_width {
            let screen_point = Point {
                x: f64::from(x) * view.eye_width / f64::from(view.img_width) - (view.eye_width / 2.0),
                y: f64::from(y) * view.eye_height / f64::from(view.img_height) - (view.eye_height / 2.0),
                z: 0.0,
            };
            let dir = view.eye.vector_to(&screen_point).normalize();
            let r = Ray{p: view.eye, dir: dir};
            line.push(cast_vector(&r, &s));
        }
        bitmap.push(line);
    }
    bitmap
}

pub fn print_bitmap(bitmap: &Vec<Vec<Color>>) {
    for row in bitmap {
        for pixel in row {
            if (pixel.r + pixel.g + pixel.b) < 0.5 {
                print!("  ");
            } else {
                print!("##");
            }
        }
        print!("\n");
    }
}
