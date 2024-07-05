/*
A logical group of code is called a Module. 
Multiple modules are compiled into a unit called crate. 
Rust programs may contain a binary crate or a library crate. 
A binary crate is an executable project that has a main() method. 
A library crate is a group of components that can be reused in other projects. 
Unlike a binary crate, a library crate does not have an entry point (main() method). 
The Cargo tool is used to manage crates in Rust. 
For example, the network module contains networking related functions and the graphics module contains drawing-related functions. 
Modules are similar to namespaces in other programming languages. 
Third-party crates can be downloaded using cargo from crates.io.

crate-Is a compilation unit in Rust; Crate is compiled to binary or library.
cargo-The official Rust package management tool for crates.
module-Logically groups code within a crate.
crates.io-The official Rust package registry.

public module

pub mod a_public_module {
   pub fn a_public_function() {
      //public function
   }
   fn a_private_function() {
      //private function
   }
}

private module

mod a_private_module {
   fn a_private_function() {
   }
}

Modules can be public or private. Components in a private module cannot be accessed by other modules. 
Modules in Rust are private by default. On the contrary, functions in a public module can be accessed by other modules.
Modules should be prefixed with pub keyword to make it public. 
Functions within a public module must also be made public.

Access to Private Methods or Functions
Within the Same Module: Private methods or functions (those without pub) can be accessed by other items within the same module.
From Nested Modules: Private items can also be accessed by nested modules within the same module hierarchy.
*/

pub mod movie {
    pub fn play(name:String) {
       println!("Playing movie {}",name);
    }
 }
 pub mod my_module {
    // Private function, not visible outside `my_module`
    fn private_function() {
        println!("This is a private function.");
    }

    // Public function, visible outside `my_module`
    pub fn public_function() {
        println!("This is a public function.");
        private_function(); // Private function can be accessed within `my_module`
    }

    // Nested module
    pub mod nested_module {
        // Accessing private function from the parent module
        use super::private_function;

        pub fn call_private_function() {
            private_function();
        }
    }
}
pub mod students{
    pub fn student_name(name:String){
        println!("The name of the student is {}",name);
    }
}
//nested modules
pub mod movies {
    pub mod english {
       pub mod comedy {
          pub fn play(name:String) {
             println!("Playing comedy movie {}",name);
          }
       }
    }
 }
use students::student_name;
use movies::english::comedy::play; 

 fn main(){
    movie::play(String::from("Bahuballi"));

    my_module::public_function();// Output: This is a public function.
    // Cannot access private_function directly from outside `my_module`
    // my_module::private_function(); // This would result in a compile-time error.

    // Accessing a function in a nested module that accesses the private function
    my_module::nested_module::call_private_function(); // Output: This is a private function.

    //The use keyword helps to import a public module.
    //use public_module_name::function_name;
    student_name(String::from("Ravi"));

    //nested modules 
       // short path syntax
    play("Herold and Kumar".to_string());
    play("The Hangover".to_string());

     //full path syntax
     movies::english::comedy::play("Airplane!".to_string());

 }