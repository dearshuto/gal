use std::fs::File;

fn main() {
    let sphere = gal::prim::Builder::sphere().triangulated();
    let file = File::create("sphere.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &sphere).unwrap();
}
