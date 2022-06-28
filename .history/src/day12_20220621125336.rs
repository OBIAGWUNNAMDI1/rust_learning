                    //ERROR HANDLING IN RUST.
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
    let colors = vec!["Pink", "Blue", "Yellow", "Purple", "Red"];
    let first = colors.get(0); //picking the first item 
    println!("{:?}", first);
    let third = colors.get(2); //picking the second item.
    println!("{:?}", third);
    let non_existent = colors.get(99); //picking the non existent index. 
    println!("{:?}", non_existent);
    }
//Catching errors with match patterning. 
//match is used to provide patterns, and runs the code that is supplied with that pattern. Also the provided patterns helps to control the flow of code.
fn using_match_patterns(){
    let colors = vec!["Pink", "Blue", "Yellow", "Purple", "Red", "Black", "Green", "White"];
    for &color in [0,1,3,5,7,9].iter(){
        match colors.get(color){
            Some(color_name) => println!("I love this {} car", color_name),
            None =>println!("The colour index isn't available."),

        }
    }
}
//Recovering errors with Result enum. 
// Result<T,E> is used for returning and propagating errors with a variant Ok(T) for success, where there is a value and Err(E) as error, and when there is an error value. 
//The result Type has a lot og methods which helps to handle and propagate errors. result.unwrap, result.
fn using_Result(){
    let 

}
fn main(){
    using_panic("blue");
    using_panic("red"); 
    //thread 'main' panicked at 'I dont like this colour', src/main.rs:12:10 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    panic_backtrace();
    colour_checker();
    // The output is Some("banana") Some("coconut") None.
    //The non existent variable gave an output of None due to the fact there is an absence of the index 99 , instead of it panicking. 
    using_match_patterns();
}
