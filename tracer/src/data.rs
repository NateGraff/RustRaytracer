use std::ops::Sub;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn vector_to(&self, other: &Point) -> Vector {
        Vector{x: other.x - self.x, y: other.y - self.y, z: other.z - self.z}
    }

    pub fn vector_from(&self, other: &Point) -> Vector {
        other.vector_to(self)
    }

    pub fn translate(&self, v: &Vector) -> Point {
        Point{x: self.x + v.x, y: self.y + v.y, z: self.z + v.z}
    }
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn scale(&self, scalar: f64) -> Vector {
        Vector {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let length = self.len();
        self.scale(1.0 / length)
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

pub struct Ray {
    pub p: Point,
    pub dir: Vector,
}

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Finish {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub roughness: f64,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub c: Color,
    pub f: Finish,
}

impl Sphere {
    pub fn intersection(&self, r: &Ray) -> Option<Point> {
        let a = r.dir.dot(&r.dir);
        let b = self.center.vector_to(&r.p).scale(2.0).dot(&r.dir);
        let c = self.center.vector_to(&r.p).dot(&self.center.vector_to(&r.p)) - self.radius.powf(2.0);

        let delta = (b.powf(2.0) - 4.0 * a * c).sqrt();
        let t1 = (-1.0 * b - delta) / (2.0 * a);
        let t2 = (-1.0 * b + delta) / (2.0 * a);

        if t1 >= 0.0 && t2 >= 0.0 {
            // two intersections
            return Some(r.p.translate(&r.dir.scale(t1.min(t2))));
        }
        else if t1 >= 0.0 && t2 < 0.0 {
            // one intersection
            return Some(r.p.translate(&r.dir.scale(t1)));
        }
        else if t1 < 0.0 && t2 >= 0.0 {
            // one intersection
            return Some(r.p.translate(&r.dir.scale(t2)));
        }
        else {
            return None;
        }
    }

    pub fn normal_at(&self, p: &Point) -> Vector {
        self.center.vector_to(&p).normalize()
    }
}

pub struct Light {
    pub p: Point,
    pub c: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Point method tests
    #[test]
    fn test_vector_to_point() {
        let p1 = Point{x: 0.0, y: 0.0, z: 0.0};
        let p2 = Point{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(p1.vector_to(&p2), Vector{x: 1.0, y: 1.0, z: 1.0});
        assert_eq!(p2.vector_to(&p1), Vector{x: -1.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn test_vector_from_point() {
        let p1 = Point{x: 0.0, y: 0.0, z: 0.0};
        let p2 = Point{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(p2.vector_from(&p1), Vector{x: 1.0, y: 1.0, z: 1.0});
        assert_eq!(p1.vector_from(&p2), Vector{x: -1.0, y: -1.0, z: -1.0});
    }

    #[test]
    fn test_translate_point() {
        let p1 = Point{x: 0.0, y: 0.0, z: 0.0};
        let v1 = Vector{x: 1.0, y: 1.0, z: 1.0};
        let p2 = Point{x: 1.0, y: 1.0, z: 1.0};
        assert_eq!(p1.translate(&v1), p2);
    }

    // Vector method tests

    #[test]
    fn test_scale_vector() {
        let v = Vector{x: 1.0, y: 2.0, z: 3.0};
        assert_ne!(v, Vector{x: 2.0, y: 4.0, z: 6.0});
        assert_eq!(v.scale(2.0), Vector{x: 2.0, y: 4.0, z: 6.0});
    }

    #[test]
    fn test_dot_vector() {
        let v1 = Vector{x: 1.0, y: 0.0, z: 0.0};
        let v2 = Vector{x: 0.0, y: 1.0, z: 0.0};
        let v3 = Vector{x: 3.0, y: 0.0, z: 0.0};
        assert_eq!(v1.dot(&v1), 1.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v1.dot(&v3), 3.0);
        assert_eq!(v3.dot(&v1), 3.0);
    }

    #[test]
    fn test_len_vector() {
        let v1 = Vector{x: 1.0, y: 0.0, z: 0.0};
        let v2 = Vector{x: 3.0, y: 4.0, z: 0.0};
        assert_eq!(v1.len(), 1.0);
        assert_eq!(v2.len(), 5.0);
    }

    #[test]
    fn test_normalize_vector() {
        let v1 = Vector{x: 1.0, y: 0.0, z: 0.0};
        let v2 = Vector{x: 3.0, y: 0.0, z: 0.0};
        assert_eq!(v1.normalize(), v1);
        assert_eq!(v2.normalize(), v1);
    }

    #[test]
    fn test_subtract_vector() {
        let v1 = Vector{x: 1.0, y: 0.0, z: 0.0};
        let v2 = Vector{x: 1.0, y: 1.0, z: 0.0};
        let v3 = Vector{x: 0.0, y: 1.0, z: 0.0};
        assert_eq!(v2 - v3, v1);
    }

    #[test]
    fn test_intersection_sphere() {
        let s = Sphere {
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 1.0,
            c: Color{r: 0.0, g: 0.0, b: 0.0},
            f: Finish{ambient: 0.0, diffuse: 0.0, specular: 0.0, roughness: 0.0}
        };
        let r1 = Ray {
            p: Point{x: 3.0, y: 0.0, z: 0.0},
            dir: Vector{x: -1.0, y: 0.0, z: 0.0}
        };
        let r2 = Ray {
            p: Point{x: 3.0, y: 0.0, z: 0.0},
            dir: Vector{x: 1.0, y: 0.0, z: 0.0}
        };
        assert_eq!(s.intersection(&r1), Some(Point{x: 1.0, y: 0.0, z: 0.0}));
        assert_eq!(s.intersection(&r2), None);
    }

    #[test]
    fn test_normal_at_sphere() {
        let s = Sphere {
            center: Point{x: 0.0, y: 0.0, z: 0.0},
            radius: 1.0,
            c: Color{r: 0.0, g: 0.0, b: 0.0},
            f: Finish{ambient: 0.0, diffuse: 0.0, specular: 0.0, roughness: 0.0}
        };
        let p = Point{x: 1.0, y: 0.0, z: 0.0};
        assert_eq!(s.normal_at(&p), Vector{x: 1.0, y: 0.0, z: 0.0});
    }
}
