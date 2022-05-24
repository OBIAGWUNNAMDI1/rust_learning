fn main() {
    // in rust to declare functions the keyword fn is used. lso the main function. rust used functions as the basis of code reusability. 
    //to define a function we start with fn, the function name, list of arguments in parenthesis,  an optional return type , and the function body.
    //function names in rust uses snake case as conventional style, all letters are lowercase and underscore separates the word. 
    let word:String=String::from("this is a new word");
    change_to_upper_case(word); 
    labeled_values(1000, 'm');
    let z = multiply_number(25, 50);
    println!( "The value of the multiplication is {}", z);
  }
  
  fn change_to_upper_case(input:String ) 
   {
    let mut output =input.to_uppercase();
    output.push('!');
    println!("The upper_case output is  :{}",output);
  }
  
  //functions with more than one arguments. 
  fn labeled_values(value:u32 , label: char){
    println!("The road measurement is {}{}", value , label);
  }
  
  //functions that returns a value. In rust functions exactly return one value and you declare the type after an arrow.
  //The last line of the function determines what is returned and the last line doesn't contain a semicolon.
  fn multiply_number(x:i32, y:i32) -> i32 {
    x * y
  
  }
  
  
  
  