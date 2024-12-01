use std::io;
mod graph_utils;
mod loader;
mod pt;
fn main() -> io::Result<()> {
    let filename = "otypes06.b08";
    let point_count = 6;
    let byte_size = 1;

    let point_sets = loader::load_file(filename, point_count, byte_size)?;
    let mut graph = pt::PartialPT::from_point_set(&point_sets[1]);
    let ch: Vec<usize> = graph.convex_hull();
    let mut iter_ch = ch.iter();
    while let (Some(&x),Some(&y)) = (iter_ch.next(), iter_ch.next()) {
        graph.add_edge((x,y)).ok();
    }
    graph.add_edge((1,2)).ok();
    println!("{:?}", ch);
    println!("{:?}", graph);
    Ok(())
}

fn create_pairs(vec: &[usize]) -> Vec<(usize, usize)> {
    vec.iter()
        .enumerate()
        .map(|(i, &x)| {
            let next = vec.get(i + 1).unwrap_or(&vec[0]); // Toma el siguiente elemento, o el primero si es el último
            (x, *next)
        })
        .collect()
}
