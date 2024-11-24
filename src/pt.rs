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

pub struct Node<'a> {
    point: &'a (u32, u32),
    idx: usize,
}

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
            edges: vec![],
        };
    }
}
