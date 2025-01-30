use std::collections::{HashMap, HashSet};

use crate::{prim::TriangulatedMesh, ITrianguratedMeshProvider};

pub fn calculate_convex_hull<TMesh>(mesh: &TMesh) -> TriangulatedMesh
where
    TMesh: ITrianguratedMeshProvider,
{
    // 四面体を拡大して凸包を作成する
    let simplex = crate::prim::Builder::tetrahedron().triangulated();

    let vertices: Vec<_> = simplex.vertices().collect();
    let vertex_ids: Vec<_> = (0..4).map(|_| VertexId::new()).collect();
    let facets: HashMap<_, _> = mesh
        .triangles()
        .map(|(i0, i1, i2)| {
            (
                FacetId::new(),
                Facet {
                    vertex_ids: [vertex_ids[i0], vertex_ids[i1], vertex_ids[i2]],
                    neighbors: Default::default(),
                    outside_set: Default::default(),
                },
            )
        })
        .collect();
    let vertices: HashMap<_, _> = vertex_ids
        .iter()
        .zip(vertices.iter())
        .map(|(id, v)| (*id, *v))
        .collect();
    let non_empty_facet_id_set = HashSet::new();

    // assign p to facet's outside set
    for (facet_id, facet) in facets {
        for (vertex_id, position) in &vertices {
            let is_above = is_above(*vertex_id, &facet, &vertices);
            if !is_above {
                continue;
            }

            facet.outside_set.insert(*vertex_id);
            non_empty_facet_id_set.insert(facet_id);
        }
    }

    while !non_empty_facet_id_set.is_empty() {
        for facet_id in &non_empty_facet_id_set {
            let facet = facets.get(facet_id).unwrap();

            // select the furthest point p
            let furthest_vertex_id = facet
                .outside_set
                .iter()
                // .max_by(|vertex_id, y| false)
                .last()
                .unwrap();
            let (x, y, z) = vertices.get(furthest_vertex_id).unwrap();

            // visible set to F
            let visible_facets = HashSet::default();

            let new_facets = Vec::default();
            let horizon_ridges = Vec::default();
            for horizon_ridge in horizon_ridges {
                // create a new facet from horizon_ridge and p
                // link the new facet to its neighbors
            }

            // each unassigned point q in an outside set of a facet
            for new_facet in new_facets {
                for visible_facet in visible_facets {
                    // if q is above new_facet...
                    // assign q to new_facet's outside set
                }
            }

            // delete the facets in visible_facets
        }
    }

    // 四面体を拡大して凸包を作成する
    crate::prim::Builder::tetrahedron().triangulated()
}

fn is_above(
    vertex: VertexId,
    facet: &Facet,
    vertex_table: &HashMap<VertexId, (f64, f64, f64)>,
) -> bool {
    // ファセットの各頂点
    let facet_v0 = vertex_table.get(&facet.vertex_ids[0]).unwrap();
    let facet_v1 = vertex_table.get(&facet.vertex_ids[0]).unwrap();
    let facet_v2 = vertex_table.get(&facet.vertex_ids[0]).unwrap();

    // ファセットの方向を階席から算出
    let facet_v0 = nalgebra::Vector3::new(facet_v0.0, facet_v0.1, facet_v0.2);
    let facet_v1 = nalgebra::Vector3::new(facet_v1.0, facet_v1.1, facet_v1.2);
    let facet_v2 = nalgebra::Vector3::new(facet_v2.0, facet_v2.1, facet_v2.2);
    let direction = (facet_v1 - facet_v0).cross(&(facet_v2 - facet_v1));

    // ファセット上の頂点からターゲットの頂点に向かうベクトルを考えて、
    // ファセットの方向と一致していたら "上" とみなす
    let vertex_position = vertex_table.get(&vertex).unwrap();
    let vertex_position =
        nalgebra::Vector3::new(vertex_position.0, vertex_position.1, vertex_position.2);
    direction.dot(&(vertex_position - facet_v0)) > 0.0
}

struct Facet {
    vertex_ids: [VertexId; 3],
    neighbors: HashSet<FacetId>,
    outside_set: HashSet<VertexId>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct FacetId(uuid::Uuid);
impl FacetId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct VertexId(uuid::Uuid);
impl VertexId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}
