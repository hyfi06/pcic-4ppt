use crate::graph_utils::{on_segment, orientation};
use std::ops::Deref;

#[derive(Debug)]
pub struct PointSet {
    pub points: Vec<(u32, u32)>, // Usa u8 o u16 según el archivo
}

impl Deref for PointSet {
    type Target = Vec<(u32, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

#[derive(Debug)]
pub struct Node<'a> {
    point: &'a (u32, u32),
    idx: usize,
}

#[derive(Debug)]
pub struct PartialPT<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<(usize, usize)>,
}

impl<'a> PartialPT<'a> {
    pub fn from_point_set(point_set: &'a PointSet) -> Self {
        let nodes: Vec<Node> = point_set
            .iter()
            .enumerate()
            .map(|(idx, point)| Node { point, idx })
            .collect();
        return PartialPT {
            nodes,
            edges: Vec::new(),
        };
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
            .any(|&edge2| self.edges_cross(edge, edge2));

        if conflict {
            return Err("The edge intersects with another existing one".to_string());
        }

        self.edges.push(edge);
        Ok(())
    }

    fn edges_cross(&self, edge1: (usize, usize), edge2: (usize, usize)) -> bool {
        let (a1, b1) = edge1;
        let (a2, b2) = edge2;

        let p1 = self.nodes[a1].point;
        let p2 = self.nodes[b1].point;
        let q1 = self.nodes[a2].point;
        let q2 = self.nodes[b2].point;

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

    pub fn convex_hull(&self) -> Vec<&Node> {
        let mut nodes: Vec<&Node> = self.nodes.iter().collect();

        nodes.sort_by(|a, b| {
            if a.point.0 == b.point.0 {
                a.point.1.cmp(&b.point.1)
            } else {
                a.point.0.cmp(&b.point.0)
            }
        });

        //  Graham Scan
        let mut lower_hull: Vec<&Node<'a>> = Vec::new();
        nodes.iter().for_each(|&node| {
            while lower_hull.len() >= 2
                && orientation(
                    lower_hull[lower_hull.len() - 2].point,
                    lower_hull[lower_hull.len() - 1].point,
                    node.point,
                ) != 2
            {
                lower_hull.pop();
            }
            lower_hull.push(node);
        });

        let mut upper_hull: Vec<&Node<'a>> = Vec::new();
        nodes.iter().rev().for_each(|&node| {
            while upper_hull.len() >= 2
                && orientation(
                    upper_hull[upper_hull.len() - 2].point,
                    upper_hull[upper_hull.len() - 1].point,
                    node.point,
                ) != 2
            {
                upper_hull.pop();
            }
            upper_hull.push(node);
        });

        lower_hull.pop();
        upper_hull.pop();

        lower_hull.extend(upper_hull);
        lower_hull
    }
}
