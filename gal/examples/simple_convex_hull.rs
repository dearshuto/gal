use std::fs::File;

fn main() {
    let teapot = gal::prim::Builder::teapot().triangulated();

    let convex_hull = gal::meshgen::calculate_convex_hull(&teapot);
    let file = File::create("convex_hull_teapot.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &convex_hull).unwrap();
}
