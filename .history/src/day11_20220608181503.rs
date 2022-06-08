//A traits tells a group of type what functionalities it can implement.
//It is a collection of methods that is defined for a type, which represents the behavior of its implementor. The implementor behavior tells us what methods that can be called.
// A trait is defined by using the ''trait'' keyword and the trait name. Traits mostly only contains method signatures such as functions without definitions.
pub trait Area{
    fn area(&self) -> f64; // Example of trait definition.  The trait was declared pub so that other crates depending on this crate can make use of the trait.
    //inside the curly bracket th method signature that describes the type behavior that implements this trait is a function signature.     
}
//The reason why there is no implementation is because each type implementing this trait gives provision for its own custom behavior for the method. 
//Using Trait on a type. 
//To implement a trait for a type the keyword "impl trait for Type" where traits is the name of the trait been implemented and Type is the name of the struct or enum. 
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
        PI *self.rad.powf(2.0)
    }
}
impl Area for Triangle{
    fn area(&self) -> f64{ //Using the method signature the trait definition defined within the impl block.
        (self.breadth * self.height) / 2.0
    }
}
fn calc_area(){
    let circle = Circle{rad: 6.9};
    let triangle = Triangle{breadth: 8.4, height: 18.9};
    println!("The area of the circle is :{}",circle.area());
    println!("The area of the triangle is : {}", triangle.area());
}
fn main(){
    calc_area();
}
//Traits can be used as parameters for a functions and can be used to constrain generic types. 
trait Authentication{
    fn auth(&self) -> String;
}
fn auth_(word: &impl Authentication){
    println!("The user information is {}" word.auth());
}
pub struct User1 {
        name:String,
        age: u64,
        fav_color : String,
    }
pub struct Car{
    name:String,
    drive: u64,
    vcolou
}    
impl Authentication for User{
    fn auth(&self) -> User{
        self.username.to_string();
        self.password_hash
    }

}  
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }    
    }
    fn hash_password(input: &str) -> u64 { /*...*/ }
}

fn main() {

    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.username);
    println!("The password is: {}", user.password_hash);

}
