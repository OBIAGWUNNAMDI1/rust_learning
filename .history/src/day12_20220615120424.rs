//In rust errors are grouped into two; the recoverable and irrecoverable errors.
//Recoverable errors are errors that can be related as a result of absence of files, values. The option<T> enum is used when an error is a recoverable error.
//Unrecoverable errors are errors that can be related to accessing a location of an array that has left it's scope, attempting to access and index that's not present. 
//The panic! macro is used for an unrecoverable error and stops the program.
//** Unrecoverable Errors with panic!
//panic! macro is commonly invoked when the compiler sees a bug, and doesn't know how to handle the bug at the time of writing our code.
// panic! executes by printing a failure message, free resources and exits program.
fn using_panic(car_colour: &str){
    if car_colour == "blue"{
        println!("{} is the colour of the car ", car_colour)
    } 
    else{panic!("I dont like this colour")};
}
fn panic_backtrace()
fn main(){
    using_panic("blue");
    using_panic("red"); 
    //thread 'main' panicked at 'I dont like this colour', src/main.rs:12:10 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


}
