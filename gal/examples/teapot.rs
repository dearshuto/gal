use std::fs::File;

fn main() {
    let teapot = gal::prim::Builder::teapot().triangulated();
    let file = File::create("teapot.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &teapot).unwrap();
}
