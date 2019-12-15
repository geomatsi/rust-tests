fn main() {
    println!("run tests: cargo test");
}

// Example #1
// Input array assumptions:
// - array of ints sorted in ascending order
// - each input would have exactly one solution
// - using the same element twice is not allowed
//
// Find two elements such that they add up to a specific target number.
// Note that returned indexes (both index1 and index2) should be
// one-based, not zero-based.

#[allow(dead_code)]
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut j = (numbers.len() - 1) as usize;
    let mut i = 0 as usize;

    while i < j {
        let sum = numbers[i] + numbers[j];

        if sum < target {
            i += 1;
        } else if sum > target {
            j -= 1;
        } else {
            break;
        }
    }

    return vec![(i + 1) as i32, (j + 1) as i32];
}

#[test]
fn test_two_sum() {
    assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![2, 4], two_sum(vec![1, 2, 3, 4, 6], 6));
    assert_eq!(vec![2, 3], two_sum(vec![1, 10, 21, 22, 31, 40], 31));
}

// Example #2
// Given a binary array (contains only 0s and 1s), find
// the maximum number of consecutive 1s in this array.

#[allow(dead_code)]
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut m: i32 = 0;
    let mut k: i32 = -1;

    for (i, e) in nums.iter().enumerate() {
        if *e == 0 {
            k = i as i32;
        }

        let m1: i32 = (i as i32) - k;

        if m1 > m {
            m = m1;
        }
    }

    m
}

#[test]
fn test_find_max_cons_ones() {
    assert_eq!(0, find_max_consecutive_ones(vec![]));
    assert_eq!(0, find_max_consecutive_ones(vec![0]));
    assert_eq!(1, find_max_consecutive_ones(vec![1]));
    assert_eq!(1, find_max_consecutive_ones(vec![1, 0]));
    assert_eq!(2, find_max_consecutive_ones(vec![1, 1]));
    assert_eq!(3, find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]));
    assert_eq!(3, find_max_consecutive_ones(vec![1, 1, 1, 0, 1, 1]));
    assert_eq!(5, find_max_consecutive_ones(vec![0, 1, 1, 1, 1, 1, 0]));
}

// Example #3
// Given a matrix of M x N elements (M rows, N columns),
// return all elements of the matrix in diagonal order
// as shown in the below image.
//
// Input:
// [
//  [ 1, 2, 3 ],
//  [ 4, 5, 6 ],
//  [ 7, 8, 9 ]
// ]
//
// Output:  [1,2,4,7,5,3,6,8,9]

#[allow(dead_code)]
fn find_diag_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 {
        return vec![];
    }

    let m = matrix[0].len();
    if m == 0 {
        return vec![];
    }

    let mut res: Vec<i32> = Vec::with_capacity(n * m);

    for k in 0..(n + m - 1) {
        for i in 0..=k {
            if k % 2 == 0 {
                if i < m && (k - i) < n {
                    res.push(matrix[k - i][i]);
                }
            } else if i < n && (k - i) < m {
                res.push(matrix[i][k - i]);
            }
        }
    }

    res
}

