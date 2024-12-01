use crate::graph_utils::{on_segment, orientation};
use std::{ops::Deref, vec};

#[derive(Debug, Clone)]
pub struct PointSet {
    pub points: Vec<(u32, u32)>, // Usa u8 o u16 según el archivo
}

impl Deref for PointSet {
    type Target = Vec<(u32, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    point: (u32, u32),
    idx: usize,
}

impl Node {
    fn get_coord(&self) -> &(u32, u32) {
        return &self.point;
    }

    fn get_idx(&self) -> usize {
        return self.idx.clone();
    }
}

#[derive(Debug, Clone)]
pub struct PartialPT {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl PartialPT {
    pub fn from_point_set(point_set: &PointSet) -> Self {
        let nodes: Vec<Node> = point_set
            .iter()
            .enumerate()
            .map(|(idx, &point)| Node { point, idx })
            .collect();
        let mut pt = PartialPT {
            nodes,
            edges: Vec::new(),
        };

        return pt;
    }

    pub fn add_edge(&mut self, edge: (usize, usize)) -> Result<(), String> {
        let (idx1, idx2) = edge;

        if idx1 >= self.nodes.len() || idx2 >= self.nodes.len() {
            return Err(format!("Some index is out of range: ({}, {})", idx1, idx2));
        }

        // Canonic form (a,b) a < b
        let edge = if idx1 < idx2 {
            (idx1, idx2)
        } else {
            (idx2, idx1)
        };

        // No repetitions
        if self.edges.contains(&edge) {
            return Err("Edge already exists".to_string());
        }

        // No crosses
        let conflict: bool = self
            .edges
            .iter() //todo! "par_iter"
            .any(|edge2| self.edges_cross(&edge, edge2));

        if conflict {
            return Err("The edge intersects with another existing one".to_string());
        }

        self.edges.push(edge);
        Ok(())
    }

    fn edges_cross(&self, edge1: &(usize, usize), edge2: &(usize, usize)) -> bool {
        // Introduction to Algorithms - Thomas H. Cormen et al (1018)
        let (a1, b1) = edge1.clone();
        let (a2, b2) = edge2.clone();

        let p1 = self.nodes[a1].get_coord();
        let p2 = self.nodes[b1].get_coord();
        let q1 = self.nodes[a2].get_coord();
        let q2 = self.nodes[b2].get_coord();

        let o1 = orientation(p1, p2, q1);
        let o2 = orientation(p1, p2, q2);
        let o3 = orientation(q1, q2, p1);
        let o4 = orientation(q1, q2, p2);

        // Crossing condition
        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Collinear condition

        if o1 == 0 && on_segment(p1, q1, p2) {
            return true;
        }
        if o2 == 0 && on_segment(p1, q2, p2) {
            return true;
        }
        if o3 == 0 && on_segment(q1, p1, q2) {
            return true;
        }
        if o4 == 0 && on_segment(q1, p2, q2) {
            return true;
        }

        false
    }

    pub fn convex_hull(&self) -> Vec<usize> {
        let mut nodes: Vec<usize> = (0..self.nodes.len()).collect();

        nodes.sort_by(|&a, &b| {
            if self.nodes[a].get_coord().0 == self.nodes[b].get_coord().0 {
                self.nodes[a]
                    .get_coord()
                    .1
                    .cmp(&self.nodes[b].get_coord().1)
            } else {
                self.nodes[a]
                    .get_coord()
                    .0
                    .cmp(&self.nodes[b].get_coord().0)
            }
        });

        //  Graham Scan
        // Introduction to Algorithms - Thomas H. Cormen et al (1031)
        let mut lower_hull: Vec<usize> = Vec::new();
        nodes.iter().for_each(|&node_idx| {
            while lower_hull.len() >= 2
                && orientation(
                    self.nodes[lower_hull[lower_hull.len() - 2]].get_coord(),
                    self.nodes[lower_hull[lower_hull.len() - 1]].get_coord(),
                    self.nodes[node_idx].get_coord(),
                ) != 2
            {
                lower_hull.pop();
            }
            lower_hull.push(node_idx);
        });

        let mut upper_hull: Vec<usize> = Vec::new();
        nodes.iter().rev().for_each(|&node_idx| {
            while upper_hull.len() >= 2
                && orientation(
                    self.nodes[upper_hull[upper_hull.len() - 2]].get_coord(),
                    self.nodes[upper_hull[upper_hull.len() - 1]].get_coord(),
                    self.nodes[node_idx].get_coord(),
                ) != 2
            {
                upper_hull.pop();
            }
            upper_hull.push(node_idx);
        });

        lower_hull.pop();
        upper_hull.pop();
        lower_hull.extend(upper_hull);
        lower_hull
    }

    pub fn is_a_possible_ppt(&self) -> bool {
        return self.edges.len() == 2 * self.nodes.len() - 3;
    }
}

// pub fn find_pseudo_triangles(initial_state: &mut PartialPT) {
//     let mut solutions: Vec<PartialPT> = Vec::new();
//     let convex_hull = initial_state.convex_hull();

//     convex_hull.windows(2).for_each(|window| {
//         if let [w1, w2] = window {
//             initial_state.add_edge((w1.idx.clone(), w2.idx.clone()));
//         }
//     });

//     let possible_edges: Vec<(usize, usize)> = (0..initial_state.nodes.len())
//         .flat_map(|i| (i + 1..initial_state.nodes.len()).map(move |j| (i, j)))
//         .filter(|edge| !initial_state.edges.contains(edge))
//         .collect();
// }
