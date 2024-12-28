use super::{
    builder::{IParametricBuilder, ITriangulatedBuilder},
    triangulated_mesh::TriangulatedMesh,
};

pub struct BezierSurfaceBuilder {
    /* 制御点の順番はこんな感じ
    0---1---2---3
    4---5---6---7
    8---9---10--11
    12--13--14--15
    */
    control_points: [(f32, f32, f32); 16],
}

impl IParametricBuilder for BezierSurfaceBuilder {
    type Params = [(f32, f32, f32); 16];

    fn params<F>(mut self, func: F) -> Self
    where
        F: FnOnce(Self::Params) -> Self::Params,
    {
        self.control_points = func(self.control_points);
        self
    }
}

impl BezierSurfaceBuilder {
    pub fn new() -> Self {
        BezierSurfaceBuilder {
            control_points: [
                (0.0f32, 0.0f32, 0.0f32),
                (1.0 / 3.0f32, 0.0f32, 0.0f32),
                (2.0 / 3.0f32, 0.0f32, 0.0f32),
                (1.0f32, 0.0f32, 0.0f32),
                (0.0f32, 1.0 / 3.0f32, 0.0f32),
                (1.0 / 3.0f32, 1.0 / 3.0f32, 0.0f32),
                (2.0 / 3.0f32, 1.0 / 3.0f32, 0.0f32),
                (1.0f32, 1.0 / 3.0f32, 0.0f32),
                (0.0f32, 2.0 / 3.0f32, 0.0f32),
                (1.0 / 3.0f32, 2.0 / 3.0f32, 0.0f32),
                (2.0 / 3.0f32, 2.0 / 3.0f32, 0.0f32),
                (1.0f32, 2.0 / 3.0f32, 0.0f32),
                (0.0f32, 1.0f32, 0.0f32),
                (1.0 / 3.0f32, 1.0f32, 0.0f32),
                (2.0 / 3.0f32, 1.0f32, 0.0f32),
                (1.0f32, 1.0f32, 0.0f32),
            ],
        }
    }

    fn compute_bernstein_polynormal(t: f32, i: u32) -> f32 {
        // 3 次バーンスタイン多項式
        const N: u64 = 3; // 3次なので

        // 二項係数
        let binomial_coefficients =
            Self::factorial(N as u32) / (Self::factorial(i) * Self::factorial(N as u32 - i));

        let v = (1.0f32 - t).powi(N as i32 - i as i32);
        return binomial_coefficients as f32 * num::pow(t, i as usize) as f32 * v;
    }

    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }
}

impl ITriangulatedBuilder for BezierSurfaceBuilder {
    fn triangulated(self) -> super::triangulated_mesh::TriangulatedMesh {
        let mut vertices = Vec::default();
        let mut indices = Vec::default();

        // たてよこの頂点数を取得。
        // 両端に頂点があるので +2 してます。
        let div = 10;
        let sub_div = 10;
        let v_count = div + 1;
        let u_count = sub_div + 1;

        // 各頂点の UV 座標
        // i 番目の要素がが m_vertices[i] の UV を表します
        // UV はこの関数内でしか使わないので、頂点と UV で別変数を管理してます。
        let mut uv = Vec::default();

        // 頂点数分の領域を確保
        uv.resize(v_count * u_count, BezierCoordinate { u: 0.0, v: 0.0 });

        // 足しこみを行うので 全ての要素を 0 で初期化しておく
        vertices.resize(v_count * u_count * 3, 0.0f64);

        // ベジエ曲面上で隣り合う頂点の距離
        // 等分割を前提にしているので、すべて同じ感覚で並んでます。
        let stride_u = 1.0f32 / div as f32;
        let stride_v = 1.0f32 / sub_div as f32;

        // 求める頂点の UV 座標を算出。
        for n in 0..v_count {
            for m in 0..u_count {
                uv[n + v_count * m].u = stride_u * (n as f32);
                uv[n + v_count * m].v = stride_v * (m as f32);
            }
        }

        // 全頂点に対してベジエ曲面上の座標を算出
        for n in 0..v_count {
            for m in 0..u_count {
                let u = uv[n + v_count * m].u;
                let v = uv[n + v_count * m].v;
                let index = (n + m * v_count) * 3;

                // ずべての制御点からの影響を足しこむ
                for i in 0..4 {
                    for j in 0..4 {
                        let control_point = &self.control_points[i + 4 * j];
                        let bu = Self::compute_bernstein_polynormal(u, i as u32) as f64;
                        let bv = Self::compute_bernstein_polynormal(v, j as u32) as f64;
                        let (x, y, z) = (
                            bu * bv * control_point.0 as f64,
                            bu * bv * control_point.1 as f64,
                            bu * bv * control_point.2 as f64,
                        );
                        vertices[index] += x;
                        vertices[index + 1] += y;
                        vertices[index + 2] += z;
                    }
                }
            }
        }

        // 三角ポリゴンとして登録。
        for n in 0..(v_count - 1) {
            for m in 0..(u_count - 1) {
                let left_up = n + m * v_count;
                indices.push(left_up);
                indices.push(left_up + v_count + 1);
                indices.push(left_up + v_count);

                indices.push(left_up);
                indices.push(left_up + 1);
                indices.push(left_up + v_count + 1);
            }
        }

        TriangulatedMesh::from((&vertices as &[f64], &indices as &[usize]))
    }
}

#[derive(Debug, Clone, Copy)]
struct BezierCoordinate {
    u: f32,
    v: f32,
}
