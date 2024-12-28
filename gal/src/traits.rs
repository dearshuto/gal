pub trait ITrianguratedMeshProvider {
    fn vertices(&self) -> impl Iterator<Item = (f64, f64, f64)>;

    fn triangles(&self) -> impl Iterator<Item = (usize, usize, usize)>;
}

pub trait IVolumeData<T>
where
    T: num_traits::NumOps,
{
    fn get(&self, x: usize, y: usize, z: usize) -> T;

    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn depth(&self) -> usize;
}
