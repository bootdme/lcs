use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut check = HashMap::new();

        for num in nums {
            if check.contains_key(&num) {
                return true;
            }  else {
                check.insert(num, 0);
            }
        }

        false
    }
}