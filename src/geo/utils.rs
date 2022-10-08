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
