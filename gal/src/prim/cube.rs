use super::{
    builder::{IParametricBuilder, ITriangulatedBuilder},
    triangulated_mesh::TriangulatedMesh,
};

pub struct CubeBuilder;

impl CubeBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl IParametricBuilder for CubeBuilder {
    type Params = f32;

    fn params<F>(self, _func: F) -> Self
    where
        F: FnOnce(Self::Params) -> Self::Params,
    {
        self
    }
}

impl ITriangulatedBuilder for CubeBuilder {
    fn triangulated(self) -> TriangulatedMesh {
        let vertices: &[f64] = &[
            -0.5f64, -0.5f64, 0.5f64, 0.5f64, -0.5f64, 0.5f64, 0.5f64, 0.5f64, 0.5f64, -0.5f64,
            0.5f64, 0.5f64, -0.5f64, -0.5f64, -0.5f64, 0.5f64, -0.5f64, -0.5f64, 0.5f64, 0.5f64,
            -0.5f64, -0.5f64, 0.5f64, -0.5f64,
        ];
        let indices: &[usize] = &[
            0, 4, 5, 0, 5, 1, 1, 5, 6, 1, 6, 2, 2, 6, 7, 2, 7, 3, 3, 7, 4, 3, 4, 0, 3, 0, 1, 3, 1,
            2, 4, 7, 5, 5, 7, 6,
        ];
        TriangulatedMesh::from((vertices, indices))
    }
}

pub struct Cube;

impl Cube {
    pub fn build(vertices: &mut [f32], indices: &mut [u32]) {
        let vertex_data = [
            -0.5f32, -0.5f32, 0.5f32, 0.5f32, -0.5f32, 0.5f32, 0.5f32, 0.5f32, 0.5f32, -0.5f32,
            0.5f32, 0.5f32, -0.5f32, -0.5f32, -0.5f32, 0.5f32, -0.5f32, -0.5f32, 0.5f32, 0.5f32,
            -0.5f32, -0.5f32, 0.5f32, -0.5f32,
        ];
        vertices.copy_from_slice(&vertex_data);

        let index_data = [
            0, 4, 5, 0, 5, 1, 1, 5, 6, 1, 6, 2, 2, 6, 7, 2, 7, 3, 3, 7, 4, 3, 4, 0, 3, 0, 1, 3, 1,
            2, 4, 7, 5, 5, 7, 6,
        ];
        indices.copy_from_slice(&index_data);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
