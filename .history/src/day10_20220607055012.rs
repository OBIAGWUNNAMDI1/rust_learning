//Handling of duplications in rust is done with the generics tool. Generics have multiple form over a given parameter. 
//That means a parameter that is generic can be of type integers, booleans, floating number all at the same time.
//Using single generic type parameter
//Generics creates definitions for function signature, structs, enums, which we can use with many concrete data structure.

struct NewGeneric<T>{
    x:T,
    y:T //Tje parameters x, y has to be the same type. 
}
fn main(){
    let boolean = NewGeneric{x:true, y:false}; //Implicitly specified type parameter. 
    let floating_number = NewGeneric{x:1.56 , y:9.6};
    let integer = NewGeneric{x:89, y:9000};
    using_generics(MultiGeneric{a:60 , b:false});

}
//Using multiple generic type parameters so the parameter values can be of different types.
struct MultiGeneric<Y, U>{
    a : Y,
    b: U
}
fn multi_gen(){
    let int_and_boolean = MultiGeneric{a:70, b:true};
    let float_and_string =MultiGeneric{a:8.6, b:String::from("boy")};
    let int_and_float = MultiGeneric{a:69 , b:6.899};
}
//Using Generics for functions.
fn using_generics<Y, U>(s:MultiGeneric<Y,U>){ // This is a generic function that takes an argument s of type MultiGeneric<Y,U>
    // This function is generic over T because MultiGeneric<Y,U> is preceded by '<T>'
}
//Using Generics for Method Definitions.
struct c
// You may be wondering when using generic type, there should be a runtime cost. 
//Using generic types won't make your code tun slower due to rust performing monomorphization of the code using generics at compile time. 