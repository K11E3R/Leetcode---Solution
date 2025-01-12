// https://leetcode.com/problems/pass-the-pillow

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut k_time = time / (n - 1);
        let mut _mod = time % (n - 1);

        if (k_time & 1) == 1 {
            return n - _mod;
        }

        _mod + 1
    }
}