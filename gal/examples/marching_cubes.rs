fn main() {
    let mut volume_data = gal::util::VolumeData::<u8, 5>::new();
    volume_data.set(1, 1, 1, u8::MAX);
    volume_data.set(1, 2, 1, u8::MAX);
    volume_data.set(1, 3, 2, u8::MAX);
    volume_data.set(2, 2, 1, u8::MAX);
    volume_data.set(2, 2, 2, u8::MAX);

    let triangulated_mesh = gal::meshgen::MarchingCubesMeshBuilder::new().build(&volume_data);

    let file = std::fs::File::create("volume_mesh.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &triangulated_mesh).unwrap();
}
