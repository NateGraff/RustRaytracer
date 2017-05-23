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

#[cfg(test)]
mod tests {
    use super::*;


}