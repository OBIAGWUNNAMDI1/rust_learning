//A traits tells a group of type what functionalities it can implement.
//It is a collection of methods that is defined for a type, which represents the behavior of its implementor. The implementor behavior tells us what methods that can be called.
// A trait is defined by using the ''trait'' keyword and the trait name. Traits mostly only contains method signatures such as functions without definitions.
pub trait Area{
    fn area(&self) -> f64; // Example of trait definition.  The trait was declared pub so that other crates depending on this crate can make use of the trait.
    //inside the curly bracket th method signature that describes the type behavior that implements this trait is a function signature.     
}
//The reason why there is no implementation is because each type implementing this trait gives provision for its own custom behavior for the method. 
//Using Trait on a type
pub struct Circle{
    rad: f64,
} 
pub struct Triangle{
    breadth:f64,
    height :f64,
}
impl Area for Circle{
    fn area(&self) ->f64{
        use std::f64::consts::PI ; //using the constant PI from the standard library
        PI *self.rad.powf((2.0))
    }
}
impl Area for Triangle{
    fn area(&self) -> f64{
        (self.breadth * self.height) / 2.0
    }
}
fn calc_area(){
    let circle = Circle{rad: 6.9};
    let triangle = Triangle{breadth: 8.4, height}
}