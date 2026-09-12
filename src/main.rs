use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PointPair {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

fn random_point_pair(rng: &mut impl RngExt) -> PointPair {
    PointPair {
        x0: rng.random_range(-180.0..180.0),
        y0: rng.random_range(-90.0..90.0),
        x1: rng.random_range(-180.0..180.0),
        y1: rng.random_range(-90.0..90.0),
    }
}

fn main() {
    let mut rng = StdRng::seed_from_u64(1);

    let point_pairs: [PointPair; 1_000] = std::array::from_fn(|_| random_point_pair(&mut rng));

    println!("{point_pairs:?}");
}
