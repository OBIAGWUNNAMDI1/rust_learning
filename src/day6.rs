//In rust the use of reference presents a problem such as dangling pointer.
//Dangling pointer is when a reference still points to a resource that has been dropped by going out of scope.
//Rust helps to eliminate such with lifetimes.
fn main(){
    let x ;{
        let z = 43;
        x= &z; //this will give an error stating borrowed value does not live long enough
} //z dropped here while still borrowed 
    println!("x :{}", x); // borrow later used here.
longer_words();
string_slice();
}
//this error occurs because z is already dropped at the inner scope, and it can't be used at the outer scope because it's not valid.
// For Lifetime syntax the symbol is ' in <> to help declare lifetime
fn lifetime_in_functions<'a> (x:&'a String , y:&'a String) -> &'a String{ //this line implies that x, y and the return values are alive for the same scope.
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
fn longer_words(){
    let word1 = String::from("HelloWorld");
    let word2:String = String::from("Whatshername");
    let diff = lifetime_in_functions(&word1, &word2);
    println!("the longest word is {}", diff);
} 
//Slices in rust helps to reference a range of elements rather than the whole collection.
// In rust slice syntax is [starting_index..ending_index] 
fn string_slice(){
    let word = String::from("This is what I am saying");
    let slice_word1 = &word[0..4];
    let slice_word2 = &word[5..];
    println!("The sliced word is : {}", slice_word1);
    println!("The second sliced word is :{}", slice_word2);    
}
//slices are not only used in Strings for rust there are other rust collections such as array that makes uses of slices.