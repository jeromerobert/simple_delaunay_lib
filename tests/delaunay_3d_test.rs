#[cfg(test)]
mod delaunay_3d_test {
    use anyhow::Result;

    use rand::RngExt;
    use std::time::Instant;

    use simple_delaunay_lib::delaunay_3d::delaunay_struct_3d;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    fn create_and_check_delaunay(vec_pts: &Vec<[f64; 3]>) -> Result<()> {
        let now = Instant::now();
        let mut del_struct = delaunay_struct_3d::DelaunayStructure3D::new();
        del_struct.insert_vertices(vec_pts, true)?;
        let duration = now.elapsed();
        let milli = duration.as_millis();

        log::info!("Delaunay computed in {milli}ms");

        log::info!("Checking delaunay");
        assert!(del_struct.is_valid()?);
        Ok(())
    }

    #[test]
    fn test_random() -> Result<()> {
        let mut rng = rand::rng();

        let mut vec_pts: Vec<[f64; 3]> = Vec::new();
        for _ in 0..1000 {
            vec_pts.push(rng.random());
        }
        create_and_check_delaunay(&vec_pts)?;
        Ok(())
    }

    #[test]
    fn test_regular() -> Result<()> {
        let mut vec_pts: Vec<[f64; 3]> = Vec::new();
        let nb = 10;
        for ind in 0..(nb * nb * nb) {
            let ind1 = ind % nb;
            let ind2 = (ind / nb) % nb;
            let ind3 = ind / (nb * nb);

            let x = f64::from(ind1) / f64::from(nb);
            let y = f64::from(ind2) / f64::from(nb);
            let z = f64::from(ind3) / f64::from(nb);
            vec_pts.push([x, y, z]);
        }
        create_and_check_delaunay(&vec_pts)?;
        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let mut rng = rand::rng();

        let mut vec_pts: Vec<[f64; 3]> = Vec::new();
        for _ in 0..1000 {
            vec_pts.push(rng.random());
        }
        let now = Instant::now();
        let mut del_struct = delaunay_struct_3d::DelaunayStructure3D::new();
        del_struct.insert_vertices(&vec_pts, true)?;
        let duration = now.elapsed();
        let milli = duration.as_millis();

        log::info!("Delaunay computed in {milli}ms");

        log::info!("Checking delaunay");
        assert!(del_struct.is_valid()?);

        let mut vec_pts: Vec<[f64; 3]> = Vec::new();
        for _ in 0..1000 {
            vec_pts.push(rng.random());
        }
        let now = Instant::now();
        del_struct.insert_vertices(&vec_pts, true)?;
        let duration = now.elapsed();
        let milli = duration.as_millis();

        log::info!("Delaunay update computed in {milli}ms");

        log::info!("Checking delaunay");
        assert!(del_struct.is_valid()?);
        Ok(())
    }
}
