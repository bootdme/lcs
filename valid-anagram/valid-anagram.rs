impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // If s and t length are not equal, return false
        if (s.len() != t.len()) {
            return false;
        }

        // Create a mutable array of 26 0's for the letters in alphabet
        let mut alphabet_array: [usize; 26] = [0; 26];

        // Loop through s
        for i in s.chars() {
            // Convert character to ASCII
            let ascii_value: usize = i as usize;
            // Subtract 97 from ASCII value to get index for the array
            let index: usize = ascii_value - 97; 
            // Add 1 at the index of array
            alphabet_array[index] += 1;
        }

        // Loop through t
        for j in t.chars() {
            // Convert character to ASCII
            let ascii_value: usize = j as usize;
            // Subtract 97 from ASCII value to get index for the array
            let index: usize = ascii_value - 97;
            // Subtract 1 at the index of array
            alphabet_array[index] -= 1;
        }

        // Loop through array
        for index in alphabet_array {
            // If value at the index does not equal 0, return false
            if (index != 0) {
                return false;
            }
        }
        
        // Return true
        return true;
    }
}