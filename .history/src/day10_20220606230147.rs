//Handling of duplications in rust is done with the generics tool. Generics have multiple form over a given parameter. 
//That means a parameter that is generic can be of type integers, booleans, floating number all at the same time.
//Using single generic type parameter
struct NewGeneric<T>{
    x:T,
    y:T
}
fn main(){
    let boolean = NewGeneric{x:True, y:False};
    let floating_number = NewGeneric{x:1.56 , y:9.6};
    let integer = NewGeneric{x:89, y:9000};
}
//Using multiple generic type parameters so the parameter values can be of different types.
struct MultiGeneric<Y, U>{
    a : Y,
    b: U
}
fn multi_gen(){
    let int_and_boolean = MultiGeneric{a:70, b:true};
    let float_string =MultiGeneric{a:8.6}
}