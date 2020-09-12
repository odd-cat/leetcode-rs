struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut m = std::collections::HashMap::new();
        for i in 0..nums.len() {
            if let Some(j) = m.get(&(target - nums[i])) {
                return vec![*j as i32, i as i32];
            } else {
                m.insert(nums[i], i);
            }
        }
        vec![]
    }
}

fn main() {
    let target = 9;
    let v = Solution::two_sum(vec![2, 7, 11, 15], target);
    println!("nums[{}] + nums[{}] = {}", v[0], v[1], target);
}
