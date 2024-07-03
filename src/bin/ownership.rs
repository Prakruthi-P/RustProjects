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
fn display(v:Vec<i32>){
    println!("inside display {:?}",v);
 }
fn main(){
    /*let v = vec![1,2,3]; 
    // vector v owns the object in heap
    //only a single variable owns the heap memory at any given time
    let v2 = v; 
    // here two variables owns heap value,
    //two pointers to the same content is not allowed in rust
    //Rust is very smart in terms of memory access ,so it detects a race condition
    //as two variables point to same heap
 
    println!("{:?}",v2);*/

    /*let v = vec![1,20,3];     // vector v owns the object in heap
    let v2 = v;              // moves ownership to v2
    display(u2);             // v2 is moved to display and v2 is invalidated
    println!("In main {:?}",u2);    //v2 is No longer usable here
    */
    let u1 = 10;
    let u2 = u1;  // u1 value copied(not moved) to u2
 
    println!("u1 = {}",u1);
 }