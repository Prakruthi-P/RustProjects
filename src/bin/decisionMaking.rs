fn main() {
    let num = 2 ;
    if num > 0 {
       println!("{} is positive",num);
    } else if num < 0 {
       println!("{} is negative",num);
    } else {
       println!("{} is neither positive nor negative",num) ;
    }
    /*Match Statement
    The match statement checks if a current value is matching from a list of values, this is very much similar to the switch statement in C language. 
    In the first place, notice that the expression following the match keyword does not have to be enclosed in parentheses. 
    
    The syntax is as shown below.

    let expressionResult = match variable_expression {
    constant_expr1 => {
      //statements;
    },
    constant_expr2 => {
      //statements;
    },
    _ => {
      //default
        }
    };
    */
    let state_code = "Mo";
    let state = match state_code {
       "MH" => {println!("Found match for MH"); "Maharashtra"},
       "KL" => "Kerala",
       "KA" => "Karnadaka",
       "GA" => "Goa",
       _ => "Unknown"
    };
    println!("State name is {}",state);
 }