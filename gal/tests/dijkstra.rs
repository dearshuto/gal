// 始点: 0, 終点: 1
// ----0 --- 1--- 5
// |   |     |    |
// |   |-----2----4
// |         |    |
// 3----------    |
// |              |
// ----------------

use std::collections::HashMap;

struct Graph {
    neighbor_table: [Vec<usize>; 6],
    cost_table: HashMap<(usize, usize), u32>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            neighbor_table: [
                vec![1, 2, 3],
                vec![0, 2, 5],
                vec![0, 1, 3, 4],
                vec![0, 2, 4],
                vec![2, 3, 5],
                vec![1, 2, 4],
            ],
            cost_table: HashMap::from([
                ((0, 1), 5),
                ((0, 2), 4),
                ((0, 3), 2),
                ((1, 2), 2),
                ((2, 3), 3),
                ((3, 4), 6),
                ((1, 5), 6),
                ((4, 5), 4),
                ((2, 4), 2),
            ]),
        }
    }
}

impl gal::graph::IGraph for Graph {
    type VertexId = usize;
    type EdgeId = usize;

    fn vertex_ids(&self) -> impl Iterator<Item = Self::VertexId> {
        0..6
    }

    fn neighbors(&self, id: Self::VertexId) -> impl Iterator<Item = Self::VertexId> {
        self.neighbor_table[id].clone().into_iter()
    }

    fn cost(&self, lhs: Self::VertexId, rhs: Self::VertexId) -> u32 {
        let key = if lhs < rhs { (lhs, rhs) } else { (rhs, lhs) };

        let Some(cost) = self.cost_table.get(&key) else {
            return u32::MAX;
        };

        *cost
    }
}

#[test]
fn it_adds_two() {
    let graph = Graph::new();
    let (cost, path) = gal::graph::Dijkstra::calculate_cost(0, 5, &graph).unwrap();
    assert_eq!(cost, 10);
    assert_eq!(path, [0, 2, 4, 5])
}
