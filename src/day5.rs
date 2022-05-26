// Ownership is an important concept in rust which helps in memory safety
// Ownership is checked at compile time.
//When an object passes out of the scope, it is destroyed and is no longer usable.
fn main() {
    move_variable(); //this will give an error of error[E0382]: borrow of moved value: `a`
    outside_of_scope(); // will give an error not found in scope. 
    copy_example();
    calc_string_len()
}
fn move_variable(){
    //In rust each object can only be used once. Once an object is moved to a new location, it can not be used in the old.
    let a = String::from("word");
    let b = a ;
    println!("the value of a is {}", a);
}
fn outside_of_scope(){
    {
        let z = 20;
    } let y = z ; //in this scope z is no longer valid.
}
fn copy_trait(input :u32){}
fn copy_example(){
//Rust allows scalar datatype to implement the copy trait instead of move. Scalar datatype like integers, floating number , booleans uses the copy trait.
//The reason is that types like integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make
    let n:u32 = 60 ;
    copy_trait(n); //ownership of the number in n is copied into copy_trait.
    //n can be used again since it was copied and not moved 
}
//some types such as string doesn't use the copy trait. To copy the heap data of strings, and not stack data we use the "clone" keyword in rust, which duplicates memory and produces a new value. 

fn clone_trait(value: String){}
fn clone_example(){
    let word = String::from("Hello how are you?");
    clone_trait(word.clone()); // this passes another value cloned from word
    clone_trait(word); // word was never moved so it can still be used. 
}
// Rust uses reference to help us allow functions and other variables to use certain data  without fully owning it
// The ampersands & sign signifies references which helps us to borrow data.
//Borrowing works by taking reference of the original variable and use it to access the data.
//The original variable retains ownership during borrow and afterwards, also a borrow is over when a reference goes out of scope.
// A borrowed data has limitations because we can't do everything we can do with a fully owned value. A borrowed data cannot be modified because references are immutable. 

fn ref_value(s: &String) -> usize {// is a reference to a String
    s.len()
} // s goes out of scope.
fn calc_string_len(){
    let s2 = String::from("good morning");
    let length = ref_value(&s2);
    println!("The byte length of '{}' is {}", s2 ,length);
}
//Mutable references can be created with  &mut keyword. This allows us to modify data.
//mutable references have restrictions that there can only be one mutable reference to a particular data in a scope. 
//Also in a scope mutable reference and immutable references can not be mixed.It will give an error.
fn mut_ref(word:&mut String){
    word.push_str(", students");
}
fn use_mut_ref(){
    let mut greetings = String::from("Good morning Class A");
    mut_ref(&mut greetings);    
}
