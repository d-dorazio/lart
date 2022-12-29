use lart::*;

sketch_parms! {
    strata: u8 = 4,
    da: f64 = 5.0,
    minh: f64 = 100.0,
    maxh: f64 = 240.0,

    shadow_thickness: f64 = 1.5,
    trunk_height: f64 = 8.0,
    trunk_dy: f64 = -2.0,
    ground_height: f64 = 1.0,

    pen_width: f64 = 0.2,
}

fn arc(a: f64, r: f64) -> Path {
    frange(a, PI - a, f64::to_radians(1.0))
        .map(|t| V::polar(t, r))
        .collect()
}

fn yrepeat<'a>(p: &'a Path, ys: impl Iterator<Item = f64> + 'a) -> impl Iterator<Item = Path> + 'a {
    ys.map(|yd| p.clone() * Xform::xlate(v(0.0, yd)))
}

pub fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("xmas-tree")
        // .with_page(Page::A4)
        .with_page(Page(105.0, 140.0))
        .plugin(Vpype::new(&["linesimplify", "squiggles"]));

    let da = parms.da.to_radians();
    let a = f64::to_radians(60.0) + doc.gen_range(-da..=da);
    for i in 0..parms.strata {
        let r = map(i, 0, parms.strata, parms.minh, parms.maxh);
        let rr = map(i.saturating_sub(1), 0, parms.strata, parms.minh, parms.maxh);

        let texline = arc(a, r);
        let poly;
        if i == 0 {
            let mut p = Path::new();
            p.push(v(0, 0));
            p.extend(texline.iter());

            poly = Polygon::from(p);
        } else {
            let aa = f64::max(0.0, a + doc.gen_range(0.0..=da));

            let mut p = arc(aa, rr);
            p.reverse();
            p.extend(texline.iter());
            poly = Polygon::from(p);
        }

        let mut texture = Geometry::new();
        for yd in frange(0.0, poly.bbox().unwrap().height(), 1.0) {
            let mut l = texline.clone();
            l.reverse();
            l = l.slice(..doc.gen_range(texline.len() / 4..texline.len() / 2));
            l *= Xform::xlate(v(0.0, -yd));
            texture.push_path(l);
        }

        for l in yrepeat(
            &texline,
            frange(0.0, parms.shadow_thickness, parms.pen_width),
        ) {
            doc.geometry(l);
        }

        if i == parms.strata - 1 {
            let trunk = arc(f64::to_radians(80.0), r);
            for l in yrepeat(&trunk, frange(0.0, parms.trunk_height, parms.pen_width)) {
                doc.geometry(l);
            }

            let bbox = texline.bbox().unwrap();
            let ground_line = Path::from([
                v(bbox.left() - 5.0, r + parms.trunk_dy),
                v(bbox.right() + 5.0, r + parms.trunk_dy),
            ]);
            for l in yrepeat(
                &ground_line,
                frange(0.0, parms.ground_height, parms.pen_width),
            ) {
                doc.geometry(Geometry::from(l) - &poly);
            }
        }

        doc.geometry(texture & &poly);
        doc.geometry(poly);
    }

    doc.fit_to_page(10.0);
    doc.save().unwrap();
}
