use std::io;
mod loader;
fn main() -> io::Result<()> {
    let filename = "otypes06.b08";
    let point_count = 6;
    let byte_size = 1;
    
    let point_sets = loader::load_file(filename, point_count, byte_size)?;
    for (i, point_set) in point_sets.iter().enumerate() {
        println!("Set {}: {:?}", i, point_set);
    }
    Ok(())
}
