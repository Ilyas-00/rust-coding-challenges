//! Problem Statement:
// The task requires you to find all words starting with a "c" in a string passed as a parameter.
// Concatenate them together and return the result.
//
// Input:
// - A String.
//
// Output:
// - A concatenated String with all words starting with "c".
//
// Example:
// Input: "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course"
// Output: "comprehensive course concentration content course"

fn test(my_str: String) -> String {
    let mut mots = Vec::new(); 

    for token in my_str.split_whitespace() {
        if token.starts_with('c') {
            mots.push(token); 
        }
    }

    mots.join(" ")
}


fn main() {
    let input = "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course".to_string();
    let output = test(input);
    println!("Result: {}", output);
}
