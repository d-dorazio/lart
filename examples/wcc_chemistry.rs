/// My entry for #WCCChallenge: Chemistry
///
/// I tried to replicate the aesthetics of skeletal formulas which I find
/// particularly interesting to look at even though I don't really understand
/// them. I'm a sucker for diagrams, I think.
///
/// My brother, who studies chemistry, says that these drawings are not accurate
/// at all :D
///
/// Anyway, the generated diagrams miss the names of the various atoms composing
/// the molecule, but I still have to add support for vector fonts in my
/// framework. I did a good amount of work on the interactive viewer and the
/// geometric primitives though.
///
use std::collections::HashSet;

use lart::*;

sketch_parms! {
    n: u16 = 3,
    extra: u16 = 5,
}

const HEX_NEIGHBORS: [(i32, i32, f64); 6] = [
    (0, -1, 0.0),
    (1, 0, TAU / 6.0),
    (1, 1, TAU / 3.0),
    (0, 1, TAU / 2.0),
    (-1, 0, -TAU / 3.0),
    (-1, -1, -TAU / 6.0),
];

fn hexagon(c: V) -> Path {
    polar_angles(6)
        .map(|a| c + V::polar(a, 0.5))
        .collect::<Path>()
}

struct HexGrid {
    pts: HashSet<(i32, i32)>,
    di: V,
    dj: V,
    hex_side: f64,
}

impl HexGrid {
    pub fn new() -> Self {
        let hex_side = (v(0.5, 0.0) - V::polar(TAU / 6.0, 0.5)).norm();
        let di = (v(0.5, 0.0) + V::polar(TAU / 6.0, 0.5)) / 2.0;
        let dj = v(-di.x, di.y);

        Self {
            di,
            dj,
            hex_side,
            pts: HashSet::new(),
        }
    }

    pub fn insert(&mut self, i: i32, j: i32) -> bool {
        self.pts.insert((i, j))
    }

    pub fn remove(&mut self, i: i32, j: i32) {
        self.pts.remove(&(i, j));
    }

    pub fn contains(&self, i: i32, j: i32) -> bool {
        self.pts.contains(&(i, j))
    }

    pub fn project(&self, i: i32, j: i32) -> V {
        f64::from(i * 2) * self.di + f64::from(j * 2) * self.dj
    }
}

fn main() {
    let parms = Parms::from_cli();

    let mut doc = Sketch::new("wcc_chemistry").with_page(Page::A4);

    let mut grid = HexGrid::new();
    let (mut i, mut j) = (0, 0);
    while grid.pts.len() < usize::from(parms.n + parms.extra) {
        let (di, dj, _) = HEX_NEIGHBORS.choose(&mut doc).unwrap();

        let n =
            doc.gen_range(1..=usize::min(usize::from(parms.n + parms.extra) - grid.pts.len(), 3));

        for _ in 0..n {
            grid.insert(i + di, j + dj);
            i += di;
            j += dj;
        }
    }

    // try to disconnect the grid
    for _ in 0..parms.extra {
        let p = *grid.pts.iter().next().unwrap();
        grid.remove(p.0, p.1);
    }

    for &(i, j) in &grid.pts {
        let c = grid.project(i, j);
        let hex = hexagon(c);

        // boundary
        doc.geometry(Polygon::from(hex.clone()));

        {
            // internal decoration
            let xform = Xform::xlate(-c) * Xform::scale(v(0.9, 0.9)) * Xform::xlate(c);
            let hex = hex * xform;

            let s = doc.gen_range(0..6);
            for i in 0..=doc.gen_range(0..=1) {
                doc.geometry(hex.segment((s + i * 2) % 6));
            }
        }

        // external connections
        if doc.gen_bool(0.8) {
            let mut free_neighbors = vec![];
            for (ii, (di, dj, a)) in HEX_NEIGHBORS.iter().enumerate() {
                if grid.contains(i + di, j + dj) {
                    continue;
                }

                let (di, dj, _) = HEX_NEIGHBORS[(ii + 1) % HEX_NEIGHBORS.len()];
                if grid.contains(i + di, j + dj) {
                    continue;
                }

                free_neighbors.push(*a);
            }

            if let Some(a) = free_neighbors.choose(&mut doc) {
                let d = V::polar(*a, 0.5);

                if doc.gen_bool(0.6) {
                    // lines
                    let da = f64::to_radians(3.0);
                    let (off, r) = if doc.gen_bool(0.4) {
                        (vec![0.0], 0.5)
                    } else {
                        (vec![da, -da], 0.55)
                    };

                    for o in off {
                        let start = c + V::polar(a + o, r);
                        doc.geometry(Path::from([
                            start,
                            start + d.normalized() * (grid.hex_side * (1.0 - r)),
                        ]));
                    }
                } else {
                    // wedges
                    let da = f64::to_radians(10.0);
                    let start = c + d;

                    doc.layer(2);
                    doc.geometry(Polygon::from([
                        start,
                        start + V::polar(a + da, 0.4),
                        start + V::polar(a - da, 0.4),
                    ]));
                    doc.layer(1);
                }
            }
        }
    }

    doc.fit_to_page(20.0);

    // fill wedges after scaling
    // NOTE: this way of filling is super inefficient, but I didn't have the
    // time to implement something smarter
    let mut geo = doc.layer(2).geo().clone();
    loop {
        geo = geo.buffer(-0.2);
        if geo.is_empty() {
            break;
        }
        doc.geometry(geo.clone());
    }

    doc.save().unwrap();
}
