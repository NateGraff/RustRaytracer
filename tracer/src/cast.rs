use data::*;
use scene::*;

pub fn cast_vector(r: &Ray, s: &Scene) -> Color {
    for sphere in &s.spheres {
        match sphere.intersection(r) {
            Some(_) =>
                return Color{r: 0.0, g: 0.0, b: 0.0},
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
                print!("00");
            } else {
                print!("11");
            }
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}