use anyhow::Result;
use rand::RngExt;
use std::time::Instant;

use simple_delaunay_lib::delaunay_2d::delaunay_struct_2d::DelaunayStructure2D;

fn generate_random_vertices(nb_vert: usize) -> Vec<[f64; 2]> {
    let mut rng = rand::rng();

    let mut vec_pts: Vec<[f64; 2]> = Vec::new();
    for _ in 0..nb_vert {
        let (x, y): (f64, f64) = rng.random();
        vec_pts.push([x, y]);
    }
    vec_pts
}

fn main() -> Result<()> {
    env_logger::init();

    let nb_vert_array = [10, 100, 1000, 10000, 100000, 1000000];

    for nb_vert in nb_vert_array {
        let vec_pts = generate_random_vertices(nb_vert);

        let now = Instant::now();
        let mut del_struct = DelaunayStructure2D::new();
        del_struct.insert_vertices(&vec_pts, true)?;
        let duration = now.elapsed();
        let milli = duration.as_millis();

        println!("{nb_vert} vertices: {milli}ms");
    }

    Ok(())
}
