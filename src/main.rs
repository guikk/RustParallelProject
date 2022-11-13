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
    make_svg();
}
