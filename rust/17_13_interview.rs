/*

https://leetcode-cn.com/problems/re-space-lcci/

面试题 17.13. 恢复空格
哦，不！你不小心把一个长篇文章中的空格、标点都删掉了，并且大写也弄成了小写。像句子"I reset the computer. It still didn’t boot!"已经变成了"iresetthecomputeritstilldidntboot"。在处理标点符号和大小写之前，你得先把它断成词语。当然了，你有一本厚厚的词典dictionary，不过，有些词没在词典里。假设文章用sentence表示，设计一个算法，把文章断开，要求未识别的字符最少，返回未识别的字符数。

注意：本题相对原题稍作改动，只需返回未识别的字符数



示例：

输入：
dictionary = ["looked","just","like","her","brother"]
sentence = "jesslookedjustliketimherbrother"
输出： 7
解释： 断句后为"jess looked just like tim her brother"，共7个未识别字符。
提示：

0 <= len(sentence) <= 1000
dictionary中总字符数不超过 150000。
你可以认为dictionary和sentence中只包含小写字母。

 */

struct Solution;

impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        fn rest(sentence: &str, dictionary: &Vec<String>, dp: &mut Vec<usize>) {
            let al = sentence.len();
            let mut min = dp[al - 1] + 1;
            for cc in dictionary {
                if sentence.starts_with(cc) {
                    min = min.min(dp[sentence.len() - cc.len()]);
                }
            }
            dp[al] = min;
        }

        let mut dp = vec![0; sentence.len() + 1]; // a的最后i个字符的剩余

        for i in 1..sentence.len() + 1 {
            let aa = &sentence[sentence.len() - i..];
            rest(&aa, &dictionary, &mut dp);
        }
        dp[sentence.len()] as i32
    }
}

fn main() {
    let dictionary: Vec<String> = vec![
        String::from("looked"),
        String::from("just"),
        String::from("like"),
        String::from("her"),
        String::from("brother"),
    ];
    let sentence: String = String::from("jesslookedjustliketimherbrother");

    let result = Solution::respace(dictionary, sentence);
    println!("{:?}", result);
}
