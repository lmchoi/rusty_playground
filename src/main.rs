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

fn _split_adjacents(chunk: &[(usize, &i32)]) -> Vec<Vec<i32>> {
    return vec![vec![]];
}

fn get_value_at(chunk: &[(usize, &i32)], i: usize) -> i32 {
    chunk
        .iter()
        .find_map(|&(index, &value)| (index == i).then_some(value))
        .unwrap_or(0)
}

// Follow-up: Can you solve it in O(nk) time with O(nk) space?
fn max_sum_of_subseq(nums: &[i32], k: usize) -> i32 {
    if nums.len() < k {
        return -1;
    }

    let mut indexed: Vec<_> = nums.iter().enumerate().collect();
    indexed.sort_by_key(|&(_idx, &val)| std::cmp::Reverse(val));

    if let Some(chunk) = indexed.get(0..k) {
        let total: i32 = chunk.iter().map(|&(_idx, val)| val).sum();

        let ii = chunk.get(0).unwrap().0;

        let left_value = get_value_at(chunk, ii - 1);
        let right_value = get_value_at(chunk, ii + 1);

        // get replacement value(s)

        return total - left_value - right_value;
    }

    return 0;
}

// day 2
// start: 6:57
// end: 8:04
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
