use crate::prim::triangulated_mesh::TriangulatedMesh;

use super::builder::ITriangulatedBuilder;

pub struct SphereBuilder;

impl ITriangulatedBuilder for SphereBuilder {
    fn triangulated(self) -> super::triangulated_mesh::TriangulatedMesh {
        let div = 20;
        let sub_div = 20;
        const RADIUS: f64 = 0.5;
        let radian = 2.0 * std::f64::consts::PI / (div as f64);
        let radian_z = std::f64::consts::PI / (sub_div as f64);
        let mut vertices = Vec::default();
        vertices.reserve(div * (sub_div - 1) + 2 /*北極と南極の分*/);

        for m in 1..sub_div {
            let z = RADIUS * (m as f64 * radian_z).cos();
            let local_radius = RADIUS * (m as f64 * radian_z).sin();

            for n in 0..div {
                let x = local_radius * (n as f64 * radian).cos();
                let y = local_radius * (n as f64 * radian).sin();

                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
            }
        }

        let mut indices = Vec::default();
        for n in 0..(div - 1) {
            for m in 0..(sub_div - 2) {
                let left_up = n + m * div;
                let next_row = div;
                let next_column = 1;
                indices.push(left_up);
                indices.push(left_up + next_row);
                indices.push(left_up + next_row + next_column);

                indices.push(left_up);
                indices.push(left_up + next_row + next_column);
                indices.push(left_up + next_column);
            }
        }

        // 周回
        for m in 0..(sub_div - 2) {
            indices.push(m * div);
            indices.push((m + 1) * div - 1);
            indices.push((m + 1) * div);

            indices.push((m + 1) * div - 1);
            indices.push((m + 2) * div - 1);
            indices.push((m + 1) * div);
        }

        // 北極
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(RADIUS);

        for n in 0..(div - 1) {
            indices.push(vertices.len() / 3 - 1);
            indices.push(n);
            indices.push(n + 1);
        }
        indices.push(vertices.len() / 3 - 1);
        indices.push(div - 2 + 1);
        indices.push(0);

        // 南極
        vertices.push(0.0);
        vertices.push(0.0);
        vertices.push(-RADIUS);
        for n in 0..(div - 1) {
            indices.push(vertices.len() / 3 - 1);
            indices.push(vertices.len() / 3 - 3 - n);
            indices.push(vertices.len() / 3 - 4 - n);
        }

        indices.push(vertices.len() / 3 - 1);
        indices.push(vertices.len() / 3 - 4 - (div - 2));
        indices.push(vertices.len() / 3 - 3 - 0);

        let vertices: &[f64] = &vertices;
        let indices: &[usize] = &indices;
        TriangulatedMesh::from((vertices, indices))
    }
}
