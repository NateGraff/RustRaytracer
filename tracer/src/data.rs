use std::ops::Sub;

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn vector_to(&self, other: &Point) -> Vector {
        Vector{x: other.x - self.x, y: other.y - self.y, z: other.z - self.z}
    }

    fn vector_from(&self, other: &Point) -> Vector {
        other.vector_to(self)
    }

    fn translate(&self, v: &Vector) -> Point {
        Point{x: self.x + v.x, y: self.y + v.y, z: self.z + v.z}
    }
}

#[derive(PartialEq, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn scale(&self, scalar: f64) -> Vector {
        Vector {x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn len(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    fn normalize(&self) -> Vector {
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

struct Ray {
    p: Point,
    dir: Vector,
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

struct Finish {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    roughness: f64,
}

struct Sphere {
    center: Point,
    radius: f64,
    c: Color,
    f: Finish,
}

impl Sphere {
    fn intersection(&self, r: &Ray) -> Option<Point> {
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

    fn normal_at(&self, p: &Point) -> Vector {
        self.center.vector_to(&p).normalize()
    }
}

struct Light {
    p: Point,
    c: Color,
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
