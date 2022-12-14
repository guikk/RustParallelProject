# Maximum Product of Three Numbers
Rust project for the Parallel Systems course at Grenoble INP - Ensimag.

It consists of taking a known programming problem, from Leetcode for example, and solving it concurrently.

## Problem
Given an integer array `nums`, find three numbers whose product is maximum and return the maximum product.

Example 1:

~~~
Input: nums = [1,2,3]
Output: 6
~~~
Example 2:

~~~
Input: nums = [1,2,3,4]
Output: 24
~~~
Example 3:

~~~
Input: nums = [-1,-2,-3]
Output: -6
~~~
 

Constraints:

- 3 <= nums.length <= 104
- -1000 <= nums[i] <= 1000

## Sequential Solutions
In order to think about a parallel solution, a good way to start is by finding a sequential solution and then parallelizing it.

### Intuition
This problem can be seen as an extension of the maximum product of two numbers. We can find the greatest element of the array and then multiply it by the maximum product of two numbers in the rest of the array. The exception to this simplification is when the greatest element is negative, then we will look for the pair with the smallest product.

Following, the maximum product of two numbers is achievable by multiplying either the two greatest elements or the two smallest elements of the array (in the case where the smallest elements are negative).

Finally, respecting the previous contraints and edge cases, the proposed solution is expressed as the following expression:

~~~
S = max(G1 * G2 * G3, G1 * S1 * S2)
~~~
where `Gn` is the nth greatest number on the array and `Sn` is the nth smallest.

### Sorting

The first obvious way to find the 3 greatest and 2 smallest elements of the array is by sorting and reading by index.

`O(nlogn)` Time complexity
`O(n)` Space complexity

Complexity based on the Rust implementation of [`sort`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort) for slices

### Single-pass

The better optimized alternative is allocating memory for 5 integers (the 3 greatest and 2 smallest elements), and updating those values for each element of the array in a single pass.

`O(n)` Time complexity
`O(1)` Space complexity

## Parallelization
### Parallel Single-pass with Shared Limits
By sharing the the integer variables of greatest and smallest elements, the array elements could be analysed in parallel.

The problem with this solution is that if we use a lock to synchronize the access to the variables and avoid a race condition, elements would be analyzed one at a time, turning the sollution the same as the sequential single-pass.

I couldn't find a way to avoid this from happening, so I decided to invest in other solutions.

### Parallel MergeSort
As seen in class, there is a way of parallelizing the MergeSort algorithm. We can use it to sort the array, and possibly get a better performance.

This solution could outperform the sequential sort by a fair bit, but stood no chance against the `O(n)` solution as expected.

### Local greatest and smallest elements
Using the idea of divide and conquer, we can separately find the greatest and smallest elements of subarrays, then merge the resuls to find the global greatest and smallest elements.

Relying on the Master Theorem, the complexity of this solution is also `O(n)`, as both the division factor and the number of calls are equal to 2, while the time to create the subproblems and combine their results is `O(1)`
> $C(n) = 2*C(\frac{n}{2}) + O(1)$
>
> **Case 2c** $\to O(n^{\log_2 2}) = O(n)$

*Algorithm thread usage example*

![Thread usage example](graphs/maximum_product.svg)

## Result Comparison

*Comparison of Sorting Solutions*

![](graphs/comparison_sorting.png)

*Comparison of Single-pass Solutions*

![](graphs/comparison_single_pass.png)
