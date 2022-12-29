use std::collections::HashMap;

use lart::*;

type Cache = HashMap<(i64, i64), u8>;

fn split(h: &mut Cache, p: &Path) -> Vec<Path> {
    // FIXME: this is borked and doesn't work too well, but it did ok for this
    // sketch
    let mut res = vec![];

    let prec = 50.0;
    let p = p.simplify(1.0 / prec);

    let mut cur = Path::new();
    for vv in p.iter() {
        let ix = (vv.x * prec).round() as i64;
        let iy = (vv.y * prec).round() as i64;

        let c = h.entry((ix, iy)).or_default();

        if *c > 3 {
            if !cur.is_empty() {
                res.push(cur);
                cur = Path::new();
            }
        } else {
            *c += 1;
            cur.push(v(ix as f64 / prec, iy as f64 / prec));
        }
    }

    if !cur.is_empty() {
        res.push(cur);
    }

    res
}

fn main() {
    let mut doc = Sketch::new("ptpx_2022").with_page(Page(105.0, 140.0));
    let bbox = doc.page_bbox();

    let mut path = Path::new();
    for i in 0..3 {
        let x = map(i, 0, 2, -0.5, 0.5) * bbox.width();
        path.push(v(x, doc.gen_range(0.0..bbox.height() / 2.0)));
    }
    path = spline::sample(&path, 0.1);

    let mut c = Cache::new();
    for _i in 0..75 {
        for p in split(&mut c, &path) {
            doc.geometry(p);
        }

        // doc.geometry(path.clone());
        path *= Xform::scale(v(0.94, 1.0)) * Xform::xlate(v(0.0, 0.6));

        if path.bbox().unwrap().width() < 3.0 {
            break;
        }
    }

    doc.fit_to_page(5.0);
    doc.save().unwrap();
}
