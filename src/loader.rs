use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub struct PointSet {
    points: Vec<(u32, u32)>, // Usa u8 o u16 según el archivo
}

impl PointSet {
    fn from_bytes(bytes: &[u8], point_count: usize, byte_size: usize) -> Self {
        let mut points = Vec::new();

        for i in 0..point_count {
            let x = match byte_size {
                1 => bytes[i * 2] as u32,
                2 => u16::from_le_bytes([bytes[i * 2], bytes[i * 2 + 1]]) as u32,
                _ => panic!("Formato no soportado"),
            };
            let y = match byte_size {
                1 => bytes[i * 2 + 1] as u32,
                2 => u16::from_le_bytes([bytes[i * 2 + 2], bytes[i * 2 + 3]]) as u32,
                _ => panic!("Formato no soportado"),
            };
            points.push((x, y));
        }
        
        PointSet { points }
    }
}

pub fn load_file<P: AsRef<Path>>(path: P, point_count: usize, byte_size: usize) -> io::Result<Vec<PointSet>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut sets = Vec::new();
    let mut buffer = vec![0; point_count * 2 * byte_size];

    while reader.read_exact(&mut buffer).is_ok() {
        let point_set = PointSet::from_bytes(&buffer, point_count, byte_size);
        sets.push(point_set);
    }

    Ok(sets)
}