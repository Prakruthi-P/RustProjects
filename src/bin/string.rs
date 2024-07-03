/*
The String data type in Rust can be classified into the following −

=>String Literal(&str)
=>String Object(String)
 */

 fn print_literal(data:&str ){
    println!("displaying string literal {}\n",data);
 }
  fn main(){
    /*
    String Literal
    String literals (&str) are used when the value of a string is known at compile time. 
    String literals are a set of characters, which are hardcoded into a variable. 
     */
    let company:&str="TutorialsPoint";
    let location:&str = "Hyderabad";
    println!("company is : {} location :{}\n",company,location);

    /*
    String literals are static by default. 
    This means that string literals are guaranteed to be valid for the duration of the entire program. 
    We can also explicitly specify the variable as static as shown below −
     */
    let company:&'static str = "TutorialsPoint";
    let location:&'static str = "Hyderabad";
    println!("company is : {} location :{}",company,location);

    /*
    String Object
    The String object type is provided in Standard Library. 
    Unlike string literal, the string object type is not a part of the core language. 
    It is defined as public structure in standard library pub struct String. 
    String is a growable collection. It is mutable and UTF-8 encoded type. 
    The String object type can be used to represent string values that are provided at runtime. 
    String object is allocated in the heap.

    Syntax
    String::new()
    The above syntax creates an empty string

    String::from()
    This creates a string with some default value passed as parameter to the from() method.

     */
    let empty_string = String::new();
    println!("length is {}",empty_string.len());
 
    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());

    /*
   Sr	Method	        Signature & Description
   1	new()	           pub const fn new() → String	Creates a new empty String.
   2	to_string()	     fn to_string(&self) → String	Converts the given value to a String.
   3	replace()	     pub fn replace<'a, P>(&'a self, from: P, to: &str) → String	Replaces all matches of a pattern with another string.
   4	as_str()	        pub fn as_str(&self) → &str	Extracts a string slice containing the entire string.
   5	push()	        pub fn push(&mut self, ch: char)	Appends the given char to the end of this String.
   6	push_str()	     pub fn push_str(&mut self, string: &str)	Appends a given string slice onto the end of this String.
   7	len()	           pub fn len(&self) → usize	Returns the length of this String, in bytes.
   8	trim()	        pub fn trim(&self) → &str	Returns a string slice with leading and trailing whitespace removed.
   9	split_whitespace()	pub fn split_whitespace(&self) → SplitWhitespace	Splits a string slice by whitespace and returns an iterator.
   10	split()	        pub fn split<'a, P>(&'a self, pat: P) → Split<'a, P> , where P is pattern can be &str, char, or a closure that determines the split.	
                       Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.
   11	chars()	        pub fn chars(&self) → Chars	Returns an iterator over the chars of a string slice.
     */
    let mut z = String::new();
    z.push_str("hello");
    println!("{}",z);
    /*
    Illustration: to_string() 
    To access all methods of String object, convert a string literal to object type using the to_string() function.
    Also for type casting.
     */
    let name1 = "Hello TutorialsPoint , Hello!".to_string();
    println!("{}",name1);
    let number = 2020;
    let number_as_string = number.to_string(); 
    
    // convert number to string
    println!("{}",number_as_string);
    println!("{}\n",number_as_string=="2020");
     /*
      The replace() function takes two parameters − 
      the first parameter is a string pattern to search for and the second parameter is the new value to be replaced.
      all the matched pairs
      */

    let name1 = "Hello TutorialsPoint ,  Hello!";         //String object
    let name2 = name1.replace("Hello","Howdy");    //find and replace
    println!("{}",name2);

    /*The as_str() function extracts a string slice containing the entire string */
    let example_string = String::from("example_string");
    print_literal(example_string.as_str());

    /*The push() function appends the given char to the end of this String. */
    let mut company = "Tutorial".to_string();
    company.push('s');
    println!("{}",company);

    /*The push_str() function appends a given string slice onto the end of a String. */
    let mut company = "Coffee".to_string();
    company.push_str(" Beans");
    println!("{}\n",company);

    /*The len() function returns the total number of characters in a string (including spaces). */
    let fullname = "CoffeeBeans  !";
    println!("length is {}\n",fullname.len());

    /*The trim() function removes leading and trailing spaces in a string. 
    NOTE that this function will not remove the inline spaces. */
    let fullname = " CoffeeBeans Consultancy  \r\n";
    println!("Before trim ");
    println!("length is {}",fullname.len());
    println!();
    println!("After trim ");
    println!("length is {}\n",fullname.trim().len());
     
     /*The split_whitespace() splits the input string into different strings. 
     It returns an iterator so we are iterating through the tokens  */
     let msg = "CoffeeBeans has good work space".to_string();
     let mut i = 1;
     println!("{:?}",msg.split_whitespace());
     for token in msg.split_whitespace(){
        println!("token {} {}",i,token);
        i+=1;
     }

     /*
     The split() string method returns an iterator over substrings of a string slice, separated by characters matched by a pattern. 
     The limitation of the split() method is that the result cannot be stored for later use. 
     The collect method can be used to store the result returned by split() as a vector.
      */
      let fullname = "Kannan,Sudhakaran,Tutorialspoint";

      for token in fullname.split(","){
         println!("token is {}",token);
      }
   
      //store in a Vector
      println!("\n");
      let tokens:Vec<&str>= fullname.split(",").collect();
      println!("firstName is {}",tokens[0]);
      println!("lastname is {}",tokens[1]);
      println!("company is {}",tokens[2]);

      /*Individual characters in a string can be accessed using the chars method. Let us consider an example to understand this. */
      let n1 = "Tutorials".to_string();
      for n in n1.chars(){
         println!("{}",n);
      }

      /*concatenation */
      let n1 = "Tutorials".to_string();
      let n2 = "Point".to_string();
   
      let n3 = n1 + &n2; // n2 reference is passed
      println!("{}",n3);
      /*Another way to add to String objects together is using a macro function called format. */

      let n1 = "Tutorials".to_string();
      let n2 = "Point".to_string();
      let n3 = format!("{} {}",n1,n2);
      println!("{}",n3);
}