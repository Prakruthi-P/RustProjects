/*
In Rust programming, when we have to select a value from a list of possible variants we use enumeration data types. 
An enumerated type is declared using the enum keyword. Following is the syntax of enum −

enum enum_name {
   variant1,
   variant2,
   variant3
}

*/
// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
    Male,Female
 }

// The `derive` attribute automatically creates the implementation required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct Person {
   name:String,
   gender:GenderCategory
}
 fn main() {
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;
 
    println!("{:?}",male);
    println!("{:?}",female);

    //Struct and Enum
    let p1 = Person {
      name:String::from("Mohtashim"),
      gender:GenderCategory::Male
   };
   let p2 = Person {
      name:String::from("Amy"),
      gender:GenderCategory::Female
   };
   println!("{:?}",p1);
   println!("{:?}",p2);
 }
 