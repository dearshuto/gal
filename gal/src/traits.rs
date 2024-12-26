pub trait ITrianguratedMeshProvider {
    fn vertices(&self) -> impl Iterator<Item = (f64, f64, f64)>;

    fn triangles(&self) -> impl Iterator<Item = (usize, usize, usize)>;
}
