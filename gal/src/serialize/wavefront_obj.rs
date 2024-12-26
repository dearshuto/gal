use std::io::Write;

use crate::ITrianguratedMeshProvider;

pub struct WavefrontObj;

impl WavefrontObj {
    pub fn serialize<W, M>(mut writer: W, mesh_data_provider: &M) -> Result<(), ()>
    where
        W: Write,
        M: ITrianguratedMeshProvider,
    {
        let vertices = mesh_data_provider
            .vertices()
            .map(|(x, y, z)| obj_exporter::Vertex { x, y, z })
            .collect();
        let geometry = mesh_data_provider
            .triangles()
            .map(|(x, y, z)| obj_exporter::Geometry {
                material_name: None,
                shapes: vec![obj_exporter::Shape {
                    primitive: obj_exporter::Primitive::Triangle(
                        (x, Some(x), Some(0)),
                        (y, Some(y), Some(0)),
                        (z, Some(z), Some(0)),
                    ),
                    groups: Vec::default(),
                    smoothing_groups: Vec::default(),
                }],
            })
            .collect();

        let obj_set = obj_exporter::ObjSet {
            material_library: None,
            objects: vec![obj_exporter::Object {
                name: String::from("obj"),
                vertices,
                tex_vertices: Vec::default(),
                normals: Vec::default(),
                geometry,
            }],
        };
        obj_exporter::export(&obj_set, &mut writer).unwrap();

        Ok(())
    }
}
