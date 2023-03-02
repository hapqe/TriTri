use std::collections::{HashMap, HashSet};

use crate::math::{Line3, Mat4, Vec3};

pub struct Caster {
    verts: Vec<Vec3>,
    indices: Vec<i32>,
    transform: Mat4,
}

impl Caster {
    pub fn new(verts: Vec<Vec3>, indices: Vec<i32>, transform: Mat4) -> Caster {
        Caster {
            verts,
            indices,
            transform,
        }
    }
}

pub struct IndexedLine {
    line: Line3,
    tris: (Option<usize>, Option<usize>),
}

pub fn calculate_edges(caster: Caster) -> Vec<IndexedLine> {
    let Caster { verts, indices, .. } = caster;

    let mut map = HashSet::new();

    for indices in indices.chunks(3) {
        let v: Vec<_> = indices.iter().map(|i| verts[*i as usize]).collect();

        map.insert(Line3::new(v[0], v[1]));
        map.insert(Line3::new(v[1], v[2]));
        map.insert(Line3::new(v[2], v[0]));
    }

    let mut applied = HashMap::new();
    for i in indices.iter() {
        let v = verts[*i as usize];
        let t = (*i as f32 / 3.0).floor();
        applied
            .entry(v)
            .or_insert(HashSet::new())
            .insert(t as usize);
    }

    map.iter()
        .map(|l| {
            let mut tris = applied[&l.a].intersection(&applied[&l.b]);

            IndexedLine {
                line: *l,
                tris: (tris.next().copied(), tris.next().copied()),
            }
        })
        .collect()
}
