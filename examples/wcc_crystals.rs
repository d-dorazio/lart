/// My entry for #WCCChallenge: Crystals
///
/// I tried to replicate some form of crystal growth coming out from the ground
/// or something like that, but I think the results look more like low-poly
/// corals/rocks than crystals. Anyway, I'm quite happy about how the plots came
/// out!
///
/// I wanted to add a few things to my framework (namely a generic system to
/// transform geometries), but unfortunately I didn't manage to and the code is
/// a bit messier than I'd like it to be, but it's okay.
///
use lart::*;

sketch_parms! {
    n: u16 = 6,
    ydivs: u16 = 6,
    h: f64 = 180.0,
    radius: f64 = 25.0,
    shapes: u16 = 15,
    space: f64 = 200.0,
    displacement: f64 = 3.0,
    n_shadows: u16 = 15,
}

fn crystal(doc: &mut Sketch, parms: &Parms, texture_spacing: f64) -> (Geometry, Geometry) {
    let mut horzs: Vec<Path> = vec![];
    let mut top = Polygon::new();

    for y in 0..parms.ydivs {
        let yt = mapu(y, 0, parms.ydivs - 1);
        let y0 = yt * -parms.h;

        let ngon: Path = polar_angles(parms.n)
            .map(|a| {
                let mut p = parms.radius * (1.0 - yt * 0.3) * v(a.cos(), a.sin() * 0.5);
                p += V::in_range(
                    doc,
                    -parms.displacement..=parms.displacement,
                    -parms.displacement..=parms.displacement,
                );
                v(0.0, y0) + p
            })
            .collect();

        let visible = usize::from(parms.n) / 2;
        horzs.push(ngon.slice(0..=visible));
        if y == parms.ydivs - 1 {
            top = Polygon::from(ngon);
        }
    }

    let mut cry = Geometry::new();
    cry.push_polygon(top);

    let mut texture = Geometry::new();

    for r in 0..horzs.len().saturating_sub(1) {
        let n = horzs[0].len();
        for c in 0..n.saturating_sub(1) {
            let quad = Polygon::from([
                horzs[r][c],
                horzs[r + 1][c],
                horzs[r + 1][c + 1],
                horzs[r][c + 1],
            ]);
            cry.push_polygon(quad.clone());

            if c == n - 2 {
                let bbox = quad.bbox().unwrap();
                let mut lines = Geometry::new();
                for x in frange(bbox.left() - bbox.width(), bbox.right(), texture_spacing) {
                    lines.push_path(Path::from([
                        v(x, bbox.top()),
                        v(x + bbox.width(), bbox.bottom()),
                    ]))
                }
                lines = lines & quad;
                texture.extend(&lines);
            }
        }
    }

    (cry, texture)
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("wcc_crystals").with_page(Page::A4);

    let mut crystals = vec![];

    for _ in 0..parms.shapes {
        let at = doc.gen_range(-0.4..=0.4);
        let a = at * PI;

        let cry_parms = Parms {
            h: parms.h * (0.4 + doc.gen_range(0.0..=(1.0 - at.abs() / 0.5))),
            radius: parms.radius * doc.gen_range(0.5..=1.0),
            ..parms
        };

        let (mut cry, mut text) = crystal(&mut doc, &cry_parms, map(at, -0.4, 0.4, 0.5, 1.5));

        cry *= Xform::rot(a);
        text *= Xform::rot(a);

        crystals.push((cry, text));
    }

    crystals.sort_by_cached_key(|(c, _)| F64Key(c.bbox().unwrap().area()));

    let bbox = bbox_union(crystals.iter().map(|(c, _)| c)).unwrap();
    let clip = Rect::with_dimensions(v(bbox.left(), bbox.bottom() - 20.0), bbox.width(), 30.0);

    let mut geos = Geometry::new();
    for (cry, text) in crystals {
        let boundary = &cry - &geos - Geometry::from(clip.clone());

        for p in cry.polygons() {
            let p = p & (&boundary);
            geos.extend(&p);
            doc.geometry(p);
        }

        doc.geometry(text & boundary);
    }

    let x0 = clip.left();
    let x1 = clip.right();
    let y0 = clip.top();
    let y1 = y0 + 10.0;
    doc.geometry(Path::from([v(x0, y0), v(x1, y0)]));

    let mut lines = 0;
    while lines < parms.n_shadows {
        let v0 = V::in_range(&mut doc, x0..=x1, y0..=y1);
        let v1 = v(doc.gen_range(x0..=x1), v0.y);

        if v0.dist(v1) < 30.0 {
            continue;
        }
        doc.geometry(Path::from([v0, v1]));
        lines += 1;
    }

    doc.fit_to_page(20.0);
    doc.save().unwrap();
}
