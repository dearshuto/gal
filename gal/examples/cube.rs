use std::fs::File;

fn main() {
    let cube = gal::prim::Builder::cube()
        .with_params(|_| 0.5f32)
        .triangulated();

    let file = File::create("cube.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &cube).unwrap();
}
