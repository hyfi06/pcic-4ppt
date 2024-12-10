pub fn orientation(p: &(u32, u32), q: &(u32, u32), r: &(u32, u32)) -> i32 {
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

pub fn on_segment(p: &(u32, u32), q: &(u32, u32), r: &(u32, u32)) -> bool {
    q.0 >= p.0.min(r.0) && q.0 <= p.0.max(r.0) && q.1 >= p.1.min(r.1) && q.1 <= p.1.max(r.1)
}

pub fn cicle_to_edges(cicle: &Vec<usize>) -> Vec<(usize, usize)> {
    cicle
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let next = cicle.get(i + 1).unwrap_or(&cicle[0]);
            (x, *next)
        })
        .collect()
}

pub fn canonicalize_cycle(cycle: Vec<usize>) -> Vec<usize> {
    let mut canonical_cycle = cycle.clone();

    // Si es un ciclo de 4, rotarlo para que comience con el menor vértice
    let min_idx = canonical_cycle
        .iter()
        .enumerate()
        .min_by_key(|&(_, &val)| val)
        .map(|(i, _)| i)
        .unwrap();

    canonical_cycle.rotate_left(min_idx);

    canonical_cycle
}
