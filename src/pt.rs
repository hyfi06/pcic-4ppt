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

        fn orientation(p: &(u32, u32), q: &(u32, u32), r: &(u32, u32)) -> i32 {
            let val = (q.1 as i64 - p.1 as i64) * (r.0 as i64 - q.0 as i64)
                - (q.0 as i64 - p.0 as i64) * (r.1 as i64 - q.1 as i64);
            if val == 0 {
                0 // collinear
            } else if val > 0 {
                1 // Clockwise direction
            } else {
                2 // Counterclockwise
            }
        }

        let o1 = orientation(p1, p2, q1);
        let o2 = orientation(p1, p2, q2);
        let o3 = orientation(q1, q2, p1);
        let o4 = orientation(q1, q2, p2);

        // Crossing condition
        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Collinear condition
        fn on_segment(p: &(u32, u32), q: &(u32, u32), r: &(u32, u32)) -> bool {
            q.0 >= p.0.min(r.0) && q.0 <= p.0.max(r.0) && q.1 >= p.1.min(r.1) && q.1 <= p.1.max(r.1)
        }

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
}
