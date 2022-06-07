//No matter how many functions are in a code, rust has a module system that helps the reuse of code in an organized fashion.
//modules is a collection of items such as Traits, Impl blocks, Enums, Structs, Functions, Type aliases, constants
//modules is a namespace that contains definitions of functions or types, the definitions in the module can be visible outside their module<public>  or not.
//modules are created with the mod keyword, to change definitions such as functions, types, constants and modules which are private by default to public the pub keyword is used.
//modules also help to provide privacy when we want to control which part of our code can access which part of a given module.
 
//What are packages and crates, and what exactly is their main purpose?
//Packages are one or more crates that contain a set of functionality, which has a cargo.toml that tells us how to build those crates.
//Crates is the smallest amount of code that a Rust compiler can work on. Can be a binary crate or a library crate.
//Binary crates compile to an executable that can be run.
//Library crates don't compile to an executable due to not having a main function.
//With Library crates functionalities intended to be shared with various projects are defined.
mod house{ //A module house that contains a function house_creation and structs.
    pub  fn house_creation(){
         struct Windows{
             front_windows:u8,
             back_windows:u8,
             side_windows:u8
            };
        struct HouseColor{
            front_view: String,
            side_view:String,
            rear_view:String
        };    
    }
 }
//modules let us organize code into groups for readability and easy reuse.This can be done with nested modules.
mod totalscore{
    pub mod maths_score{
        pub fn class1_score(){}
        fn class2_score(){}
    }
    mod english_score{
        fn class1_score(){}
        fn class2_score(){}
        //we can use the same name of functions because the functions are in different scopes.
 
    }
}
//rust uses 'path' to tell where an item is in a module tree.
//path takes two forms, absolute path and relative path
//Absolute path starts from crate root using a crate name for external crate or 'crate' from the current crate.
//Relative path starts from the current module and uses an identifier in the current module.
pub fn using_absolute_path(){
    crate::house::house_creation();
}
pub fn using_relative_path(){
    totalscore::maths_score::class1_score();
}
//Third party crates can be added to our project by adding the dependent crate name and version  we want to use in our Cargo.toml file.
//With the aid of the use keyword, rust brings the modules of the function our code wants to call into scope. 
mod text_processing {
    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|ref c| c.is_numeric()).count()
        }
    }
}
use text_processing::letters::count_letters(text);
pub fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = count_letters(text)
    let number_of_numbers:usize = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

