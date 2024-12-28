use num_traits::ToPrimitive;

use crate::{prim::TriangulatedMesh, traits::IVolumeData};

use super::detail;

pub struct MarchingCubesMeshBuilder {
    _threshold: f32,
}

impl MarchingCubesMeshBuilder {
    pub fn new() -> Self {
        Self { _threshold: 0.0 }
    }

    pub fn build<TVolumeData>(&self, volume_data: &TVolumeData) -> TriangulatedMesh
    where
        TVolumeData: IVolumeData<u8>,
    {
        let mut vertices = Vec::default();
        let mut vertex_count = 0;
        for z in 0..(volume_data.depth() - 1) {
            for y in 0..(volume_data.height() - 1) {
                for x in 0..(volume_data.width() - 1) {
                    let value = [
                        volume_data.get(x, y, z),
                        volume_data.get(x + 1, y, z),
                        volume_data.get(x, y + 1, z),
                        volume_data.get(x + 1, y + 1, z),
                        volume_data.get(x, y, z + 1),
                        volume_data.get(x + 1, y, z + 1),
                        volume_data.get(x, y + 1, z + 1),
                        volume_data.get(x + 1, y + 1, z + 1),
                    ];
                    let mut pattern = 0;
                    for index in 0..8 {
                        if self._threshold < (value[index].to_f32().unwrap()) {
                            pattern |= 1 << index;
                        }
                    }

                    let triangles = &detail::TRIANGLE_TABLE[pattern];
                    for triangle in triangles {
                        if *triangle == -1 {
                            continue;
                        }

                        let position = &detail::CUBE_MID_POINT_TABLE[*triangle as usize];
                        let position_x = position[0] as f64 + x as f64;
                        let position_y = position[1] as f64 + y as f64;
                        let position_z = position[2] as f64 + z as f64;
                        vertices.push(position_x);
                        vertices.push(position_y);
                        vertices.push(position_z);
                        vertex_count += 1;
                    }
                }
            }
        }

        // インデクス
        let mut indices = Vec::default();
        for i in 0..(vertex_count as usize / 3) {
            let index0 = 3 * i + 0;
            let index1 = 3 * i + 1;
            let index2 = 3 * i + 2;
            indices.push(index0);
            indices.push(index1);
            indices.push(index2);
        }

        TriangulatedMesh::from((&vertices as &[f64], &indices as &[usize]))
    }
}
