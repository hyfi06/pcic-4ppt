use std::{collections::HashSet, io};
mod graph_utils;
mod loader;
mod pt;
use pt::PartialPT;

fn main() -> io::Result<()> {
    let filename = "otypes06.b08";
    let point_count = 6;
    let byte_size = 1;

    let point_sets = loader::load_file(filename, point_count, byte_size)?;
    let mut graph = pt::PartialPT::from_point_set(&point_sets[1]);
    graph.add_edge((0, 4)).ok();
    graph.add_edge((1, 3)).ok();
    graph.add_edge((3, 4)).ok();
    graph.add_edge((4, 5)).ok();
    graph.draw_ascii(40, 30);
    println!("{:?}",graph);

    println!("{:?}",graph.faces());

    // find_pseudo_triangles(&mut graph);
    Ok(())
}

fn find_pseudo_triangles(initial_state: &mut PartialPT) {
    let mut solutions: Vec<PartialPT> = Vec::new();

    let possible_edges: Vec<(usize, usize)> = (0..initial_state.get_nodes_len())
        .flat_map(|i| (i + 1..initial_state.get_nodes_len()).map(move |j| (i, j)))
        .filter(|edge| !initial_state.contains_edge(edge))
        .collect();
    // println!("{:?}",possible_edges);
    // let mut visited_states:HashSet<String> = HashSet::new();
    backtrack_with_hash(
        &initial_state,
        &possible_edges,
        &mut solutions,
        // &mut visited_states
    );
    println!("Soluciones encontradas {}", solutions.len());
}

fn backtrack_with_hash(
    current_state: &PartialPT,
    remaining_edges: &[(usize, usize)],
    solutions: &mut Vec<PartialPT>,
    // explored_hashes: &mut HashSet<String>,
) {
    // Generar hash de la configuración actual
    // let current_hash = current_state.hash_edges();

    // Si ya exploramos esta configuración, retornar
    // if !explored_hashes.insert(current_hash.clone()) {
    //     println!("{}: Ya visitado", current_hash);
    //     return;
    // }

    // Si cumple con alguna condición de solución (ejemplo: es una triangulación completa)
    if current_state.is_a_possible_ppt() {
        // println!("Hash {}", current_hash);
        current_state.draw_ascii(40, 40);
        solutions.push(current_state.clone());
        println!("");
        return;
    }

    // Explorar las posibles aristas restantes
    for (i, &edge) in remaining_edges.iter().enumerate() {
        let mut new_state = current_state.clone();
        if new_state.add_edge(edge).is_ok() {
            let next_edges: &[(usize, usize)] = &remaining_edges[i + 1..];
            backtrack_with_hash(
                &new_state, next_edges, solutions,
                // explored_hashes
            );
        }
    }
}
