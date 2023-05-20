impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Include HashMap
        use std::collections::HashMap;

        // Create a mutable HashMap variable
        let mut map: HashMap<i32, i32> = HashMap::new();

        // Loop through nums vector
        for (index, &value) in nums.iter().enumerate() {
            // Initialize a complement variable
            let complement: i32 = target - value;

            // If complement is in HashMap
            if let Some(&i) = map.get(&complement) {
                // Return [index of complement, index of current number]   
                return vec![i as i32, index as i32];
            }
            // Add [current number, index] to HashMap
            map.insert(value, index as i32);
        }

        // Return empty vector since the function requires to return a vector
        vec![]
    }
}