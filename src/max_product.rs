use crate::merge_sort::merge_sort;
use diam::join;

/*
    Sequential Solutions
*/ 

pub fn sort_maximum_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut a = nums.to_owned();
    a.sort();
    (a[n-1] * a[n-2] * a[n-3]).max(a[n-1] * a[0] * a[1])
}

struct Limits {
    min1: i32,
    min2: i32,
    max1: i32,
    max2: i32,
    max3: i32,
}

impl Limits {
    fn new() -> Self {
        Limits {
            min1: i32::MAX,
            min2: i32::MAX,
            max1: i32::MIN,
            max2: i32::MIN,
            max3: i32::MIN,
        }
    }

    fn update_from_vec(&mut self, nums: Vec<i32>) {
        for num in nums {
            if num < self.min1 {
                self.min2 = self.min1;
                self.min1 = num;
            } else if num < self.min2 {
                self.min2 = num;
            }
            if num > self.max1 {
                self.max3 = self.max2;
                self.max2 = self.max1;
                self.max1 = num;
            } else if num > self.max2 {
                self.max3 = self.max2;
                self.max2 = num;
            } else if num > self.max3 {
                self.max3 = num;
            }
        }
    }

    fn max_product(&self) -> i32 {
        (self.max1 * self.max2 * self.max3).max(self.max1 * self.min1 * self.min2)
    }

    fn merge(&mut self, other: Self) {
        let v: Vec<i32> = vec![other.max1, other.max2, other.max3, other.min1, other.min2];

        self.update_from_vec(v);
    }
}

pub fn sp_maximum_product(nums: Vec<i32>) -> i32 {
    let mut l = Limits::new();
    
    l.update_from_vec(nums);

    l.max_product()
}

/*
    Parallel Solutions
*/

pub fn par_sort_maximum_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut a = nums.to_owned();
    // parallel sort
    merge_sort(&mut a);
    (a[n-1] * a[n-2] * a[n-3]).max(a[n-1] * a[0] * a[1])
}

fn par_find_array_limits(nums: Vec<i32>, levels: usize) -> Limits {
    if levels == 0 {
        let mut l = Limits::new();
        l.update_from_vec(nums);
        return l;
    }
    
    let (left, right) = nums.split_at(nums.len() / 2);

    let (mut l1, l2) = join(
        || par_find_array_limits(left.to_vec(), levels - 1),
        || par_find_array_limits(right.to_vec(), levels - 1),
    );

    l1.merge(l2);

    l1
}

pub fn par_sp_maximum_product(nums: Vec<i32>) -> i32 {
    let levels = 4;

    par_find_array_limits(nums, levels).max_product()
}