use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    u32,
};

pub trait IGraph {
    type VertexId: Hash + Eq + Copy + Clone;
    type EdgeId: Copy + Clone;

    fn vertex_ids(&self) -> impl Iterator<Item = Self::VertexId>;

    fn neighbors(&self, id: Self::VertexId) -> impl Iterator<Item = Self::VertexId>;

    fn cost(&self, lhs: Self::VertexId, rhs: Self::VertexId) -> u32;
}

pub struct Dijkstra;

impl Dijkstra {
    pub fn calculate_cost<TGraph>(
        begin: TGraph::VertexId,
        end: TGraph::VertexId,
        graph: &TGraph,
    ) -> Result<(u32, Vec<TGraph::VertexId>), ()>
    where
        TGraph: IGraph,
    {
        // 探索途中のコストを登録しておくテーブル
        // 開始点への到達コストはゼロなのでそれははじめに登録しておく
        let mut distance_map = HashMap::<TGraph::VertexId, (u32, Vec<TGraph::VertexId>)>::from([(
            begin,
            (0, vec![begin]),
        )]);

        let mut known_nodes = HashSet::from([begin]);
        let mut unknown_nodes: HashSet<_> = graph.vertex_ids().filter(|id| *id != begin).collect();

        while !unknown_nodes.is_empty() {
            // 距離を更新
            // 確定させるノードの識別子を返す
            let closest_node = {
                let mut neighbor_distance_min = u32::MAX;
                let mut closest_node = None;
                for current in &known_nodes {
                    let (current_distance, path) = distance_map.get(&current).unwrap().clone();

                    for neighbor in graph
                        .neighbors(*current)
                        .filter(|id| unknown_nodes.contains(id))
                    {
                        // 次の走査点への距離を計算
                        let distance = graph.cost(*current, neighbor) + current_distance;

                        // 近接点の最短をキャッシュしておく処理
                        // より短い距離で行けるご近所が見つかれば更新しておく
                        if distance < neighbor_distance_min {
                            closest_node = Some(neighbor);
                            neighbor_distance_min = distance;
                        }

                        // 登録済みの距離を取得
                        // 初回訪問ならそのまま登録
                        distance_map
                            .entry(neighbor)
                            .and_modify(|(previous_distance, previous_path)| {
                                // すでに登録済みの経路の方が安上がりなら何もしない
                                if *previous_distance < distance {
                                    return;
                                }

                                // より短い距離で訪問できたので更新
                                *previous_distance = distance;
                                let mut new_path = path.clone();
                                new_path.push(neighbor);
                                *previous_path = new_path;
                            })
                            .or_insert_with(|| {
                                let mut new_path = path.clone();
                                new_path.push(neighbor);
                                (distance, new_path)
                            });
                    }
                }
                closest_node.unwrap()
            };

            //
            known_nodes.insert(closest_node);
            unknown_nodes.remove(&closest_node);
        }

        let cost = distance_map.get(&end).unwrap();
        Ok(cost.clone())
    }
}
