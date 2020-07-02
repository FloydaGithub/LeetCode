/*

https://leetcode-cn.com/problems/kth-smallest-element-in-a-sorted-matrix/

378. 有序矩阵中第K小的元素
给定一个 n x n 矩阵，其中每行和每列元素均按升序排序，找到矩阵中第 k 小的元素。
请注意，它是排序后的第 k 小元素，而不是第 k 个不同的元素。



示例：

matrix = [
   [ 1,  5,  9],
   [10, 11, 13],
   [12, 13, 15]
],
k = 8,

返回 13。


提示：
你可以假设 k 的值永远是有效的，1 ≤ k ≤ n2 。

 */

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::with_capacity((k + 1) as usize);

        for array in matrix {
            for element in array {
                heap.push(element);

                if heap.len() > k as usize {
                    heap.pop();
                }
            }
        }

        heap.pop().unwrap()
    }
}

fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]];
    let k = 8;
    let result: i32 = Solution::kth_smallest(matrix, k);
    println!("{:?}", result);
}
