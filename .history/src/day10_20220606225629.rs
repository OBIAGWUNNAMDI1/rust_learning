//Handling of duplications in rust is done with the generics tool. Generics have multiple form over a given parameter. 
//That means a parameter that is generic can be of type integers, booleans, floating number all at the same time.

struct NewGeneric<T>{
    x:T,
    y:T
}
fn main(){
    let boolean = NewGeneric{x:True, y:False};
    let floating_number = NewGeneric{x:1.56 , y:9.6};
    let integer = NewGeneric{x:89, y:9000};
}