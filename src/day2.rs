use std::mem;
fn main (){
    //datatype are important in Rust because Rust is a statically typed language that means it must know the type of variables at compile time.
    //There are two data types subsets: Scalar types and Compound types.
    //Scalar types represents a single value and can be further divided into four: integers, floating-point numbers , booleans and characters.

    //Integer Types: It is a number without fractional part. There are unsigned integer,
    //which are only positive number and the signed integers which have both negative and positive numbers.
    //Integers types have bit length ranging from 8, 16 , 32, 64, 128 and arch which depends on the architecture of the computer the program is running on.
    //Signed variables can store numbers from -(2n - 1) to 2n - 1 - 1 where n is the number of bits. So an i8 can store numbers from -128 to 127.
    // In rust there is integer overflow, a variable of type u8 tht holds value between 0 and 255 when assigned a value of 298 overflow occurs. 

    let x : u8 = 125;
    let y : i8 = -60;
    let x_1 : u16 = 4000;
    let y_1 : i16 = -1500;
    let x_2 : u32 = 567577;
    let y_2  : i32 = -200000;


    //Floating-Point Types: This are numbers with decimal points and are f32 and f64 which are 32 and 64 bit size. Floating point types are signed.
    let f_1 :f64 = 5.7767685;

    // Numeric Operations Rust supports the basic maths operations for all number types.Integer division rounds down to the nearest integer. 
    let floor = 2 / 3 ; // results in zero

    //Boolean Types are one byte in size and has two possible values , true or false. The main use of booleans is through conditionals.
    let t : bool = false ;

    //The Character Type the char type is the alphabetic type and are specified with single quotes as opposed to string 
    //literals which uses double quotes. char type is four bytes in size  and represent a Unicode Scalar Value, that is it can represent a lot more than just ASCII. 

    let c = 'z';

    //Compound Types groups multiple value into one type. In rust there are two primitive types arrays and tuples.

    //Tuples groups variety of types into a compound type. 
    let tup :(i32, f64, u8, bool) = (-500 , 3.67, 90, false); 

    //tuples values can be destructured with the help of pattern matching
    let (e,f,g, h) = tup;
    println!("The value of e:{}", e);
    println!("The value of f:{}", f);
    println!("The value of g:{}", g);
    println!("The value of h:{}", h);
    //values can be extracted using tuple indexing with '.' 
    println!("The second value in the tuple: {}", tup.1);
    println!("The first value in the tuple: {}", tup.0);

    //Array Type. This are collection of multiple values of the same type. Arrays have a fixed length
    let arr:[u8; 4] =[56,34,67,78];
    //array type can be accessed using indexing.
    let first = arr[0];
    println!("The first value of the array is:{}", first);
    // arrays  are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&arr));
  //Slices  can help to point to a section  of an array [starting_index .. ending_index]
   let cut_out_slice = &arr[1 ..3 ];

}
