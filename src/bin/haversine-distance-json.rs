use std::arch::x86_64::_mm256_sllv_epi32;
use std::convert::TryInto;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

use rayon::prelude::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    pairs: Vec<PointPair>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PointPair {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

// See also https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/trigonometry.html
fn HaversineOfDegrees(x0: f64, y0: f64, x1: f64, y1: f64, R: f64) -> f64 {
    let dY = (y1 - y0).to_radians();
    let dX = (x1 - x0).to_radians();
    let Y0 = (y0).to_radians();
    let Y1 = (y1).to_radians();

    let RootTerm = (dY / 2.0).sin().powi(2) + Y0.cos() * (Y1).cos() * (dX / 2.0).sin().powi(2);
    2.0 * R * RootTerm.sqrt().asin()
}

// https://blog.devgenius.io/reading-and-writing-a-json-file-in-rust-2731da8d6ad0
fn main() {
    let file_path = "data/distance/data-1000000.json";
    // let file_path = "data/distance/data-10.json";

    let start_time = Instant::now();

    let data = {
        let data_string = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str::<Data>(&data_string).unwrap()
    };


    // let data: Data = {
    //     let file = File::open(file_path).unwrap();
    //     let reader = BufReader::new(file);
    //     serde_json::from_reader(reader).unwrap()
    // };

    eprintln!("data = {:?}", data.pairs.len());
    let duration_read = start_time.elapsed();

    let earth_radius_kilometer = 6371.0_f64;
    let sum: f64 = data.pairs.into_iter().map(|pair| HaversineOfDegrees(pair.x0, pair.y0, pair.x1, pair.y1, earth_radius_kilometer)).sum();

    eprintln!("sum = {:?}", sum);

    let duration_complete = start_time.elapsed();
    let duration_calc = duration_complete - duration_read;
    eprintln!("duration_read = {:?}", duration_read);
    eprintln!("duration_calc = {:?}", duration_calc);
    eprintln!("duration_complete = {:?}", duration_complete);
}
