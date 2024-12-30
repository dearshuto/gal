use std::fs::File;

fn main() {
    let tetrahedron = gal::prim::Builder::tetrahedron().triangulated();
    let file = File::create("tetrahedron.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &tetrahedron).unwrap();
}
