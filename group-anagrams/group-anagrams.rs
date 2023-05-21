impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Add HashMap from standard library
        use std::collections::HashMap;

        // Create a HashMap<String, Vec<String>>
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        // Loop through strs
        for str in strs {
            // Split the current str into characters
            let mut c: Vec<char> = str.chars().collect::<Vec<char>>();
            // Sort the characters
            c.sort();
            // Combine the characters into a string
            let combine: String = c.iter().collect::<String>();
            // Add or append combined characters to HashMap
            map.entry(combine).or_default().push(str);
        }

        map.into_values().collect()
    }
}