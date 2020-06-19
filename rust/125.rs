/*

https://leetcode-cn.com/problems/valid-palindrome/

125. 验证回文串
给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。

说明：本题中，我们将空字符串定义为有效的回文串。

示例 1:

输入: "A man, a plan, a canal: Panama"
输出: true
示例 2:

输入: "race a car"
输出: false
 */

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // let mut values: Vec<u8> = Vec::new();
        // for b in s.to_lowercase().bytes() {
        //     // 0..9
        //     if b >= 48 && b <= 57 {
        //         values.push(b);
        //     }
        //     // a..z
        //     if b >= 97 && b <= 122 {
        //         values.push(b);
        //     }
        // }
        let s = s.to_ascii_lowercase();
        let values: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .rev()
            .collect();

        let (mut left, mut right) = (0 as i32, values.len() as i32 - 1);
        while left <= right {
            if values[left as usize] == values[right as usize] {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    let input: String = "A man, a plan, a canal: Panama".to_string();
    // let input: String = "".to_string();
    let result: bool = Solution::is_palindrome(input);
    println!("{:?}", result);
}
