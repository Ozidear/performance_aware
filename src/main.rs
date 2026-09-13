use clap::Parser;
use rand::{RngExt, SeedableRng, rngs::StdRng};
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

const EARTH_RADIUS_KM: f64 = 6_372.8;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    count: usize,

    #[arg(long)]
    seed: u64,

    #[arg(long, default_value = "point_pairs.json")]
    json_output: PathBuf,

    #[arg(long, default_value = "haversine_answers.f64")]
    answer_output: PathBuf,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut rng = StdRng::seed_from_u64(args.seed);

    let mut json_writer = BufWriter::new(File::create(args.json_output)?);
    let mut serializer = serde_json::Serializer::new(&mut json_writer);
    let mut point_pairs = serializer.serialize_seq(Some(args.count))?;

    let mut answer_writer = BufWriter::new(File::create(args.answer_output)?);

    let mut haversine_sum = 0.0;

    for _ in 0..args.count {
        let point_pair = random_point_pair(&mut rng);
        point_pairs.serialize_element(&point_pair)?;

        let haversine_distance = haversine(&point_pair, EARTH_RADIUS_KM);
        answer_writer.write_all(&haversine_distance.to_le_bytes())?;
        haversine_sum += haversine_distance;
    }
    point_pairs.end()?;
    json_writer.flush()?;

    answer_writer.write_all(&haversine_sum.to_le_bytes())?;
    answer_writer.flush()?;

    let average_haversine = haversine_sum / args.count as f64;
    println!("Average Haversine distance: {average_haversine}");

    Ok(())
}
