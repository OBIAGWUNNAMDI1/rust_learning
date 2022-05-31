//Rust have various complex data types and one way to create it is with the help of Struct. 
//Struct allows us to combine one of more simple datatype into a unified datatype.
//Struct fields are private by default and can only be accessed within the module where the struct is declared.
//Struct fields can be made public with "pub" keyword.
struct Car{
    engine:u8,
    door:u8,
    automatic:bool,
    color :String,
}
fn main(){
    car_make();
    create_enum();
}
fn car_make(){
    //Rust does not support field mutability at the language level, so you cannot write something like this:struct Point {mut x: i32, // This causes an error y: i32}
    let mut car1 = Car{
        engine: 8,
        door: 4,
        automatic:true,
        color:String::from("Blue"),    
    };
    // a specific value from a struct can be gotten with the use of dot notation. 
    println!("The new car is a {} door {} color car with {} engine and {} automatic", car1.door, car1.color,car1.engine, car1.automatic); //The new car is a 4 door Blue color car with 8 engine and true automatic

    // If the struct instance is mutable with the keyword mut, we can change a value from the struct. Mutability is property of the binding not the structure itself.
    car1.automatic =false; 
    car1.engine = 4; 
    println!("The new car is a {} door {} color car with {} engine and {} automatic", car1.door, car1.color,car1.engine, car1.automatic); //The new car is a 4 door Blue color car with 4 engine and false automatic

    // A struct can updated with the help of an initialized struct
    let car2 =Car{
        engine:6,
        door: 2,
        ..car1
    };
    println!("The new car is a {} door {} color car with {} engine and {} automatic", car2.door, car2.color,car2.engine, car2.automatic);
    //Tuple Structs are struct variants with a name but no named field. struct color(String, String, String)
}

// Enum helps to represent some data that may be one of several variants. 
#[derive(Debug)]
//The #[derive(Debug)] helps us to see certain values that aren't otherwise viewable in standard output. 
// with the use of syntax {:#?} it aids to view debug data and to format data in a readable manner  with the printIn! macro.
enum PhoneShop{
    //An enum can be like a unit struct without fields o data types.
    ShopName(String),
    //An enum variant can be like a tuple struct with data types but no named field
    PhoneTypes(String, String, String),
    // An enum variant can be like a classic struct with named fields and their data types.
    PhoneAvailable{phone_brand:String ,no_available:u32}
}
//initializing an enum. 
fn create_enum(){
    // we use the syntax <enum>::<variant> to create instances of enum variants. 
    let shop_name = PhoneShop::ShopName(String::from("Boyo"));
    let phone_types = PhoneShop::PhoneTypes(String::from("Nokia"),String::from("Samsung"),String::from("Iphone"));
    let phone_available = PhoneShop::PhoneAvailable{phone_brand:String::from("Nokla"), no_available: 100 };
    println!("\n PhoneShop enum structure: \n\n{:#?} \n\n{:#?} \n\n{:#?}", shop_name, phone_types, phone_available);
}

