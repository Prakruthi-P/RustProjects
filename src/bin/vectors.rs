/*
A Vector is a resizable array. 
It stores values in contiguous memory blocks. 
The predefined structure Vec can be used to create vectors. 
Some important features of a Vector are −

A Vector can grow or shrink at runtime.
A Vector is a homogeneous collection.
A Vector stores data as sequence of elements in a particular order. Every element in a Vector is assigned a unique index number. 
The index starts from 0 and goes up to n-1 where, n is the size of the collection.
A Vector will only append values to (or near) the end. In other words, a Vector can be used to implement a stack.
Memory for a Vector is allocated in the heap.

Syntax - Creating a Vector
let mut instance_name = Vec::new();
let vector_name = vec![val1,val2,val3]

new()	
pub fn new()->Vect
Constructs a new, empty Vec. The vector will not allocate until elements are pushed onto it.

push()	
pub fn push(&mut self, value: T)
Appends an element to the back of a collection.

remove()	
pub fn remove(&mut self, index: usize) -> T
Removes and returns the element at position index within the vector, shifting all elements after it to the left.

contains()	
pub fn contains(&self, x: &T) -> bool
Returns true if the slice contains an element with the given value.

len()	
pub fn len(&self) -> usize
Returns the number of elements in the vector, also referred to as its 'length'.

*/

fn main() {
    let mut  v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
 
    println!("size of vector is :{}",v.len());
    println!("{:?}",v);
    v.remove(1);
    println!("After deleting the element at index 1 {:?}",v);

    let v1 = vec![1,2,3];
    println!("{:?}",v1);

    if v.contains(&10) {
        println!("found 10 in the vector");
     }
     else{
        println!("Element 10 is not presnt in the vector");
     }
    
    //Accessing values from a Vector
    println!("{:?}",v[0]);

    //accessing elements via for loop
    for i in &v {
        println!("{}",i);
     }
    
    //modifying an element 
    v[1]=200;
    println!("After modification {:?}",v);

    //inserting elemant at specific index
    v.insert(2,122);
    println!("After insering {:?} , length {} ",v,v.len());
    v.sort();
    println!("After sorting {:?}",v);
    let mut vec_of_vecs: Vec<Vec<i32>> = Vec::new();
    // Add vectors to the vector of vectors
    vec_of_vecs.push(vec![1, 2, 3]);
    vec_of_vecs.push(vec![4, 5, 6]);
    
    // Borrowing elements
    let first_subvec = &vec_of_vecs[0];
    
    println!("first_subvec: {:?}",first_subvec);


    
    //vector can only contain values of the same data type. The following snippet will throw error
    //let v = vec![1,2,3,"hello"];
 }