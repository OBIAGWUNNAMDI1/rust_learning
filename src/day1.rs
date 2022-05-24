fn main() {
    // variables in rust are immutable
    let x = 5 ;
    println!("The value  of x is:{}" , x);
    x = 6 ;
    println!("The value  of x is:{}" , x);

    // to make a variable mutable you use the "mut" keyword

    let mut x = 10 ;
    println!("The value  of x is:{}" , x);
    
    x = 6;
    println!("The value  of x is:{}" , x);

    // constant are values that are immutable and are bounded to constant expression and not something that could only be computed at runtime.
    //constants are immutable they can not become mutable with the "mut" keyword

    const mins_in_seconds: u32 = 60;

    //shadowing is when the same name is used to create a new variable. So when the variable is used the program sees the second variable's value.

    //shadowing can also allow a change of the value type with the keyword let, which allows us to effectively create a new variable.

    let x = 20 ;

    let x = x + 1 ;
    {
        let x = x * 10 ;
        println!("The value of x in the inner scope is: {}" , x );
    }
    println!("The value of x in the outer scope is: {}", x);

}