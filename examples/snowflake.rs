use lart::*;

sketch_parms! {
    symmetry: u8 = 7,
    min_dx: i32 = 7,
    min_dy: i32 = 5,
    max_dx: i32 = 22,
    max_dy: i32 = 15,
    step: usize = 5,
    buff: u32 = 5,
    xdivs: u32 = 1,
    ydivs: u32 = 1,
}

pub fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("snowflake")
        .with_page(Page::A4)
        .with_background("#A0E3F6");

    doc.layer(1).with_fill("white").with_stroke("none");

    for bbox in doc.page_bbox().subdivide(parms.xdivs, parms.ydivs) {
        let mut geo = Geometry::new();

        for y in (0..=100).step_by(parms.step) {
            if doc.gen_bool(1.0 - mapu(y, 0, 100)) {
                continue;
            }

            let dx = doc.gen_range(parms.min_dx..=parms.max_dx);
            let dy = [-1, 1].choose(&mut doc).unwrap() * doc.gen_range(parms.min_dy..=parms.max_dy);

            geo.push_path([v(0, y), v(dx, y + dy)].into());
        }
        geo.extend(&(geo.clone() * Xform::scale(v(-1, 1))));
        geo.push_path(Path::from([v(0, 0), v(0, 100)]));

        let mut snowflake = Geometry::new();
        for a in 0..parms.symmetry {
            snowflake.extend(&(geo.clone() * Xform::rot(map(a, 0, parms.symmetry, 0, TAU))));
        }

        snowflake = snowflake.buffer(parms.buff.into());
        snowflake *= Xform::rect_to_rect(&snowflake.bbox().unwrap(), &bbox.padded(-5.0));

        doc.geometry(snowflake);
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
