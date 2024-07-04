/*
Arrays are used to represent a homogeneous collection of values. 
Similarly, a structure is another user defined data type available in Rust that allows us to combine data items of different types, including another structure. 
A structure defines data as a key-value pair.
The struct keyword is used to declare a structure. 
Since structures are statically typed, every field in the structure must be associated with a data type.

struct Name_of_structure {
   field1:data_type,
   field2:data_type,
   field3:data_type
}

Syntax - Initializing a structure

let instance_name = Name_of_structure {
   field1:value1,
   field2:value2,
   field3:value3
}; 

Accessing values in a structure
Use the dot notation to access value of a specific field.
instance_name.field1
 
 without Derive-debug

 use std::fmt;

struct Person {
    name: String,
    age: u32,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Person")
         .field("name", &self.name)
         .field("age", &self.age)
         .finish()
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{:?}", person);
}

*/
fn display( emp:&Employee){
    println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}
fn display_pass_by_value( emp:Employee){
    println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
}
fn who_is_elder (emp1:Employee,emp2:Employee)->Employee {
    if emp1.age>emp2.age {
        emp1
    } else {
       return emp2;
    }
 }
struct Employee {
    name:String,
    company:String,
    age:u32
 }
 struct Rectangle {
    width:u32, height:u32
 }
 
 //logic to calculate area of a rectangle
 impl Rectangle {
    fn area(&self)->u32 {
       //use the . operator to fetch the value of a field via the self keyword
       self.width * self.height
    }
 }
 struct Point {
    x: i32,
    y: i32,
 }
 impl Point {
    //static method that creates objects of the Point structure
    fn get_instance(x: i32, y: i32) -> Point {
       Point { x: x, y: y }
    }
    //display values of the structure's field
    fn display(&self){
       println!("x ={} y={}",self.x,self.y );
    }
 }

 #[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
 fn main() {
    let emp1 = Employee {
       company:String::from("CoffeeBeans"),
       name:String::from("Prakruthi"),
       age:22
    };
    println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);

    /*
    Modifying a struct instance
    To modify an instance, the instance variable should be marked mutable. 
    */

    let mut emp1 = Employee {
        company:String::from("CoffeeBeans"),
        name:String::from("Prakruthi"),
        age:24
     };
     emp1.age = 22;
     println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);
     /*Passing a struct to a function */

     display(&emp1);
     println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);

     let emp2 = Employee{
      company:String::from("TutorialsPoint"),
      name:String::from("Mohtashim"),
      age:50
    };
    let emp3 = Employee {
      company:String::from("TutorialsPoint"),
      name:String::from("Kannan"),
      age:32
    };
    let elder = who_is_elder(emp2,emp3);
    println!("elder is:");

   //prints details of the elder employee
    display_pass_by_value(elder);

    /*
    Methods are like functions. 
    They are a logical group of programming instructions. 
    Methods are declared with the fn keyword. T
    he scope of a method is within the structure block.

    Methods are declared outside the structure block. 
    The impl keyword is used to define a method within the context of a structure. 
    The first parameter of a method will be always self, which represents the calling instance of the structure. 
    Methods operate on the data members of a structure.

    To invoke a method, we need to first instantiate the structure. 
    The method can be called using the structure's instance.

     struct My_struct {}
    impl My_struct { 
     //set the method's context
    fn method_name() { 
      //define a method
        }
    }
     */

     let small = Rectangle {
        width:10,
        height:20
     };
     //print the rectangle's area
     println!("width is {} height is {} area of Rectangle  is {}",small.width,small.height,small.area());

     /*Static methods can be used as utility methods. 
     These methods exist even before the structure is instantiated. 
     Static methods are invoked using the structure's name and can be accessed without an instance. 
     Unlike normal methods, a static method will not take the &self parameter. 
     
    impl Structure_Name {
     //static method that creates objects of the Point structure
    fn method_name(param1: datatype, param2: datatype) -> return_type {
      // logic goes here
    }
     Syntax - Invoking a static method
     structure_name::method_name(v1,v2)
    
    */
   let p1 = Point::get_instance(10,20);
   p1.display();

   let person = Person {
      name: String::from("Alice"),
      age: 30,
  };

  // Print the Person instance using the {:?} format specifier
  println!("{:?}", person);

 }