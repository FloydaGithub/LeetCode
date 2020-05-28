use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::with_capacity(nums.len());

        for (k, v) in nums.iter().enumerate() {
            match hash.get(&(target - v)) {
                None => { hash.insert(v, k); },
                Some(pos) => {
                    return vec![*pos as i32, k as i32]
                }
            }
        }
        vec![]
    }
}

fn main() {
    let result = Solution::two_sum(vec![1,2,3,4,5,6,7], 11);
    println!("{:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}