#[test]
fn test_diag_order() {
    assert_eq!(find_diag_order(vec![vec![]]), vec![]);
    assert_eq!(find_diag_order(vec![vec![], vec![]]), vec![]);
    assert_eq!(find_diag_order(vec![vec![1]]), vec![1]);
    assert_eq!(find_diag_order(vec![vec![1, 2]]), vec![1, 2]);
    assert_eq!(find_diag_order(vec![vec![1, 2, 3, 4]]), vec![1, 2, 3, 4]);
    assert_eq!(
        find_diag_order(vec![vec![1], vec![2], vec![3], vec![4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_diag_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        find_diag_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
}

// Example #4
// Given an array of n positive integers and a positive integer s, find
// the minimal length of a contiguous subarray of which the sum ≥ s.
// If there isn't one, return 0 instead.

#[allow(dead_code)]
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let mut xn: usize = nums.len() + 1;
    let mut xs: i32 = 0;
    let mut k: usize = 0;
    let mut m: usize = 0;

    loop {
        if xs < s {
            if m >= nums.len() {
                break;
            }
            xs += nums[m];
            m += 1;
        }

        if xs >= s {
            if (m - k) < xn {
                xn = m - k;
            }

            xs -= nums[k];
            k += 1;
        }
    }

    if xn > nums.len() {
        return 0;
    }

    xn as i32
}

#[test]
fn test_min_sub_array_len() {
    assert_eq!(0, min_sub_array_len(1, vec![]));
    assert_eq!(0, min_sub_array_len(1, vec![0]));
    assert_eq!(1, min_sub_array_len(1, vec![1]));
    assert_eq!(1, min_sub_array_len(1, vec![1, 0, 1, 0]));
    assert_eq!(3, min_sub_array_len(2, vec![1, 0, 1, 0]));
    assert_eq!(4, min_sub_array_len(4, vec![1, 1, 1, 1]));
    assert_eq!(3, min_sub_array_len(4, vec![2, 1, 1, 0, 0, 1]));
    assert_eq!(2, min_sub_array_len(4, vec![2, 1, 1, 0, 0, 1, 3]));
    assert_eq!(2, min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
}

// Example #5
// Given an array of 2n integers, your task is to group these integers into n pairs of integer,
// say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n
// as large as possible.

#[allow(dead_code)]
pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut res: Vec<i32> = nums.to_vec();
    let mut sum = 0;
    let mut ind = 0;

    res.sort();

    while ind < res.len() {
        sum += res[ind];
        ind += 2;
    }

    sum
}

#[test]
fn test_array_pair_sum() {
    assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
    assert_eq!(array_pair_sum(vec![1, 1, 2, 10]), 3);
    assert_eq!(array_pair_sum(vec![1, 10, 90, 100]), 91);
    assert_eq!(array_pair_sum(vec![1, 2, 3, 4, 5, 6]), 9);
}

// Example #6
// Pascal triangle.

#[allow(dead_code)]
fn pascal_generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if num_rows == 0 {
        return res;
    }

    res.push(vec![1]);
    if num_rows == 1 {
        return res;
    }

    res.push(vec![1, 1]);
    if num_rows == 2 {
        return res;
    }

    for i in 1..(num_rows as usize - 1) {
        let mut v: Vec<i32> = vec![1];

        for j in 0..(res[i].len() - 1) {
            v.push(res[i][j] + res[i][j + 1]);
        }

        v.push(1);
        res.push(v);
    }

    res
}

#[test]
fn test_pascal_generate() {
    assert_eq!(pascal_generate(1), vec![vec![1]]);
    assert_eq!(pascal_generate(2), vec![vec![1], vec![1, 1]]);
    assert_eq!(
        pascal_generate(3),
        vec![vec![1], vec![1, 1], vec![1, 2, 1],]
    );
    assert_eq!(
        pascal_generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
    assert_eq!(
        pascal_generate(6),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1]
        ]
    );
    assert_eq!(
        pascal_generate(7),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1]
        ]
    );
}

// Example #7
// Get single specific row of Pascal triangle

#[allow(dead_code)]
pub fn pascal_get_row(row_index: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![0; (row_index as usize) + 1];

    res[0] = 1;

    for n in 0..(row_index as usize + 2) {
        let mut a: i32 = 0;
        let mut b: i32;

        for e in res.iter_mut().take(n) {
            b = a;
            a = *e;
            *e += b;
        }
    }

    res
}

