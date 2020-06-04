/*
给定一个经过编码的字符串，返回它解码后的字符串。

编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。

你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。

此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。

示例:

s = "3[a]2[bc]", 返回 "aaabcbc".
s = "3[a2[c]]", 返回 "accaccacc".
s = "2[abc]3[cd]ef", 返回 "abcabccdcdcdef".
*/

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut nums = vec![];
        let mut repeat_str = vec![];
        let mut current: String = "".to_string();
        let mut num = 0;

        for c in s.chars() {
            match c {
                '0'..='9' => num = num * 10 + c as usize - 48,
                '[' => {
                    nums.push(num);
                    num = 0;
                    repeat_str.push(current);
                    current = "".to_string();
                }
                ']' => {
                    current = repeat_str.pop().unwrap() + &current.repeat(nums.pop().unwrap());
                }
                _ => current.push(c),
            }
        }
        current
    }
}

fn main() {
    // let mut result: String = "3[a]2[bc]".to_string();
    // let mut result: String = "3[a234kk]2[bc]".to_string();
    let mut result: String = "2[a3[b4[c5[d]]]]".to_string();
    result = Solution::decode_string(result);
    println!("{:?}", result);
}
