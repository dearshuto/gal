use crate::ITrianguratedMeshProvider;

pub struct TriangulatedMesh {
    vertices: Vec<(f64, f64, f64)>,
    indices: Vec<(usize, usize, usize)>,
}

impl From<(&[f64], &[usize])> for TriangulatedMesh {
    fn from(value: (&[f64], &[usize])) -> Self {
        TriangulatedMesh {
            vertices: value.0.chunks(3).map(|v| (v[0], v[1], v[2])).collect(),
            indices: value.1.chunks(3).map(|v| (v[0], v[1], v[2])).collect(),
        }
    }
}

impl ITrianguratedMeshProvider for TriangulatedMesh {
    fn vertices(&self) -> impl Iterator<Item = (f64, f64, f64)> {
        self.vertices.clone().into_iter()
    }

    fn triangles(&self) -> impl Iterator<Item = (usize, usize, usize)> {
        self.indices.clone().into_iter()
    }
}
