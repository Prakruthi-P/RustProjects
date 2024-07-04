/*
A slice is a pointer to a block of memory. 
Slices can be used to access portions of data stored in contiguous memory blocks. 
It can be used with data structures like arrays, vectors and strings. Slices use index numbers to access portions of data. 
The size of a slice is determined at runtime.

Slices are pointers to the actual data. 
They are passed by reference to functions, which is also known as borrowing.

For example, slices can be used to fetch a portion of a string value. 
A sliced string is a pointer to the actual string object. 
Therefore, we need to specify the starting and ending index of a String. Index starts from 0 just like arrays.

syntax:
let sliced_value = &data_structure[start_index..end_index]

*/
fn use_slice(slice:&[i32]) { 
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
 }
 fn use_slice_mut(slice:&mut [i32]) {
    println!("length of slice is {:?}",slice.len());
    println!("{:?}",slice);
    slice[0] = 1010;
    slice[1]=233 // replaces 20 with 1010
 }
fn main() {
    let n1 = "Tutorials".to_string();
    println!("length of string is {}",n1.len());
    let c1 = &n1[4..9];  // fetches characters at 4,5,6,7, and 8 indexes
    println!("{}",c1);

    //Slicing an integer array
    let data = [10,20,30,40,50];
    use_slice(&data[1..4]);

    /*
    Mutable Slices
    The &mut keyword can be used to mark a slice as mutable.
    */
    let mut data_mutable = [10,20,30,40,50];
    use_slice_mut(&mut data_mutable[1..4]);// passes references of 20, 30 and 40
    println!("{:?}",data_mutable);
 }
 