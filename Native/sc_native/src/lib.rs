use std::ops::{Add, Mul, Sub};

/// A 3D vector.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn nan() -> Vec3 {
        Vec3 {
            x: f32::NAN,
            y: f32::NAN,
            z: f32::NAN,
        }
    }

    fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// A line in 3D space.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Line3 {
    a: Vec3,
    b: Vec3,
}

impl Line3 {
    fn new(a: Vec3, b: Vec3) -> Line3 {
        Line3 { a, b }
    }
}

/// A triangle in 3D space.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Triangle3 {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Triangle3 {
    fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle3 {
        Triangle3 { a, b, c }
    }

    /// An implementation of the Triangle-Triangle Intersection Test by Tomas Moeller: http://web.stanford.edu/class/cs277/resources/papers/Moller1997b.pdf
    /// 
    /// Returns the intersection of two triangles, if any.
    fn intersect(self, other: Triangle3) -> Option<Line3> {
        #[derive(Debug, Clone, Copy)]
        struct PlaneEq {
            n: Vec3,
            d: f32,
        }

        /// A plane equation.
        impl PlaneEq {
            /// Creates a new plane equation from a triangle.
            /// note that the normal has twice the area of the triangle
            fn new(t: Triangle3) -> PlaneEq {
                let n = (t.b - t.a).cross(t.c - t.a);
                PlaneEq { n, d: -n.dot(t.a) }
            }
            /// Returns the distances of the triangle vertices to the plane.
            /// Returns None if all vertices are on the same side of the plane
            fn distances(self, o: Triangle3) -> Option<[f32; 3]> {
                // points get inserted into the plane equation
                let d = [o.a, o.b, o.c].map(|p| self.n.dot(p) + self.d);

                let signs = d.map(|d| d.signum());
                // if all signs are the same
                if signs[0] == signs[1] && signs[1] == signs[2] {
                    return None;
                }
                Some(d)
            }
            /// Returns the intervals of intersection of the triangle with the plane of another triangle.
            /// This requires that one if the vertices of the triangle lies on the other side of the plane than the other two.
            fn intervals(self, o: Triangle3, d: [f32; 3], l: Vec3) -> (f32, f32) {
                let p: f32 = d.iter().product();
                // this is the index of the vertex that lies on the other side of the plane
                let i = d.iter().position(|&s| s * p > 0.0).unwrap();
                let j = [i, (i + 1) % 3, (i + 2) % 3];
                // the projection of the triangle vertices onto the line of intersection
                // the first vertex is the one that lies on the other side of the plane
                let p = [o.a.dot(l), o.b.dot(l), o.c.dot(l)];

                // interpolates between two projected vertices, based on the distance to the plane
                let t = |o| p[o] + (p[i] - p[o]) * d[o] / (d[o] - d[i]);
                let t = [t(j[1]), t(j[2])];

                // sort the intervals
                (t[0].min(t[1]), t[0].max(t[1]))
            }
        }

        let p0 = PlaneEq::new(self);
        // early return if all vertices of the other triangle lie on the same side of the plane
        let d0 = p0.distances(other)?;
        
        let p1 = PlaneEq::new(other);
        // -||-
        let d1 = p1.distances(self)?;

        // the line of intersection of the two planes
        let dir = p0.n.cross(p1.n);

        // calculating intervals of intersection
        let t = (p0.intervals(other, d0, dir), p1.intervals(self, d1, dir));

        // min and max of the intervals
        let t = (t.0 .0.max(t.1 .0), t.0 .1.min(t.1 .1));
        // if the intervals do intersect
        if t.0 <= t.1 {
            // the inverse square magnitudes of the normals
            let m = (1.0 / p0.n.dot(p0.n), 1.0 / p0.n.dot(p0.n));

            // calculates the offset of the line equation, considering a specific plane
            // (only needed to calculate the coordinates)
            let o = |n0, n1, d: f32, m0, m1| {
                // only if the plane is not at the origin
                if d.abs() < f32::EPSILON {
                    return Vec3::zero();
                }
                // the shortest vector from the origin to the plane
                let v: Vec3 = n0 * (d * m0);
                // project the vector into the other plane
                let p = n1 * (v.dot(n1) * m1);
                // squared magnitude of the vector
                let m = v.dot(v);
                // the projection, excluded from the vector
                let o = v - p;
                // keeping the lenght the same after excluding the projection
                o * (m / o.dot(o))
            };

            // offset of the line equation
            let o = o(p0.n, p1.n, p0.d, m.0, m.1) + o(p1.n, p0.n, p1.d, m.1, m.0);

            // the line equation
            let l = dir * (1.0 / dir.dot(dir));

            // subtract the offset from the line equation
            return Some(Line3::new(l * t.0 - o, l * t.1 - o));
        }
        None
    }
}

#[no_mangle]
pub extern "C" fn intersect(a: Triangle3, b: Triangle3) -> Line3 {
    if let Some(l) = a.intersect(b) {
        l
    } else {
        Line3::new(Vec3::nan(), Vec3::nan())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let t = parse_triangles();

        let l = t.0.intersect(t.1);

        if let Some(l) = l {
            println!("Line: {:?}", l);
        }
    }

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn parse_triangles() -> (Triangle3, Triangle3) {
        let path = "examples/triangles.obj";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let str = reader
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        let o_indices = str
            .match_indices("o ")
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        let numbers = o_indices[0..2]
            .iter()
            .map(|&i| {
                let mut j = i;
                let mut numbers = Vec::new();
                for k in 0..9 {
                    let mut number_str = String::new();
                    let mut was_number = false;
                    loop {
                        let c = str.chars().nth(j).unwrap();
                        if c.is_numeric() || c == '-' || c == '.' {
                            number_str.push(c);
                            was_number = true;
                        } else if was_number {
                            break;
                        }
                        j += 1;
                    }
                    numbers.push(number_str.parse::<f32>().unwrap());
                }
                numbers
            })
            .collect::<Vec<Vec<f32>>>();

        let triangles = numbers
            .iter()
            .map(|n| {
                Triangle3::new(
                    Vec3::new(n[0], n[1], n[2]),
                    Vec3::new(n[3], n[4], n[5]),
                    Vec3::new(n[6], n[7], n[8]),
                )
            })
            .collect::<Vec<Triangle3>>();

        (triangles[0], triangles[1])
    }
}
