pub fn frange(mut a: f64, b: f64, s: f64) -> impl Iterator<Item = f64> {
    std::iter::from_fn(move || {
        if (b - a).abs() < s.abs() {
            return None;
        }

        let v = a;
        a += s;
        Some(v)
    })
}

/// Map the input value from the given range to the unit range [0..1]
pub fn mapu(v: impl Into<f64>, start: impl Into<f64>, end: impl Into<f64>) -> f64 {
    let start = start.into();
    (v.into() - start) / (end.into() - start)
}

/// Map the input value in the unit interval to the given one.
pub fn umap(v: f64, ostart: impl Into<f64>, oend: impl Into<f64>) -> f64 {
    let ostart = ostart.into();
    ostart + v * (oend.into() - ostart)
}

/// Map the input value from the given input range to the given output range.
pub fn map(
    v: impl Into<f64>,
    start: impl Into<f64>,
    end: impl Into<f64>,
    ostart: impl Into<f64>,
    oend: impl Into<f64>,
) -> f64 {
    umap(mapu(v, start, end), ostart, oend)
}

/// Dead simple wrapper over a f64 that can be used as the key to the various
/// sort_by_key functions.
///
/// Note that it behaves slightly differently than f64 with regard to equality
/// checking namely that `-0.0_f64 == 0.0_f64` but `F64Key(-0.0) != F64Key(0.0)`.
#[derive(Debug, Clone, Copy)]
pub struct F64Key(pub f64);

impl Eq for F64Key {}
impl PartialEq for F64Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
}

impl Ord for F64Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl PartialOrd for F64Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.total_cmp(&other.0).into()
    }
}
