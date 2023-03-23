use std::arch::x86_64::_mm256_sllv_epi32;
use std::convert::TryInto;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;
use std::{fs, io, mem};
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use std::io::Read;

use rayon::prelude::*;


struct F64Reader<R: io::BufRead> {
    inner: R,
}

impl<R: io::BufRead> F64Reader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner
        }
    }
}

impl<R: io::BufRead> Iterator for F64Reader<R> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buff: [u8; 8] = [0; 8];
        self.inner.read_exact(&mut buff).ok()?;
        Some(f64::from_le_bytes(buff))
    }
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
    let file_path = "data/distance/data-1000000.float64";
    // let file_path = "data/distance/data-10.json";

    let start_time = Instant::now();

    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    eprintln!("contents.len() = {:?}", contents.len());

    // https://stackoverflow.com/questions/49690459/converting-a-vecu32-to-vecu8-in-place-and-with-minimal-overhead
    let data = unsafe {
        let ratio = mem::size_of::<f64>() / mem::size_of::<u8>();

        let length = contents.len() /  ratio;
        let capacity = contents.capacity() / ratio;
        let ptr = contents.as_mut_ptr() as *mut f64;

        // Don't run the destructor for vec32
        mem::forget(contents);

        // Construct new Vec
        Vec::from_raw_parts(ptr, length, capacity)
    };

    eprintln!("data.len() = {:?}", data.len());



    let duration_read = start_time.elapsed();
    // let input = fs::File::open(file_path).unwrap();
    // let data: Vec<f64> = F64Reader::new(io::BufReader::new(input)).collect();
    // eprintln!("data = {:?}", data.len());

    let earth_radius_kilometer = 6371.0_f64;
    let sum: f64 = data.chunks_exact(4).map(|chunk| HaversineOfDegrees(chunk[0], chunk[1], chunk[2], chunk[3], earth_radius_kilometer)).sum();

    // let mut sum = 0.0;
    // for i in (0..data.len()).step_by(4) {
    //     sum += HaversineOfDegrees(data[i], data[i + 1], data[i + 2], data[i + 3], earth_radius_kilometer);
    // }

    eprintln!("sum = {:?}", sum);

    let duration_complete = start_time.elapsed();
    let duration_calc = duration_complete - duration_read;
    eprintln!("duration_read = {:?}", duration_read);
    eprintln!("duration_calc = {:?}", duration_calc);
    eprintln!("duration_complete = {:?}", duration_complete);
}
