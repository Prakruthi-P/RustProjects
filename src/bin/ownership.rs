/*
The memory for a program can be allocated in the following −
=> Stack
=> Heap
Stack
A stack follows a last in first out order.
Stack stores data values for which the size is known at compile time. 
For example, a variable of fixed size i32 is a candidate for stack allocation. 
Its size is known at compile time. 
All scalar types can be stored in stack as the size is fixed.
Consider an example of a string, which is assigned a value at runtime. 
The exact size of such a string cannot be determined at compile time. 
So it is not a candidate for stack allocation but for heap allocation.

Heap
The heap memory stores data values the size of which is unknown at compile time. 
It is used to store dynamic data. 
Simply put, a heap memory is allocated to data values that may change throughout the life cycle of the program. 
The heap is an area in the memory which is less organized when compared to stack.

Each value in Rust has a variable that is called owner of the value. 
Every data stored in Rust will have an owner associated with it. 
For example, in the syntax − let age = 30, age is the owner of the value 30.

Each data can have only one owner at a time.
Two variables cannot point to the same memory location. 
The variables will always be pointing to different memory locations.

Transferring Ownership

The ownership of value can be transferred by −
* Assigning value of one variable to another variable.
* Passing value to a function.
* Returning value from a function.
*/
fn take_ownership(s: String) {
    println!("{}", s);
}
fn take_and_give_back(s: String) -> String {
    println!("{}", s);
    s
}
fn main(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is still valid
    println!("s1 = {}, s2 = {}", s1, s2); // this is fine

   /* 
    Ownership and Primitive Types
    Copy
    Some types, like integers, are Copy. 
    If a type has the Copy trait, then an older variable is still usable after assignment. 
    Rust implements Copy for types that are simple and fixed-size.
   */ 
    let x = 5;
    let y = x; // x is still valid
    println!("x = {}, y = {}", x, y); // this is fine

    let u1 = 10;
    let u2 = u1;  // u1 value copied(not moved) to u2
 
    println!("u1 = {}",u1);
    println!("u2 = {}",u2);


    /*When you pass a value to a function, ownership moves to the function. */
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // this would cause a compile-time error because s is no longer valid

    /*If you want to keep using the value after the function call, you need to return it. */
    let s1 = String::from("hello");
    let s1 = take_and_give_back(s1); // s is returned and reassigned to s
    println!("{}", s1); // this is fine


 }