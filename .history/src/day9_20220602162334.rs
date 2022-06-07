//The standard library consists of useful data structure called collections. 
//Collection is a data structure that can contain multiple values, the major difference between collections , array and tuple types, collections point are stored on the heap.
//With collections the amount of data does not need to be known at compile time, so collections data can grow or shrink as the program runs.

//Vectors Vec<T> helps us to store more than one value of the same type in a single data structure.
fn main(){
    reading_vectors();
}

fn create_vector(){
    {let mut v: Vec<u32> = Vec::new(); // to create an empty vector 
    let mut v = vec![1,2,3,4,5];
    // adding elements to a vector the push keyword is used.For us to update a vector the mut keyword has to be used. 
    v.push(6);
    v.push(7);
    println!("{:?}", v );    
}//println!("{:?}", v ); V not found in scope because v is freed out of it's scope.
}
fn reading_vectors(){
    //reading elements from vectors can be done with indexing syntax or with the get method.
    let x = vec![10,20,30,40,50,60];
    let use_index: &i32 = &x[2];
    let use_get :Option<&i32> = x.get(2);
    println!("Using index the index  is {:?}", use_index);
    println!("Using get method the second index is {:?}", use_get);
}
