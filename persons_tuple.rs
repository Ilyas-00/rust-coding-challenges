//! Problem Statement
// Declare a tuple, persons, of size 6 that has the names of people along with their ages
// Print the value of persons
// Output
// The output of the code should be:

// Alex:21, Abe:22, Anna:23
// Coding Exercise#

fn test() {
    // Write code here!
    let persons = ("Alex", 21, "Abe", 22, "Anna", 23); 
    print!("{}:{}, {}:{}, {}:{}", persons.0, persons.1, persons.2, persons.3, persons.4, persons.5); 
}