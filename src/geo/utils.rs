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
