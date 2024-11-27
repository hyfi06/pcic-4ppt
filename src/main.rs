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
    let ch = graph.convex_hull();
    let mut iter_ch = ch.iter();
    while let (Some(x),Some(y)) = (iter_ch.next(),iter_ch.next()) {
        graph.add_edge((x.idx,y.idx));
    }
    println!("{:?}", ch);
    println!("{:?}", graph);
    Ok(())
}

