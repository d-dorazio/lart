use lart::*;

sketch_parms! {
    n: u8 = 5
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("splines").with_page(Page::A4);

    let (w, h) = doc.dimensions();

    let rng = doc.rng();

    let mut path = Path::new();
    for _ in 0..parms.n {
        let p = V::new(rng.gen_range(0.1..0.9) * w, rng.gen_range(0.1..0.9) * h);

        path.push(p);
    }

    doc.geometry(path.clone());

    doc.layer(2).with_stroke("red");
    doc.geometry(spline::sample(&path, 0.1));

    doc.fit_to_page(20.0);

    doc.save().unwrap();
}
