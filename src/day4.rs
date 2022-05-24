fn main () {
  //control flow is an important fundamental of any programming language and rust is no different. 
  //if statements helps us to do actions depending on conditions. A condition is provided and if it met, run this bock of code, if not met do not run the code block.
  //In rust the if must be explicitly and always provided with a boolean as it condition.
test_conditions();
more_test_conditions();
hash_map_creation();
while_loop_usage();
for_loop();

}
fn test_conditions() {
  let x = 10;
  if x < 10{
    println!{"condition was true"};
  }else{
    println!("condition was false");
  }
}
//else if is used when there are more than one condition test, and else is used if you want your program to do something in the false condition.
fn more_test_conditions(){ // this block of code compares the length between two words. 
  let word = "this is life".len();
  let word2 = "this is not life".len();
  if word == word2{
    println!("the length of the words are the same");
  }
  else if word > word2 {
    println!("the length of word is greater than word2")  
  }
  else{
    println!("the length of word is less than word2")
  }
}

//Hash Maps are standard collections module used to store data by mapping a key to it's corresponding value. In rust you have to state the datatype of the key and value pair.
// A hashmap with string and u32 as its key and value pair can be written as HashMap<String , u32>
//instantiating a new HashMap instance 
use std::collections::HashMap;
fn hash_map_creation() {
  let mut name_age: HashMap<String , u8> = HashMap::new();
  name_age.insert(String::from("Peter"), 56);
  name_age.insert(String::from("Mayo"), 12);
  name_age.insert(String::from("David"), 30);

  //accessing HashMap Values with its key 
  let peter_age = name_age.get("Peter");
  println!("The age of Peter is: {:?}" ,peter_age);

  //using an if statement to check if a value exists in a HashMap
  if name_age.contains_key("David"){
    println!("David:{:?}", name_age.get("David"))
  }
  else{
    name_age.insert(String::from("David") , 25);
  }
  //Iterating over a HashMap to get the keys and values.
  for (key, value) in name_age.iter(){
    println!("That man  {} is {} years", key, value);
  }
}
// Loops help us to make repeatable block of code and rust offers three loop expressions: loop , while , for.
// A loop consists of a condition and a code block containing the execution code.
fn while_loop_usage(){
  // while loop repeats as long as the conditional expression remains true. it stops when the conditional expression become false. 

  let mut count = 0 ;
  while count < 10 {
    count = count * 4;
    println!("count:{}", count);
    count += 1;

  }
}
fn using_loop_keyword(){
  // rust uses the loop keyword to create an infinite loop, which can be stopped if the programmer uses the Ctrl +C to halt execution or use the break keyword to set a break point.
  loop{
    println!("We loop infinitely");
   break;
  }

}
fn for_loop(){
  // for loop uses an iterator to process a collection. That means an action is taken for each of the item in the collection until the item finishes.
  // In rust iteration over collection type such as array, vector or has map is allowed.
for na in 1..102{// a..b is a range notation where a is inclusive and b is exclusive.
  if na % 4 == 0{
    println!("divisible by 4");
  }
  else if na % 8 == 0{
    println!("divisible by 8")
  }
  else if na % 12 == 0 {
    println!("divisible by 12");
  }
  else{
    println!("other numbers{}", na);
  }
}
}