use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    count: u32,

    #[arg(long)]
    seed: u64,

    #[arg(long, default_value = "point_pairs.json")]
    output: PathBuf,
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

//TODO:
// cluster/uniform
// haversine avg
// binary output

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut rng = StdRng::seed_from_u64(args.seed);

    let point_pairs: Vec<PointPair> = (0..args.count)
        .map(|_| random_point_pair(&mut rng))
        .collect();

    let output_file = File::create(args.output)?;
    let mut writer = BufWriter::new(output_file);
    serde_json::to_writer(&mut writer, &point_pairs)?;
    writer.flush()?;

    Ok(())
}
