use lart::*;

sketch_parms! {
    circles: u16 = 100,
    max_radius: f64 = 30.0,
    min_radius: f64 = 2.0,
    lines: u16 = 50,
    margin: f64 = 5.0,
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("circle_packing").with_page(Page::A4);

    let (w, h) = doc.dimensions();

    let mut cp = CirclePacker::new(Rect::with_dimensions(v(0.0, 0.0), w, h));
    cp.margin = parms.margin;

    cp.min_radius = parms.min_radius;
    cp.max_radius = parms.max_radius;
    for _ in 0..parms.circles {
        cp.generate(&mut doc);
    }

    for &(c, r) in cp.circles() {
        let mut path = Path::new();
        for _ in 0..parms.lines {
            path.push(c + V::polar(doc.gen_range(0.0..=TAU), r));
        }
        doc.geometry(spline::sample(&path, 0.1));
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
