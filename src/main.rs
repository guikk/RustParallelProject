use itertools::Itertools;
use rand::Rng;
// we can now use a `.log` method on parallel iterators
// to generate logs directly with rayon.

// # Example

// ```
// diam::display_svg(|| (0..10).into_par_iter().log("sum").sum::<u32>()).unwrap()
// ```

use diam::prelude::*;
use rayon::prelude::*;
    
/*
    Sequential Solutions
*/ 

pub fn sort_maximum_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut a = nums.to_owned();
    a.sort();
    (a[n-1] * a[n-2] * a[n-3]).max(a[n-1] * a[0] * a[1])
}

pub fn sp_maximum_product(nums: Vec<i32>) -> i32 {
    let mut min_1 = i32::MAX;
    let mut min_2 = i32::MAX;
    let mut max_1 = i32::MIN;
    let mut max_2 = i32::MIN;
    let mut max_3 = i32::MIN;

    for num in nums {
        if num < min_1 {
            min_2 = min_1;
            min_1 = num;
        } else if num < min_2 {
            min_2 = num;
        }
        if num > max_1 {
            max_3 = max_2;
            max_2 = max_1;
            max_1 = num;
        } else if num > max_2 {
            max_3 = max_2;
            max_2 = num;
        } else if num > max_3 {
            max_3 = num;
        }
    }
    (max_1 * max_2 * max_3).max(max_1 * min_1 * min_2)
}

/*
    Parallel Solutions
*/ 

pub fn par_sort_maximum_product(nums: Vec<i32>) -> i32 {
    unimplemented!()
    // parallel sort
}

pub fn par_sp_maximum_product(nums: Vec<i32>) -> i32 {
    unimplemented!()
    // divide array
    // store local 3 max and 2 min
    // merge
    // return max multiplication
}

const VSIZE: usize = 10_000;
const INTMIN: i32 = -1000;
const INTMAX: i32 = 1000;

fn main() {
    let mut rng = rand::thread_rng();
    let nums: Vec<i32> = (0..VSIZE).map(|_| rng.gen_range(INTMIN..INTMAX)).collect();

    let start = std::time::Instant::now();
    let m1 = sort_maximum_product(nums.to_owned());
    eprintln!(
        "{:?}\tSequential with Sorting",
        start.elapsed()
    );

    let start = std::time::Instant::now();
    let m2 = sp_maximum_product(nums.to_owned());
    eprintln!(
        "{:?}\tSequential with Single Pass took",
        start.elapsed()
    );

    // let start = std::time::Instant::now();
    // let m3 = par_sort_maximum_product(nums.to_owned());
    // eprintln!(
    //     "{:?}\tParallel with Sorting",
    //     start.elapsed()
    // );

    // let start = std::time::Instant::now();
    // let m4 = par_sp_maximum_product(nums.to_owned());
    // eprintln!(
    //     "{:?}\tParallel with Single Pass",
    //     start.elapsed()
    // );

    assert_eq!(m1, m2);
    // assert_eq!(m2, m3);
    // assert_eq!(m3, m4);
}
