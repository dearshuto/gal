use crate::{prim::TriangulatedMesh, ITrianguratedMeshProvider};

pub fn calculate_convex_hull<TMesh>(mesh: &TMesh) -> TriangulatedMesh
where
    TMesh: ITrianguratedMeshProvider,
{
    // 四面体を拡大して凸包を作成する
    crate::prim::Builder::tetrahedron().triangulated()
}