#[test]
fn test_pascal_get_row() {
    assert_eq!(pascal_get_row(0), vec![1]);
    assert_eq!(pascal_get_row(1), vec![1, 1]);
    assert_eq!(pascal_get_row(2), vec![1, 2, 1]);
    assert_eq!(pascal_get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(pascal_get_row(4), vec![1, 4, 6, 4, 1]);
    assert_eq!(pascal_get_row(5), vec![1, 5, 10, 10, 5, 1]);
    assert_eq!(pascal_get_row(6), vec![1, 6, 15, 20, 15, 6, 1]);
}

// Example #8
// Given a non-empty array of digits representing a non-negative integer,
// plus one to the integer. The digits are stored such that the most
// significant digit is at the head of the list, and each element
// in the array contain a single digit.

#[allow(dead_code)]
fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut p: Vec<i32> = digits.to_vec();
    let r = p.len();
    let mut z = 0;

    p[r - 1] += 1;

    for idx in (0..r).rev() {
        p[idx] += z;
        if p[idx] >= 10 {
            p[idx] -= 10;
            z = 1;
        } else {
            z = 0;
            break;
        }
    }

    if z == 1 {
        p.insert(0, 1);
    }

    p
}

#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![0]), vec![1]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
    assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    assert_eq!(plus_one(vec![4, 3, 3, 1]), vec![4, 3, 3, 2]);
}

// Example #9
// pivot index: find array index where the sum of the numbers
// to the left of the index is equal to the sum of
// the numbers to the right of the index.

#[allow(dead_code)]
fn pivot_index(num: Vec<i32>) -> i32 {
    if num.len() < 3 {
        return -1;
    }

    let mut s1: i32 = 0;
    let mut s2: i32 = num.iter().sum();

    for (idx, val) in num.iter().enumerate() {
        if s1 == s2 - val {
            return idx as i32;
        }

        s1 += val;
        s2 -= val;
    }

    -1i32
}

#[test]
fn test_pivot_index() {
    assert_eq!(pivot_index(vec![]), -1);
    assert_eq!(pivot_index(vec![1]), -1);
    assert_eq!(pivot_index(vec![1, 2]), -1);
    assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pivot_index(vec![1, 1, 1, 1, 1, 1, 1]), 3);
    assert_eq!(pivot_index(vec![-7, 1, 5, 2, -4, 3, 0]), 3);
    assert_eq!(pivot_index(vec![-1, -1, -1, 0, 1, 1]), 0);
    assert_eq!(pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
    assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(pivot_index(vec![1, 2, 3, 4]), -1);
}

// Example #10
// Given an array and a value, remove all instances of that value
// in-place using O(1) of extra memory and return the new length.

#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
    let mut k = 0;

    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k
}

#[test]
fn test_remove_element() {
    let mut num = vec![3, 2, 2, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![2, 3, 2, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![2, 2, 3, 3];
    let len = remove_element(&mut num, 3);
    num.resize(len, 0);
    assert_eq!(len, 2);
    assert_eq!(vec![2, 2], num);

    let mut num = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let len = remove_element(&mut num, 2);
    num.resize(len, 0);
    assert_eq!(len, 5);
    assert_eq!(vec![0, 1, 3, 0, 4], num);
}

// Example #11
// Shift array to the right by specified value.
// Solution using reversed subarrays:
// shift (a1a2) = rev(rev(a1)rev(a2))

#[allow(dead_code)]
fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 {
        return;
    }

    let n: i32 = nums.len() as i32;
    let k1: i32 = k % n;
    let k2: usize = (n - k1) as usize;

    nums[0..k2].reverse();
    nums[k2..].reverse();
    nums.reverse();
}

#[test]
fn test_rotate() {
    let mut nums = vec![];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![]);

    let mut nums = vec![1];
    rotate(&mut nums, 5);
    assert_eq!(nums, vec![1]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 0);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![2, 1]);

    let mut nums = vec![1, 2];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![1, 2]);

    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 1);
    assert_eq!(nums, vec![4, 1, 2, 3]);

    let mut nums = vec![1, 2, 3, 4];
    rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 4, 1, 2]);
}

// Example #12
// Given a matrix of m x n elements (m rows, n columns), return all
// elements of the matrix in spiral order.
//
// Input:
// [
//   [ 1, 2, 3 ],
//   [ 4, 5, 6 ],
//   [ 7, 8, 9 ]
// ]
//
// Output:
// [1,2,3,6,9,8,7,4,5]

