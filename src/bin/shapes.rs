use std::time::{Duration, Instant};
use std::convert::TryInto;
use std::f64::consts::PI;

const N: i64 = 1_000_000;


enum Shape {
    Square(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
    Circle(f64),
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Square(side) => { side * side }
            Shape::Rectangle(width, height) => { width * height }
            Shape::Triangle(base, height) => { 0.5 * base * height }
            Shape::Circle(radius) => { PI * radius * radius }
        }
    }
}


fn create_vec() -> Vec<Shape> {
    (0..N).map(|i| match i%4 {
        0 => Shape::Square(1.123),
        1 => Shape::Rectangle(1.123, 2.234),
        2 => Shape::Triangle(1.123, 2.234),
        3 => Shape::Circle(1.123),
        _ => {unimplemented!()}
    } ).collect()
}

fn sum_vec_index(vec: &Vec<Shape>) -> f64 {
    let mut sum = 0.0;
    for i in 0..vec.len() {
        sum += vec[i].area();
    }
    sum
}

fn sum_vec(vec: &Vec<Shape>) -> f64 {
    let mut sum = 0.0;
    for x in vec {
        sum += x.area();
    }
    sum
}

fn sum_vec_built_in(vec: &Vec<Shape>) -> f64 {
    vec.iter().map(|it| it.area()).sum()
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
