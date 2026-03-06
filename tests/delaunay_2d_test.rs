#[cfg(test)]
mod delaunay_2d_test {
    use anyhow::Result;

    use rand::RngExt;
    use simple_delaunay_lib::delaunay_2d::delaunay_struct_2d;
    use std::time::Instant;

    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }

    fn create_and_check_delaunay(vec_pts: &Vec<[f64; 2]>) -> Result<()> {
        let now = Instant::now();
        let mut del_struct = delaunay_struct_2d::DelaunayStructure2D::new();
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

        let mut vec_pts = Vec::new();
        for _ in 0..1000 {
            vec_pts.push(rng.random());
        }
        create_and_check_delaunay(&vec_pts)?;
        Ok(())
    }

    #[test]
    fn test_regular() -> Result<()> {
        let mut vec_pts = Vec::new();
        for ind in 0..1000 {
            let ind1 = ind % 100;
            let ind2 = ind / 100;

            let x = f64::from(ind1) / 100.;
            let y = f64::from(ind2) / 100.;
            vec_pts.push([x, y]);
        }
        create_and_check_delaunay(&vec_pts)?;
        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let mut rng = rand::rng();

        let mut vec_pts = Vec::new();
        for _ in 0..1000 {
            vec_pts.push(rng.random());
        }
        let now = Instant::now();
        let mut del_struct = delaunay_struct_2d::DelaunayStructure2D::new();
        del_struct.insert_vertices(&vec_pts, true)?;
        let duration = now.elapsed();
        let milli = duration.as_millis();

        log::info!("Delaunay computed in {milli}ms");

        log::info!("Checking delaunay");
        assert!(del_struct.is_valid()?);

        let mut vec_pts: Vec<[f64; 2]> = Vec::new();
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
