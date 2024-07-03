
/*
The println! macro takes two arguments −
->A special syntax {}, which is the placeholder
->The variable name or a constant

Integer Datatype 

Sr Size	    Signed	Unsigned
1	8 bit	 i8	    u8
2	16 bit	 i16	u16
3	32 bit	 i32	u32
4	64 bit	 i64	u64
5	128 bit	 i128	u128
6	Arch	 isize	usize
*/
fn main() {
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';   
    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    //Integer overflow

   // 0 to 255 only allowed for u8
    let weight:u8 = 25;   //overflow value is 0
    let height:u8 = 25;   //overflow value is 1
    let score:u8 = 25;    //overflow value is 2

    println!("\nage is {} ",age);
    println!("weight is {}",weight);
    println!("height is {}",height);
    println!("score is {}\n",score);

    println!("result value is {}",result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}\n",mark,count);                 //unicode character type
    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}\n",icon_char);

    //Float data type in Rust can be classified as f32 and f64. 
    //The f32 type is a single-precision float, and f64 has double precision. 
    //The default type is f64. Consider the following example to understand more about the float data type.

    let result = 10.00;        //f64 by default
    let interest:f32 = 8.35;
    let cost:f64 = 15000.600; 
    println!("result value is {}",result);
    println!("interest is {}",interest);
    println!("cost is {}",cost);

    //Automatic type casting is not allowed in Rust. 
    //Consider the following code snippet. 
    //An integer value is assigned to the float variable interest.

    let interest:f32 = 8.0;   // integer assigned to float variable
    println!("interest is {}\n",interest);

    /*Number Separator
    For easy readability of large numbers, we can use a visual separator _ underscore to separate digits. 
    That is 50,000 can be written as 50_000 . This is shown in the below example.*/

    let float_with_separator = 11_000.555_001;
    println!("float value {} ",float_with_separator);
   
    let int_with_separator = 50_000;
    println!("int value {}\n",int_with_separator);

    /* Boolean
    Boolean types have two possible values – true or false. Use the bool keyword to declare a boolean variable.
    */
    let isfun = true;
    println!("Is Rust Programming Fun ? {}\n",isfun);

    /* Character
    The character data type in Rust supports numbers, alphabets, Unicode and special characters.
    Use the char keyword to declare a variable of character data type. 
    Rust’s char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. 
    Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    */
    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
   
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}\n",emoji);
 }