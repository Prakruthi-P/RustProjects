/*
Traits
Traits can be used to implement a standard set of behaviors (methods) across multiple structures. 
Traits are like interfaces in Object-oriented Programming. The syntax of trait is as shown below −

Declare a Trait
trait some_trait {
   //abstract or method which is empty
   fn method1(&self);
   // this is already implemented , this is free
   fn method2(&self){
      //some contents of method2
   }
}
Traits can contain concrete methods (methods with body) or abstract methods (methods without a body). 
Use a concrete method if the method definition will be shared by all structures implementing the Trait. 
However, a structure can choose to override a function defined by the trait.

Syntax - Implement a Trait
impl some_trait for structure_name {
   // implement method1() there..
   fn method1(&self ){
   }
}
*/

fn main(){
    //create an instance of the structure
    let b1 = Book {
       id:1001,
       name:"Rust in Action"
    };
    b1.print();

    //greet trait
    let person = Person { name: String::from("Alice") };
    let person1 = Person { name: String::from("Person 2") };

    person.greet(); // This calls the greet method defined in the Greet trait.
    print_greeting(person1);

 }
 //declare a structure
 struct Book {
    name:&'static str,
    id:u32
 }
 //declare a trait
 trait Printable {
    fn print(&self);
 }
 //implement the trait
 impl Printable for Book {
    fn print(&self){
       println!("Printing book with id:{} and name {}",self.id,self.name)
    }
 }

 trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

/*
Trait Bounds

Traits can also be used as bounds to specify that a generic type must implement a particular trait. 
This ensures that the generic type can use the methods defined in that trait.
*/

fn print_greeting<T: Greet>(greetable: T) {
    greetable.greet()
}
/*
In this example:

fn print_greeting<T: Greet>(greetable: T) is a generic function that takes a parameter greetable of type T.
T: Greet is a trait bound that specifies T must implement the Greet trait.
The function print_greeting can call the greet method on greetable because it knows that T implements the Greet trait.

*/