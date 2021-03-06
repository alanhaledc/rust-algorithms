/*
 * @lc app=leetcode.cn id=475 lang=rust
 *
 * [475] 供暖器
 *
 * https://leetcode-cn.com/problems/heaters/description/
 *
 * algorithms
 * Medium (38.83%)
 * Likes:    333
 * Dislikes: 0
 * Total Accepted:    41.8K
 * Total Submissions: 107.6K
 * Testcase Example:  '[1,2,3]\n[2]'
 *
 * 冬季已经来临。 你的任务是设计一个有固定加热半径的供暖器向所有房屋供暖。
 *
 * 在加热器的加热半径范围内的每个房屋都可以获得供暖。
 *
 * 现在，给出位于一条水平线上的房屋 houses 和供暖器 heaters 的位置，请你找出并返回可以覆盖所有房屋的最小加热半径。
 *
 * 说明：所有供暖器都遵循你的半径标准，加热的半径也一样。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: houses = [1,2,3], heaters = [2]
 * 输出: 1
 * 解释: 仅在位置2上有一个供暖器。如果我们将加热半径设为1，那么所有房屋就都能得到供暖。
 *
 *
 * 示例 2:
 *
 *
 * 输入: houses = [1,2,3,4], heaters = [1,4]
 * 输出: 1
 * 解释: 在位置1, 4上有两个供暖器。我们需要将加热半径设为1，这样所有房屋就都能得到供暖。
 *
 *
 * 示例 3：
 *
 *
 * 输入：houses = [1,5], heaters = [2]
 * 输出：3
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 *
 *
 */

// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        let mut radius = 0;

        houses.sort();
        heaters.sort();
        let mut cache: Vec<i32> = vec![std::i32::MAX; houses.len()];

        let mut l = 0;
        let mut h = 0;
        while l < houses.len() && h < heaters.len() {
            if houses[l] <= heaters[h] {
                cache[l] = heaters[h] - houses[l];
                l += 1;
            } else {
                h += 1;
            }
        }

        let mut r: i32 = houses.len() as i32 - 1;
        let mut h: i32 = heaters.len() as i32 - 1;
        while r >= 0 && h >= 0 {
            if houses[r as usize] >= heaters[h as usize] {
                cache[r as usize] =
                    min(cache[r as usize], houses[r as usize] - heaters[h as usize]);
                r -= 1;
            } else {
                h -= 1;
            }
        }
        (cache.iter()).for_each(|&x| radius = max(radius, x));

        radius
    }
}

// @lc code=end
