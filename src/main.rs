use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    count: u32,

    #[arg(long)]
    seed: u64,
}

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
    let args = Args::parse();
    let mut rng = StdRng::seed_from_u64(args.seed);

    let point_pairs: Vec<PointPair> = (0..args.count)
        .map(|_| random_point_pair(&mut rng))
        .collect();

    println!("{point_pairs:?}");
}
