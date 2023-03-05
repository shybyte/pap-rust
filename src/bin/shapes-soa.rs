use std::time::{Duration, Instant};
use std::f64::consts::PI;

const N: usize = 1_000_000;


struct ShapeArray {
    shapeType: Vec<u8>,
    widthHeight: Vec<f64>,
}

const FACTOR_TABLE: &'static [f64] = &[1.0, 1.0, 0.5, PI];


fn create_vec() -> ShapeArray {
    let mut shapeType: Vec<u8> = vec![0; N];
    let mut widthHeight: Vec<f64> = vec![0.0; 2 * N];
    for i in 0..N {
        shapeType[i] = (i % 4) as u8;
        widthHeight[i * 2] = 1.234;
        widthHeight[i * 2 + 1] = 2.345;
    }
    return ShapeArray { shapeType, widthHeight };
}

fn sum_vec_index(shapeArray: &ShapeArray) -> f64 {
    let mut sum = 0.0;
    for i in 0..N {
        sum += FACTOR_TABLE[shapeArray.shapeType[i] as usize] * shapeArray.widthHeight[i * 2] * shapeArray.widthHeight[i * 2 + 1];
    }
    sum
}


fn main() {
    let shape_array = create_vec();

    let mut min_time = Duration::from_secs(10000);
    for _i in 0..20 {
        let start = Instant::now();
        let mut complete_sun = 0.0;

        complete_sun += sum_vec_index(&shape_array);

        let duration = start.elapsed();
        min_time = min_time.min(duration);

        eprintln!("complete_sun = {:?} {:?}", complete_sun, duration);
    }

    eprintln!("min_time = {:?}", min_time);
}
