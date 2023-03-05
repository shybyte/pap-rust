use std::time::{Duration, Instant};
use std::convert::TryInto;
use std::f64::consts::PI;

const N: i64 = 1_000_000;


struct Shape {
    shapeType: usize,
    width: f64,
    height: f64,
}

const FACTOR_TABLE: &'static [f64] = &[1.0, 1.0, 0.5, PI];


fn create_vec() -> Vec<Shape> {
    (0..N).map(|i| Shape { shapeType: (i % 4) as usize, width: 1.123, height: 2.234 }).collect()
}

fn area(shape: &Shape) -> f64 {
    FACTOR_TABLE[shape.shapeType] * shape.width * shape.height
}

fn sum_vec_index(vec: &Vec<Shape>) -> f64 {
    let mut sum = 0.0;
    for i in 0..vec.len() {
        sum += area(&vec[i]);
    }
    sum
}

fn sum_vec(vec: &Vec<Shape>) -> f64 {
    let mut sum = 0.0;
    for x in vec {
        sum += area(x);
    }
    sum
}

fn sum_vec_built_in(vec: &Vec<Shape>) -> f64 {
    vec.iter().map(|it| area(it)).sum()
}


fn main() {
    let vec = create_vec();

    let mut min_time = Duration::from_secs(10000);
    for _i in 0..20 {
        let start = Instant::now();
        let mut complete_sun = 0.0;

        complete_sun += sum_vec_built_in(&vec);

        let duration = start.elapsed();
        min_time = min_time.min(duration);

        eprintln!("complete_sun = {:?} {:?}", complete_sun, duration);
    }

    eprintln!("min_time = {:?}", min_time);
}
