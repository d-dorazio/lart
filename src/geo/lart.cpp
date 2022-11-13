#include "lart/src/geo/types.rs.h"

#include "clipper2/clipper.h"

static Path to_path(Clipper2Lib::Path64 const &path64, double precision)
{
    Path pp;
    pp.points.reserve(path64.size());
    for (auto p : path64)
        pp.points.push_back(V{p.x / precision, p.y / precision});
    return pp;
}

static Clipper2Lib::Path64 to_path64(Path const &path, double precision)
{
    Clipper2Lib::Path64 path64(path.points.size());

    for (size_t i = 0; i < path64.size(); ++i)
    {
        V p = path.points[i];
        path64[i] = Clipper2Lib::Point64{p.x * precision, p.y * precision};
    }

    return path64;
}

static Clipper2Lib::Paths64 to_paths64(Polygon const &polygon, double precision)
{
    Clipper2Lib::Paths64 paths64(polygon.areas.size());

    for (size_t i = 0; i < paths64.size(); ++i)
        paths64[i] = to_path64(polygon.areas[i], precision);

    return paths64;
}

struct Clipper::pimpl
{
    Clipper2Lib::Clipper64 clipper;
    double precision = 1000;

    Geometry execute(Clipper2Lib::ClipType ct)
    {
        Clipper2Lib::PolyTree64 polytree;
        Clipper2Lib::Paths64 paths;

        bool ok = clipper.Execute(ct, Clipper2Lib::FillRule::EvenOdd, polytree, paths);
        if (!ok)
            return Geometry{};

        Geometry geo;
        geo.polygons.reserve(polytree.Count());
        geo.paths.reserve(paths.size());

        for (auto const &path64 : paths)
            geo.paths.push_back(to_path(path64, precision));

        for (auto *poly : polytree)
        {
            Polygon p;
            p.areas.reserve(1);

            std::vector<Clipper2Lib::PolyPath64 *> stack;
            stack.push_back(poly);

            while (!stack.empty())
            {
                auto *polypath = stack.back();
                stack.pop_back();

                p.areas.push_back(to_path(polypath->Polygon(), precision));

                p.areas.reserve(p.areas.size() + polypath->Count());
                stack.insert(std::end(stack), std::begin(*polypath), std::end(*polypath));
            }

            geo.polygons.push_back(std::move(p));
        }

        return geo;
    }
};

Clipper::Clipper()
    : impl(std::make_shared<pimpl>())
{
}

void Clipper::add_polygon(Polygon const &polygon)
{
    impl->clipper.AddSubject(to_paths64(polygon, impl->precision));
}

void Clipper::add_polyline(Path const &polyline)
{
    impl->clipper.AddOpenSubject({to_path64(polyline, impl->precision)});
}

void Clipper::add_clip(Polygon const &polygon)
{
    impl->clipper.AddClip(to_paths64(polygon, impl->precision));
}

Geometry Clipper::union_()
{
    return impl->execute(Clipper2Lib::ClipType::Union);
}

Geometry Clipper::intersection()
{
    return impl->execute(Clipper2Lib::ClipType::Intersection);
}

Geometry Clipper::difference()
{
    return impl->execute(Clipper2Lib::ClipType::Difference);
}

Geometry Clipper::symmetric_difference()
{
    return impl->execute(Clipper2Lib::ClipType::Xor);
}

std::unique_ptr<Clipper> new_clipper()
{
    return std::make_unique<Clipper>();
}

Geometry buffer(Geometry const &geo, double delta)
{
    Clipper2Lib::ClipperOffset off;
    double precision = 1000.0;

    for (auto const &poly : geo.polygons)
        off.AddPaths(to_paths64(poly, precision), Clipper2Lib::JoinType::Round, Clipper2Lib::EndType::Polygon);

    for (auto const &path : geo.paths)
        off.AddPath(to_path64(path, precision), Clipper2Lib::JoinType::Round, Clipper2Lib::EndType::Round);

    Clipper2Lib::Paths64 paths = off.Execute(delta * precision);

    Polygon poly;
    poly.areas.reserve(paths.size());

    for (auto const &p : paths)
    {
        poly.areas.push_back(to_path(p, precision));
    }

    Geometry out;
    if (!poly.areas.empty())
        out.polygons.push_back(std::move(poly));
    return out;
}
