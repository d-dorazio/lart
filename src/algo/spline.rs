use crate::{Path, V};

pub fn sample(path: &Path, d: f64) -> Path {
    if path.is_empty() {
        return Path::new();
    }

    let l = path.norm();
    let n = f64_to_usize((l / d).ceil(), 0);

    let mut p = Path::with_capacity(n);

    let mut t = 0.0;
    while t <= l {
        p.push(get_point(path, t / l));
        t += d;
    }

    p
}

fn get_point(path: &Path, t: f64) -> V {
    let p = t * (path.len() - 1) as f64;

    let ip = p.floor() as usize;
    let weight = p.fract();

    let p0 = path[ip.saturating_sub(1)];
    let p1 = path[ip];
    let p2 = path[usize::min(ip + 1, path.len() - 1)];
    let p3 = path[usize::min(ip + 2, path.len() - 1)];

    catmull_rom(weight, p0, p1, p2, p3)
}

#[inline]
fn f64_to_usize(n: f64, def: usize) -> usize {
    usize::try_from(n as u64).unwrap_or(def)
}

fn catmull_rom(t: f64, p0: V, p1: V, p2: V, p3: V) -> V {
    let v0 = (p2 - p0) * 0.5;
    let v1 = (p3 - p1) * 0.5;
    let t2 = t * t;
    let t3 = t * t2;

    (2.0 * p1 - 2.0 * p2 + v0 + v1) * t3 + (-3.0 * p1 + 3.0 * p2 - 2.0 * v0 - v1) * t2 + v0 * t + p1
}
