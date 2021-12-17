/*
 * @lc app=leetcode.cn id=1518 lang=rust
 *
 * [1518] 换酒问题
 *
 * https://leetcode-cn.com/problems/water-bottles/description/
 *
 * algorithms
 * Easy (69.36%)
 * Likes:    87
 * Dislikes: 0
 * Total Accepted:    33.7K
 * Total Submissions: 48.7K
 * Testcase Example:  '9\n3'
 *
 * 小区便利店正在促销，用 numExchange 个空酒瓶可以兑换一瓶新酒。你购入了 numBottles 瓶酒。
 *
 * 如果喝掉了酒瓶中的酒，那么酒瓶就会变成空的。
 *
 * 请你计算 最多 能喝到多少瓶酒。
 *
 *
 *
 * 示例 1：
 *
 *
 *
 * 输入：numBottles = 9, numExchange = 3
 * 输出：13
 * 解释：你可以用 3 个空酒瓶兑换 1 瓶酒。
 * 所以最多能喝到 9 + 3 + 1 = 13 瓶酒。
 *
 *
 * 示例 2：
 *
 *
 *
 * 输入：numBottles = 15, numExchange = 4
 * 输出：19
 * 解释：你可以用 4 个空酒瓶兑换 1 瓶酒。
 * 所以最多能喝到 15 + 3 + 1 = 19 瓶酒。
 *
 *
 * 示例 3：
 *
 * 输入：numBottles = 5, numExchange = 5
 * 输出：6
 *
 *
 * 示例 4：
 *
 * 输入：numBottles = 2, numExchange = 3
 * 输出：2
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= numBottles <= 100
 * 2 <= numExchange <= 100
 *
 *
 */

// @lc code=start
impl Solution {
    // pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    //     let mut res = num_bottles;
    //     let mut num = num_bottles;
    //     while num >= num_exchange {
    //         res += num / num_exchange;
    //         num = num / num_exchange + num % num_exchange;
    //     }
    //     res
    // }

    // math
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // 每次损失 numExchange - 1 个空酒瓶
        let mut res = num_bottles + num_bottles / (num_exchange - 1);
        // 注意边界条件：当 numBottles 为 numExchange - 1 的倍数时，最后一个回合不满足兑换条件
        if num_bottles % (num_exchange - 1) == 0 {
            res -= 1
        }
        res
    }
}
// @lc code=end
