/*
https://leetcode-cn.com/problems/spiral-matrix/
https://leetcode-cn.com/problems/shun-shi-zhen-da-yin-ju-zhen-lcof/

54. 螺旋矩阵

给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。

示例 1:

输入:
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
输出: [1,2,3,6,9,8,7,4,5]
示例 2:

输入:
[
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9,10,11,12]
]
输出: [1,2,3,4,8,12,11,10,9,5,6,7]

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/spiral-matrix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

struct Solution;

#[derive(Copy, Clone)]
struct Towards {
    count: i8,
}

impl Towards {
    fn new() -> Towards {
        Towards { count: 0 }
    }

    fn move_point(self, x: i32, y: i32) -> (i32, i32) {
        match self.count {
            0 => return (x + 1, y),
            1 => return (x, y + 1),
            2 => return (x - 1, y),
            3 => return (x, y - 1),
            _ => (0, 0),
        }
    }
}

impl Iterator for Towards {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count >= 4 {
            self.count = 0;
        }
        Some(self.count)
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut scores = HashMap::new();

        let height = matrix.len() as i32;
        if height == 0 {
            return vec![];
        }
        let width = matrix[0].len() as i32;
        let total = (width * height) as usize;

        let mut result: Vec<i32> = vec![];
        let mut x = -1;
        let mut y = 0;
        let mut toward = Towards::new();
        loop {
            let (_x, _y) = toward.move_point(x, y);
            if _x < 0 || _x >= width {
                toward.next();
                continue;
            }
            if _y < 0 || _y >= height {
                toward.next();
                continue;
            }
            if *scores
                .get(&(_x.to_string() + "-" + &_y.to_string()))
                .unwrap_or(&false)
            {
                toward.next();
                continue;
            }

            result.push(matrix[_y as usize][_x as usize]);
            scores.insert(_x.to_string() + "-" + &_y.to_string(), true);
            if result.len() >= total {
                break;
            }
            x = _x;
            y = _y;
        }
        result
    }
}

fn main() {
    let input: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    let results: Vec<i32> = Solution::spiral_order(input);
    println!("{:?}", results);
}
