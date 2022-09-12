#include "rust/cxx.h"

#include <memory>

struct V;
struct Path;
struct Polygon;
struct Geometry;

class Clipper
{
public:
    Clipper();

    void add_polygon(Polygon const & /*polygon*/);
    void add_polyline(Path const & /*polyline*/);
    void add_clip(Polygon const & /*polygon*/);

    Geometry union_();
    Geometry intersection();
    Geometry difference();
    Geometry symmetric_difference();

private:
    struct pimpl;
    std::shared_ptr<pimpl> impl;
};

std::unique_ptr<Clipper> new_clipper();

Geometry buffer(Geometry const & /*geo*/, double /*delta*/);
