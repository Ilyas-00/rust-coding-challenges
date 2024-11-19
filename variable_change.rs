//! Problem Statement:
// Declare a variable x and store value 1000 in it.
// Declare a variable y and store value "Programming" in it.
// Print the values of x and y
// Change the value of x to 1100
// Print the values of x and y

fn test() {
  
    let mut x = 1000; 
    let y = "Programming"; 
    
    println!("x:{}", x); 
    
    println!("y:{}", y); 
    
    x = 1100; 
    
    println!("x:{}", x);
    
    println!("y:{}", y); 
}

fn main() {
    test(); 
}
