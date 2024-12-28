use std::fs::File;

fn main() {
    let bezier = gal::prim::Builder::bezier()
        .with_params(|_| {
            [
                // 0-3
                (0.0f32, 0.0f32, 0.0f32),
                (0.33f32, 0.0f32, 0.2f32),
                (0.66f32, 0.0f32, 0.4f32),
                (1.0f32, 0.0f32, 0.0f32),
                // 4-7
                (0.0f32, 0.33f32, 0.0f32),
                (0.33f32, 0.33f32, 0.3f32),
                (0.66f32, 0.33f32, 0.3f32),
                (1.0f32, 0.33f32, 0.0f32),
                // 8-11
                (0.0f32, 0.66f32, 0.0f32),
                (0.33f32, 0.66f32, -0.3f32),
                (0.66f32, 0.66f32, -0.3f32),
                (1.0f32, 0.66f32, 0.0f32),
                // 12-15
                (0.0f32, 1.0f32, 0.0f32),
                (0.33f32, 1.0f32, 0.0f32),
                (0.66f32, 1.0f32, 0.0f32),
                (1.0f32, 1.0f32, 0.0f32),
            ]
        })
        .triangulated();
    let file = File::create("bezier.obj").unwrap();
    gal::serialize::WavefrontObj::serialize(file, &bezier).unwrap();
}
