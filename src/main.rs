use itertools::Itertools;
// we can now use a `.log` method on parallel iterators
// to generate logs directly with rayon.

// # Example

// ```
// diam::display_svg(|| (0..10).into_par_iter().log("sum").sum::<u32>()).unwrap()
// ```

use diam::prelude::*;
use rayon::prelude::*;
    
pub fn sort_maximum_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut a = nums.to_owned();
    a.sort();
    return cmp::max(a[n-1] * a[0] * a[1], a[n-1] * a[n-2] * a[n-3]);
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

// parallel

// to be implemented
pub fn par_maximum_product(nums: Vec<i32>) -> i32 {
    // store 3 max and 2 min
    // for each update table
    // return max multiplication
    
    // parallel comparison would be the same as sequential
    
    // parallel sort?
    
    return 0;
}

fn main() {
    // let start = std::time::Instant::now();
    // let v = random_points(POINTS_NUMBER);
    // eprintln!(
    //     "generating {} points in sequential took {:?}",
    //     POINTS_NUMBER,
    //     start.elapsed()
    // );
    // let start = std::time::Instant::now();
    // let par_v = par_random_points(POINTS_NUMBER);
    // eprintln!(
    //     "generating {} points in parallel took {:?}",
    //     POINTS_NUMBER,
    //     start.elapsed()
    // );
    // let seq_nearest_points = brute_force_nearest_points(&v);
    // diam::svg("nearest_points.svg", || {
    //     let par_nearest_points = par_brute_force_nearest_points(&v);
    //     let (p1, p2) = seq_nearest_points.unwrap();
    //     let (p3, p4) = par_nearest_points.unwrap();
    //     assert_eq!(p1.squared_distance_to(&p2), p3.squared_distance_to(&p4))
    // })
    // .expect("failed generating svg file");
    // let t1 = large_enough_triangle(&v, 10_000);
    // let t2 = par_large_enough_triangle(&v, 10_000);

    // assert_eq!(t1, t2);
    // assert!(perimeter(t1.unwrap()) > 10_000);
}
