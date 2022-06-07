//No matter how many your functions are in a code, rust has a module system that helps the reuse of code in an organized fashion.
//modules is a collection of items such as Traits, Impl blocks, Enums, Structs, Functions, Type aliases, constants 
//module is a namespace that contains definition of functions or types, the definitions in the module can be visible outside their module<public>  or not.
//modules are created with the mod keyword, to change definitions such as functions, types, constants and modules which are private by default to public the pub keyword is used

//What are packages and crates and what exactly are their main purpose?
//Packages are one or more cates that contains set of functionality, which has a cargo.toml that tells us how to build those crates.
//Crate is the smallest amount of code that a Rust complier can work on. can be a binary crates or a library crates.
//Binary crates compile to an executables that can be run.
//Library crates don't compile to an executable due to not having a main function. 
//With Library crates functionalities intended to be shared with various projects are defined.
 mod house{
     fn house_creation(){
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
    mod maths_score{
        fn class1_score(){}
        fn class2_score(){}
    }
    mod english_score{
        fn class1_score()
    }

}