// Task:
// Given an array of integers nums and an integer k, return the maximum sum of a subsequence such that no two elements in the subsequence are adjacent in the array, and the subsequence has exactly k elements.

// If no such subsequence exists (e.g., k is larger than possible), return -1.

// Example 1:
// Input: nums = [3, 2, 5, 8, 7], k = 2
// Output: 10 (Explanation: [3, 7] or [5, 5] are possible)

// Example 2:
// Input: nums = [1, 2, 3, 4, 5], k = 3
// Output: 9 (Explanation: [1, 3, 5])

// Constraints:

//     1 <= nums.length <= 1000

//     1 <= k <= (nums.length + 1) // 2

// Follow-up: Can you solve it in O(nk) time with O(nk) space?
fn max_sum_of_subseq(nums: &[i32], k: usize) -> i32 {
    if nums.len() < k {
        return -1
    }

    let mut indexed: Vec<_> = nums.iter().enumerate().collect();
    indexed.sort_by_key(|&(_idx, & val)| std::cmp::Reverse(val));

    for i in 0..nums.len() - k {
        if let Some(chunk)= indexed.get(i..i+k) {
            
            

            let sum: i32 = chunk.iter().map(|&(_k, v)| v).sum();
            return sum;
        }
    }

    return 0
}

// day 1
// start: 9:46
// pause: 10:24
// resume: 10:55
// pause: 11:59
fn main() {
    println!("Hello, world!");
}

// Test module
#[cfg(test)]
mod tests {
    use super::*; // Import items from parent module
    use test_case::test_case;

    #[test_case(&[], 1, -1)]
    #[test_case(&[], 0, 0)]
    #[test_case(&[1, 2, 3], 1, 3)]
    // k = 2
    #[test_case(&[1, 2, 3], 2, 4)]
    #[test_case(&[1, 3, 2], 2, 3)]
    #[test_case(&[1, 4, 2], 2, 4)]
    fn test_max_sum_of_subseq(nums: &[i32], k: usize, expected: i32) {
        assert_eq!(max_sum_of_subseq(nums, k), expected);
    }
}
