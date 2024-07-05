/*
Variables are scalar in nature. 
In other words, a variable declaration can only contain a single value at a time. 
This means that to store n values in a program n variable declaration will be needed. 
Hence, the use of variables is not feasible when one needs to store a larger collection of values.
Variables in a program are allocated memory in random order, 
thereby making it difficult to retrieve/read the values in the order of their declaration.

An array declaration allocates sequential memory blocks.

Arrays are static. This means that an array once initialized cannot be resized.

Each memory block represents an array element.

Array elements are identified by a unique integer called the subscript/ index of the element.

Populating the array elements is known as array initialization.

Array element values can be updated or modified but cannot be deleted.

//Syntax1
let variable_name = [value1,value2,value3];

//Syntax2
let variable_name:[dataType;size] = [value1,value2,value3];

//Syntax3
let variable_name:[dataType;size] = [default_value_for_elements,size];

 
*/
//mutable and transfer of ownership
fn pass_by_value(mut arr:  [i32;3]){
    for i in 0..arr.len() {
       arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
 }
 //mutable and borrow
 fn pass_by_reference(arr:&mut [i32]){
    for i in 0..arr.len() {
       arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
 }
 
fn main(){
    let arr:[i32;4] = [10,20,30,40];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());
    let arr1 = [10,20,30,40];
    println!("array is {:?}",arr1);
    println!("array1 size is :{}",arr1.len());
    /*The following example creates an array and initializes all its elements with a default value of -1. */
    let arr2:[i32;4] = [-1;4];
    println!("array2 is {:?}",arr2);
    println!("array2 size is :{}",arr2.len());

    let arr3:[i32;4] = [10,20,30,40];
    println!("array3 is {:?}",arr3);
    println!("array3 size is :{}",arr3.len());
 
    for index in 0..4 {
       println!("index is: {} & value is : {}",index,arr3[index]);
    }
    /*The iter() function fetches values of all elements in an array. */
    let arr4:[i32;4] = [10,20,30,40];
    println!("array4 is {:?}",arr);
    println!("array4 size is :{}",arr.len());
 
    for val in arr4.iter(){
       println!("value is :{}",val);
    }
    /*
    The mut keyword can be used to declare a mutable array. 
    The following example declares a mutable array and modifies value of the second array element.
     */

     let mut arr:[i32;4] = [10,20,30,40];
     arr[1] = 0;
     println!("{:?}",arr);

     /*An array can be passed by value or by reference to functions. */

     let arr6 = [10,20,30];
     pass_by_value(arr6);
     println!("Inside main {:?}",arr6);

     let mut arr7 = [10,20,30];
     pass_by_reference(&mut arr7);
     println!("Inside main {:?}",arr7);

     /* let N: usize = 20;
     let arr = [0; N]; //Error: non-constant used with constant
     print!("{}",arr[10]) */
    
     const N: usize = 20; 
     // pointer sized
     let arr8 = [0; N];
     println!("arr8 {:?}",arr8)
 }