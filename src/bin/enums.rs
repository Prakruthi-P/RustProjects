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
fn is_even(no:i32)->Option<bool> {
   if no%2 == 0 {
      Some(true)
   } else {
      None
   }
}

enum CarType {
   Hatch,
   Sedan,
   SUV
}
fn print_size(car:CarType) {
   match car {
      CarType::Hatch => {
         println!("Small sized car");
      },
      CarType::Sedan => {
         println!("medium sized car");
      },
      CarType::SUV =>{
         println!("Large sized Sports Utility car");
      }
   }
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

   /*Option Enum
   Option is a predefined enum in the Rust standard library. This enum has two values − Some(data) and None. 
   enum Option<T> {
   Some(T),      //used to return a value
   None          // used to return null, as Rust doesn't support  the null keyword
   }*/
   let result = is_even(3);
   println!("{:?}",result);
   println!("{:?}",is_even(30));

   //Match Statement and Enum

   print_size(CarType::SUV);
   print_size(CarType::Hatch);
   print_size(CarType::Sedan);

   //Match and Option

   match is_even(5) {
      Some(data) => {
         if data==true {
            println!("{} - Even no",5);
         }
      },
      None => {
         println!("{} -not even",5);
      }
   }

   //Match & Enum with Data Type

 }
 