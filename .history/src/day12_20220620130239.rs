//In rust errors are grouped into two; the recoverable and irrecoverable errors.
//Recoverable errors are errors that can be related as a result of absence of files, values. The option<T> enum is used when an error is a recoverable error.
//Unrecoverable errors are errors that can be related to accessing a location of an array that has left it's scope, attempting to access and index that's not present. 
//The panic! macro is used for an unrecoverable error and stops the program.
//** Unrecoverable Errors with panic!
//panic! macro is commonly invoked when the compiler sees a bug, and doesn't know how to handle the bug at the time of writing our code.
// panic! executes by printing a failure message, free resources and exits program.
fn using_panic(car_colour: &str){ //calling the panic! macro directly. 
    if car_colour == "blue"{
        println!("{} is the colour of the car ", car_colour)
    } 
    else{panic!("I dont like this colour")};
}
fn panic_backtrace(){ // when a panic! call is as a result of a bug in our code. 
    let number = vec![1,2,3,4,5,6,7];
    number[2];
    number[8]; // we are accessing the 9th element of our vector but the vector has only seven elements. So rust will panic an print an error message. 
}
//**Recoverable errors with Option<T> */
//The Option<T> enum is used tp deal with values that might exist or might not exist.
//Option<T> enum has two variant the none and some.
fn colour_checker(){
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    
    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);
    
    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);
    
    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);
    }

fn main(){
    using_panic("blue");
    using_panic("red"); 
    //thread 'main' panicked at 'I dont like this colour', src/main.rs:12:10 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    panic_backtrace();
}
