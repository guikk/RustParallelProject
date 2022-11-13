use rand::Rng;
// we can now use a `.log` method on parallel iterators
// to generate logs directly with rayon.

// # Example

// ```
// diam::display_svg(|| (0..10).into_par_iter().log("sum").sum::<u32>()).unwrap()
// ```

// use diam::prelude::*;
// use rayon::prelude::*;
mod max_product;
mod merge_sort;

const VSIZE: usize = 10_000_000;
const INTMIN: i32 = -1000;
const INTMAX: i32 = 1000;

fn benchmark() {
    let mut rng = rand::thread_rng();
    const N_SAMPLES: usize = 100;
    println!("size,s1,s2,p1,p2");

    for size in vec![1_000_000, 5_000_000, 10_000_000, 15_000_000, 25_000_000] {

        let s1: f64 = 0.0; //std::iter::repeat_with(|| {
        //     let nums: Vec<i32> = (0..size).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();
        //     let start = std::time::Instant::now();
        //     let m4 = max_product::sort_maximum_product(nums.to_owned());
        //     start.elapsed().as_secs_f64() * 1000.0
        // }).take(N_SAMPLES).sum::<f64>() / N_SAMPLES as f64;

        let s2: f64 = std::iter::repeat_with(|| {
            let nums: Vec<i32> = (0..size).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();
            let start = std::time::Instant::now();
            let m4 = max_product::sp_maximum_product(nums.to_owned());
            start.elapsed().as_secs_f64() * 1000.0
        }).take(N_SAMPLES).sum::<f64>() / N_SAMPLES as f64;

        let p1: f64 = 0.0; //std::iter::repeat_with(|| {
        //     let nums: Vec<i32> = (0..size).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();
        //     let start = std::time::Instant::now();
        //     let m4 = max_product::par_sort_maximum_product(nums.to_owned());
        //     start.elapsed().as_secs_f64() * 1000.0
        // }).take(N_SAMPLES).sum::<f64>() / N_SAMPLES as f64;

        let p2: f64 = std::iter::repeat_with(|| {
            let nums: Vec<i32> = (0..size).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();
            let start = std::time::Instant::now();
            let m4 = max_product::par_local_limits_maximum_product(nums.to_owned());
            start.elapsed().as_secs_f64() * 1000.0
        }).take(N_SAMPLES).sum::<f64>() / N_SAMPLES as f64;

        println!("{:.4},{:.4},{:.4},{:.4},{:.4}", size, s1, s2, p1, p2);
    }

}

fn make_svg() {
    let mut rng = rand::thread_rng();
    let nums: Vec<i32> = (0..VSIZE).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();

    diam::svg("graphs/maximum_product.svg", || {
        max_product::par_local_limits_maximum_product(nums);
    }).unwrap();
}

fn single_comparison() {
    let mut rng = rand::thread_rng();
    let nums: Vec<i32> = (0..VSIZE).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();

    let start = std::time::Instant::now();
    let m1 = max_product::sort_maximum_product(nums.to_owned());
    eprintln!(
        "{:>25} - {:?}",
        "Sequential Sorting",
        start.elapsed()
    );

    let start = std::time::Instant::now();
    let m2 = max_product::sp_maximum_product(nums.to_owned());
    eprintln!(
        "{:>25} - {:?}",
        "Sequential Single Pass",
        start.elapsed()
    );

    let start = std::time::Instant::now();
    let m3 = max_product::par_sort_maximum_product(nums.to_owned());
    eprintln!(
        "{:>25} - {:?}",
        "Parallel Sorting",
        start.elapsed()
    );

    let start = std::time::Instant::now();
    let m4 = max_product::par_local_limits_maximum_product(nums.to_owned());
    eprintln!(
        "{:>25} - {:?}",
        "Parallel Local Limits",
        start.elapsed(),
    );

    assert_eq!(m1, m2);
    assert_eq!(m2, m3);
    assert_eq!(m3, m4);
} 

fn main() {
    single_comparison();
    //make_svg();
    //benchmark();
}
