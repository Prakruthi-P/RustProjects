/*
HashSet is a set of unique values of type T. Adding and removing values is fast, and it is fast to ask whether a given value is in the set or not. 
The HashSet structure is defined in the std::collections module. 
This module should be explicitly imported to access the HashSet structure.

Syntax :
let mut hash_set_name = HashSet::new();

*/
use::std::collections::HashSet;
fn main(){
//new: Creates an empty HashSet.
let mut set1: HashSet<i32> = HashSet::new();
set1.insert(100);
set1.insert(5000);
set1.insert(4999);
set1.insert(1);
set1.insert(0);
set1.insert(3);
set1.insert(4);
set1.insert(10);
set1.insert(20);
println!("{:?}",set1);
//with_capacity: Creates an empty HashSet with the specified capacity.
let mut set: HashSet<i32> = HashSet::with_capacity(10);
//insert: Inserts a value into the HashSet. Returns true if the value was not present.
set.insert(1);
set.insert(0);
set.insert(3);
set.insert(4);
set.insert(10);
set.insert(20);
set.insert(30);
set.insert(40);
//contains: Checks if the HashSet contains the specified value.
println!("The set contains the element 10 {}",set.contains(&10));
//remove: Removes a value from the HashSet. Returns true if the value was present.
set.remove(&10);
//iter: Returns an iterator over the values.
for value in set.iter() {
    println!("{}", value);
}
//drain: Drains all elements from the HashSet, returning an iterator over the removed values.
for value in set.drain() {
    println!("Drained {}", value);
}
//len: Returns the number of values in the HashSet.
println!("Length: {}", set.len());
//is_empty: Checks if the HashSet is empty.
println!("Is empty: {}", set.is_empty());
//capacity: Returns the number of elements the HashSet can hold without reallocating.
println!("Capacity: {}", set.capacity());
//reserve: Reserves capacity for at least the specified number of additional elements.
set.reserve(10);
//shrink_to_fit: Shrinks the capacity of the HashSet as much as possible.
set.shrink_to_fit();
//retain: Retains only the elements specified by the predicate.
set.retain(|&value| value > 5);
//difference: Returns an iterator visiting the values that are in self but not in other.
for value in set.difference(&set1) {
    println!("difference: {}", value);
}
//intersection: Returns an iterator visiting the values that are both in self and other.
for value in set.intersection(&set1) {
    println!("intersection: {}", value);
}
//union: Returns an iterator visiting all values in self or other.
for value in set.union(&set1) {
    println!("union: {}", value);
}
//symmetric_difference: Returns an iterator visiting the values that are in self or other but not in both.
for value in set.symmetric_difference(&set1) {
    println!("symmetric_difference: {}", value);
}

}