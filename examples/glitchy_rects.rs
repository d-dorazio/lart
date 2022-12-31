use lart::*;

sketch_parms! {
    shapes: u16 = 300,
    minw: f64 = 5.0,
    maxw: f64 = 15.0,
    minh: f64 = 5.0,
    maxh: f64 = 15.0,
    debug: bool = false,
}

fn main() {
    let parms = Parms::from_cli();

    let mut doc = Sketch::new("glitchy_rects")
        .with_page(Page::A4)
        // .with_page(Page(210.0, 270.0))
        ;
    let bbox = doc.page_bbox().padded(-25.0);

    let mut c = 0;
    while c < parms.shapes {
        let tl = V::in_rect(&mut doc, &bbox);
        let max = bbox.max() - tl;

        if max.x < parms.minw || max.y < parms.minh {
            continue;
        }
        c += 1;

        let r = Rect::with_dimensions(
            tl,
            doc.gen_range(parms.minw..=max.x.min(parms.maxw)),
            doc.gen_range(parms.minh..=max.y.min(parms.maxh)),
        );

        if parms.debug {
            doc.geometry(Polygon::from(r));
            continue;
        }

        if doc.gen_bool(0.5) {
            doc.layer(1);
            for y in frange(r.top(), r.bottom() + 1.0, 1.0) {
                doc.geometry(Path::from([v(r.left(), y), v(r.right(), y)]));
            }
        } else {
            doc.layer(2);
            for x in frange(r.left(), r.right() + 1.0, 1.0) {
                doc.geometry(Path::from([v(x, r.top()), v(x, r.bottom())]));
            }
        }
    }

    doc.save().unwrap();
}
