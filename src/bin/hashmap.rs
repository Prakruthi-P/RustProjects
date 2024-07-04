/*
HashMap

The HashMap structure is defined in the std::collections module.

Syntax
let mut instance_name = HashMap::new();

methods:

Creation
HashMap::new(): Creates an empty HashMap.
HashMap::with_capacity(capacity: usize): Creates an empty HashMap with a specified initial capacity.

Insertion and Updating
hashmap.insert(key, value): Inserts a key-value pair into the HashMap.
hashmap.entry(key): Returns an entry for the key, allowing for modification or insertion if the key does not exist.

Access
hashmap.get(&key): Returns a reference to the value corresponding to the key.
hashmap.get_mut(&key): Returns a mutable reference to the value corresponding to the key.
hashmap.get_key_value(&key): Returns references to the key and value as a tuple.

Removal
hashmap.remove(&key): Removes a key-value pair from the HashMap.
hashmap.clear(): Clears the HashMap, removing all key-value pairs.

Iteration
hashmap.iter(): Returns an iterator over the key-value pairs.

for (key, value) in map.iter() {
    println!("{}: {}", key, value);
}

hashmap.iter_mut(): Returns a mutable iterator over the key-value pairs.

for (key, value) in map.iter_mut() {
    *value += 1;
}

hashmap.keys(): Returns an iterator over the keys.

for key in map.keys() {
    println!("{}", key);
}


hashmap.values(): Returns an iterator over the values.
for value in map.values() {
    println!("{}", value);
}

hashmap.values_mut(): Returns a mutable iterator over the values.
for value in map.values_mut() {
    *value += 1;
}

hashmap.drain(): Removes all key-value pairs from the HashMap and returns an iterator over them.
for (key, value) in map.drain() {
    println!("Drained {}: {}", key, value);
}

Capacity and Size
hashmap.len(): Returns the number of elements in the HashMap.
println!("Length: {}", map.len());


hashmap.is_empty(): Checks if the HashMap is empty.
println!("Is empty: {}", map.is_empty());

hashmap.capacity(): Returns the number of elements the HashMap can hold without reallocating.
println!("Capacity: {}", map.capacity());


hashmap.reserve(additional: usize): Reserves capacity for at least the specified number of additional elements.
map.reserve(10);

hashmap.shrink_to_fit(): Shrinks the capacity of the HashMap as much as possible.
map.shrink_to_fit();


Miscellaneous
hashmap.contains_key(&key): Checks if the HashMap contains the specified key.
println!("Contains key: {}", map.contains_key("key"));

hashmap.retain(f: F): Retains only the elements specified by the predicate.
map.retain(|key, value| *value > 5);

hashmap.hasher(): Returns a reference to the hash builder used by the HashMap.
let hasher = map.hasher();


*/

use std::collections::HashMap;
fn main(){
    let mut state_codes = HashMap::new();
    state_codes.insert("KL","Kerala");
    state_codes.insert("MH","Maharashtra");
    println!("{:?}",state_codes);
    println!("size of map is {}",state_codes.len());
    match state_codes.get(&"KL") {
        Some(value)=> {
           println!("Value for key KL is {}",value);
        }
        None => {
           println!("nothing found");
        }
     }
     for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
     }
     state_codes.insert("GJ","Gujarat");
     if state_codes.contains_key(&"GJ") {
        println!("found key for Gujarat");
     }

     state_codes.remove(&"GJ");

     //with capacity
     let mut map: HashMap<String, i32> = HashMap::with_capacity(10);
     map.insert(String::from("key"), 10);
    


 }