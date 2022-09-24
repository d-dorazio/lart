/// My entry for #WCCChallenge: Wiggly
///
/// I've been quite stressed the past few weeks. I got promoted to tech leader
/// and even though I'm really happy about it, I'm also quite worried about the
/// "leader" part of the title. Also, days are starting to get shorter and I
/// fear seasonal depression is kicking in.
///
/// Sometimes, when I'm stressed I doodle random curves just to see some ink
/// flowing on the page. I find it really relaxing and calming and I thought
/// that making the plotter draw those curves would have been even better.
///
/// So, here it is. Just random curves with no purpose whatsoever.
///
/// Also, this sketch is using my WIP framework for pen-plotter ready SVGs
/// written in Rust. I'm quite proud of how it's going so far, but I still have
/// a lot more to add and polish.
///
/// I won't be watching the stream live, so I guess it's my time to wish you all
/// a great week!
///
use lart::*;

sketch_parms! {
    rows: u16 = 3,
    cols: u16 = 2,
    n: u16 = 5,
    nsplines: u16 = 1,
}

fn glyph(doc: &mut Sketch, n: u16) -> Path {
    let mut rng = doc.rng();

    let mut points = vec![];
    for a in polar_angles(n) {
        points.push(V::polar(a, rng.gen_range(0.3..=0.5)));
    }
    points.shuffle(&mut rng);
    Path::from(points)
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("wcc_wiggly").with_page(Page::A4);

    let (w, h) = doc.dimensions();
    let (ww, hh) = (w / f64::from(parms.cols), h / f64::from(parms.rows));

    for g in grid_positions(parms.cols, parms.rows) {
        for _ in 0..parms.nsplines {
            let n = doc.rng().gen_range(0..=1) + parms.n;

            let mut path = glyph(&mut doc, n);

            path.transform(&mut |p| g * v(w, h) + p * v(ww, hh) * 0.8);

            doc.layer(1).with_stroke("black").with_pen_width(0.7);
            doc.geometry(spline::sample(&path, 0.1));
        }
    }

    doc.fit_to_page(20.0);

    doc.save().unwrap();
}
