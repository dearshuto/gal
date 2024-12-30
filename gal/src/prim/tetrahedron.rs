use core::f64;

use super::{builder::ITriangulatedBuilder, triangulated_mesh::TriangulatedMesh};

pub struct TetrahedronBuilder;

impl TetrahedronBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl ITriangulatedBuilder for TetrahedronBuilder {
    fn triangulated(self) -> TriangulatedMesh {
        let radius: f64 = 3.0f64.sqrt() / 3.0;
        let vertices: &[f64] = &[
            0.0,
            0.0,
            6.0f64.sqrt() / 3.0,
            radius,
            0.0,
            0.0,
            radius * (f64::cos(2.0 * f64::consts::PI / 3.0)),
            radius * (f64::sin(2.0 * f64::consts::PI / 3.0)),
            0.0,
            radius * (f64::cos(-2.0 * f64::consts::PI / 3.0)),
            radius * (f64::sin(-2.0 * f64::consts::PI / 3.0)),
            0.0,
        ];
        let indices: &[usize] = &[
            0, 1, 2, //
            0, 2, 3, //
            0, 3, 1, //
            3, 2, 1, // 底面
        ];

        TriangulatedMesh::from((vertices, indices))
    }
}