#[allow(dead_code)]
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let n = matrix.len();
    if n == 0 {
        return vec![];
    }

    let m = matrix[0].len();
    if m == 0 {
        return vec![];
    }

    let mut res: Vec<i32> = Vec::new();

    let mut i1 = 0;
    let mut i2 = m - 1;
    let mut j1 = 0;
    let mut j2 = n - 1;

    loop {
        if i1 > i2 {
            break;
        }

        for s in i1..=i2 {
            res.push(matrix[j1][s]);
        }

        if (j1 + 1) > j2 {
            break;
        }

        for e in matrix.iter().take(j2 + 1).skip(j1 + 1) {
            res.push(e[i2]);
        }

        if i1 >= i2 {
            break;
        }

        for s in (i1..i2).rev() {
            res.push(matrix[j2][s]);
        }

        if (j1 + 1) >= j2 {
            break;
        }

        for s in ((j1 + 1)..j2).rev() {
            res.push(matrix[s][i1]);
        }

        i1 += 1;
        i2 = i2.saturating_sub(1);
        j1 += 1;
        j2 = j2.saturating_sub(1);
    }

    res
}

#[test]
fn test_spiral_order() {
    assert_eq!(spiral_order(vec![vec![]]), vec![]);
    assert_eq!(spiral_order(vec![vec![], vec![]]), vec![]);
    assert_eq!(spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(spiral_order(vec![vec![1, 2]]), vec![1, 2]);
    assert_eq!(spiral_order(vec![vec![1, 2, 3, 4]]), vec![1, 2, 3, 4]);
    assert_eq!(
        spiral_order(vec![vec![1], vec![2], vec![3], vec![4]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(spiral_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 4, 3]);
    assert_eq!(
        spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
}

// Example #13
// Largest number at least twice of others

#[allow(dead_code)]
fn the_largest_index(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    if nums.len() == 1 {
        return 0;
    }

    let mut p1: usize = 0;
    let mut v2: i32 = 0;

    for (idx, val) in nums[1..].iter().enumerate() {
        if *val > nums[p1] {
            if nums[p1] > v2 {
                v2 = nums[p1]
            }

            p1 = idx + 1;
        } else if *val > v2 {
            v2 = *val;
        }
    }

    if 2 * v2 <= nums[p1] {
        return p1 as i32;
    }

    -1i32
}

#[test]
fn test_the_largest_index() {
    assert_eq!(the_largest_index(vec![]), -1);
    assert_eq!(the_largest_index(vec![2, 3]), -1);
    assert_eq!(the_largest_index(vec![3, 2, 1, 0]), -1);
    assert_eq!(the_largest_index(vec![3, 3, 1, 5]), -1);
    assert_eq!(the_largest_index(vec![1]), 0);
    assert_eq!(the_largest_index(vec![1, 2]), 1);
    assert_eq!(the_largest_index(vec![3, 6, 1, 0]), 1);
    assert_eq!(the_largest_index(vec![6, 3, 1, 0]), 0);
    assert_eq!(the_largest_index(vec![1, 3, 1, 7]), 3);
}

// Example #14
// Given a sorted array nums, remove the duplicates in-place
// such that each element appear only once and return the
// new length. Do not allocate extra space for another array,
// you must do this by modifying the input array
// in-place with O(1) extra memory.
#[allow(dead_code)]
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut n = 1;

    for i in 0..nums.len() {
        if nums[n - 1] != nums[i] {
            nums[n] = nums[i];
            n += 1;
        }
    }

    n as i32
}

#[test]
fn test_remove_duplicates() {
    assert_eq!(0, remove_duplicates(&mut vec![]));
    assert_eq!(1, remove_duplicates(&mut vec![1]));
    assert_eq!(2, remove_duplicates(&mut vec![1, 2]));
    assert_eq!(1, remove_duplicates(&mut vec![1, 1, 1, 1, 1]));
    assert_eq!(4, remove_duplicates(&mut vec![0, 1, 2, 3]));
    assert_eq!(
        5,
        remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    );
}
