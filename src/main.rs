use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

const EARTH_RADIUS_KM: f64 = 6_372.8;

#[derive(Parser)]
struct Args {
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
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

fn haversine(point_pair: &PointPair, earth_radius: f64) -> f64 {
    let lat1 = point_pair.y0.to_radians();
    let lat2 = point_pair.y1.to_radians();
    let delta_lat = lat2 - lat1;
    let delta_lon = (point_pair.x1 - point_pair.x0).to_radians();

    let sin_delta_lat = (delta_lat / 2.0).sin();
    let sin_delta_lon = (delta_lon / 2.0).sin();
    let a = sin_delta_lat * sin_delta_lat + lat1.cos() * lat2.cos() * sin_delta_lon * sin_delta_lon;
    let c = 2.0 * a.sqrt().asin();

    earth_radius * c
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
// binary output

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut rng = StdRng::seed_from_u64(args.seed);

    let point_pairs: Vec<PointPair> = (0..args.count)
        .map(|_| random_point_pair(&mut rng))
        .collect();

    let average_haversine = point_pairs
        .iter()
        .map(|point_pair| haversine(point_pair, EARTH_RADIUS_KM))
        .sum::<f64>()
        / point_pairs.len() as f64;
    println!("Average Haversine distance: {average_haversine}");

    let output_file = File::create(args.output)?;
    let mut writer = BufWriter::new(output_file);
    serde_json::to_writer(&mut writer, &point_pairs)?;
    writer.flush()?;

    Ok(())
}
