use std::time::{Duration, Instant};

const COMPLETE_N: i64 = 100_000_000;
const N: i64 = 100_000;
const N2: i64 = COMPLETE_N / N;


fn create_vec() -> Vec<f64> {
    (0..N).map(|i| i as f64).collect()
}

fn sum_vec_index(vec: &Vec<f64>)-> f64 {
    let mut sum = 0.0;
    for i in 0..vec.len() {
        sum += vec[i];
    }
    sum
}

fn sum_vec(vec: &Vec<f64>)-> f64 {
    let mut sum = 0.0;
    for x in vec{
        sum += x;
    }
    sum
}

fn sum_vec_built_in(vec: &Vec<f64>)-> f64 {
    vec.iter().sum()
}

fn sum_vec_index_5(vec: &Vec<f64>)-> f64 {
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    let mut sum3 = 0.0;
    let mut sum4 = 0.0;
    let mut sum5 = 0.0;

    for i in (0..vec.len()).step_by(5) {
        sum1 += vec[i];
        sum2 += vec[i+1];
        sum3 += vec[i+2];
        sum4 += vec[i+3];
        sum5 += vec[i+4];
    }

    sum1+sum2+sum3+sum4+sum5
}


fn main() {
    let vec = create_vec();

    let mut min_time = Duration::from_secs(10000);
    for _i in 0..6 {
        let start = Instant::now();
        let mut complete_sun = 0.0;

        for _j in 0..N2 {
            complete_sun += sum_vec_index_5(&vec);
        }

        let duration = start.elapsed();
        min_time = min_time.min(duration);

        eprintln!("complete_sun = {:?} {:?}", complete_sun, duration);

    }

    eprintln!("min_time = {:?}", min_time);
    let ops_per_microsecond =  COMPLETE_N as f64 / (min_time.as_micros() as f64 );
    eprintln!("ops_per_microsecond = {:?}", ops_per_microsecond);
}
