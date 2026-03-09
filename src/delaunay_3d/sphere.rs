use std::cell::RefCell;

use glam::DVec3;
use robust::Coord3D;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: DVec3,
    pub radius_sqr: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: DVec3::default(),
            radius_sqr: f64::NAN,
        }
    }
}

impl Sphere {
    pub const fn undefined(&self) -> bool {
        !self.radius_sqr.is_finite()
    }

    pub fn degenerated(&self) -> bool {
        self.radius_sqr < 0.
    }

    /// Checks if a point is inside or on the surface of the sphere.
    ///
    /// Result it positive if in sphere, negative if outside
    pub fn contains_point(&self, point: &[f64; 3]) -> f64 {
        let p = DVec3::from_array(*point);
        1. - p.distance_squared(self.center) / self.radius_sqr
    }

    /// Creates a circumsphere from 4 points defining a tetrahedron.
    /// Handles degenerate cases (triangle or segment).
    pub fn from_tetra(points: &[[f64; 3]; 4]) -> Self {
        let [a, b, c, d] = points.map(DVec3::from_array);

        // Relative vectors from point 'a'
        let ba = b - a;
        let ca = c - a;
        let da = d - a;
        let lba = ba.length_squared();
        let lca = ca.length_squared();
        let lda = da.length_squared();

        // Calculate the denominator (6 * volume of tetrahedron)
        // Using the triple product: (ba x ca) · da
        let cross_ba_ca = ba.cross(ca);
        let denom = 2.0 * cross_ba_ca.dot(da);

        // If denom is near zero, the 4 points are coplanar (or worse)
        if denom * denom < 1e-12 * lba * lca * lda {
            return Self {
                center: DVec3::default(),
                radius_sqr: -1.,
            };
        }

        // Formula for circumcenter relative to point 'a'
        let offset = (lda * cross_ba_ca + lca * da.cross(ba) + lba * ca.cross(da)) / denom;
        let center = a + offset;
        let radius_sqr = center.distance_squared(a);

        Self { center, radius_sqr }
    }
}

#[derive(Default)]
pub struct SphereCache {
    /// Uses `RefCell` to provide interior mutability,
    /// allowing the cache to be updated within immutable methods.
    data: RefCell<Vec<Sphere>>,
    stats: RefCell<(usize, usize, usize, usize)>,
}

impl Drop for SphereCache {
    fn drop(&mut self) {
        let stat = self.stats.borrow();
        println!("Num sphere cache hit: {}", stat.0);
        println!("Num sphere cache fail: {}", stat.1);
        println!("Num sphere cache degenerated: {}", stat.2);
        println!("Num sphere cache closed: {}", stat.3);
    }
}
impl SphereCache {
    /// Resets the cached sphere for a specific tetrahedron.
    pub fn invalidate(&self, ind_tetra: usize) {
        let mut data = self.data.borrow_mut();
        if ind_tetra < data.len() {
            data[ind_tetra] = Sphere::default();
        }
    }
    pub fn insphere(&self, tetra: &[[f64; 3]; 4], point: &[f64; 3], ind_tetra: usize) -> f64 {
        let mut data = self.data.borrow_mut();
        if ind_tetra >= data.len() {
            // Ensure the cache is large enough
            data.resize(ind_tetra + 1, Sphere::default());
        }
        let s = &mut data[ind_tetra];
        if s.undefined() {
            *s = Sphere::from_tetra(tetra);
        }
        if !s.degenerated() {
            // Fast-path: check if the cached sphere gives a definitive answer
            let ds = s.contains_point(point);
            if ds.abs() > 1e-8 {
                self.stats.borrow_mut().0 += 1;
                return ds;
            } else {
                self.stats.borrow_mut().3 += 1;
            }
        } else {
            self.stats.borrow_mut().2 += 1;
        }
        // Fallback to robust predicates for degenerate cases or numerical uncertainties
        let c = |p: &[f64; 3]| Coord3D {
            x: p[0],
            y: p[1],
            z: p[2],
        };
        self.stats.borrow_mut().1 += 1;
        robust::insphere(
            c(&tetra[0]),
            c(&tetra[1]),
            c(&tetra[2]),
            c(&tetra[3]),
            c(point),
        )
    }
}
