/*
Reference: 
A reference is like an address that points to a value stored in memory. 
In Rust, references are indicated by the & symbol, and mutable references by &mut.

Dereferencing: 
Dereferencing is accessing the actual value at the address the reference points to. 
In Rust, dereferencing is done using the * operator.
*/
fn main(){
/* Here, y is a reference to x. To get the value of x through y, you dereference y: */
let x = 10;
let y = &x;
println!("The value of x is: {}", *y);

/*When you have a mutable reference, you can change the value it points to: */

let mut x1 = 10;
let y1 = &mut x1;
*y1 += 1;
println!("The value of x is: {}", x1); // This will print: The value of x is: 11

}
