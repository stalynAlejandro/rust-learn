fn main() {
    let s = String::from("hello");          // s comes into scope

    takes_ownership(s);                     // s value moves into function
                                            // and is no longer valid here
                                            
    //println!("{s}");                      // borrow of moved value : `s` value borrwed here

    let x = 5;                              // x comes into scope

    makes_copy(x);                          // x would move into function
                                            // but i32 is Copy, so it's okay to still
                                            // use x afterward
}

fn takes_ownership(some_string: String){    // some_string comes into scope
    println!("{some_string}")               // 
}                                           // some_string goes out scope
                                            // drop is called. The backing memory is free

fn makes_copy(some_integer: i32){           // some_integer comes into scope
    println!("{some_integer}")              //
}                                           // some_integr goes out of scope. 
                                            // Nothing special happens.


// Return Values and Scope ------------------------------------------------------------
fn main2 () {
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into function, which also moves
                                        // its return value into s3


}                                       // Here s3 goes out of scope and is dropped. 
                                        // s2 was moved, so nothing happens.
                                        // s1 goes out of scope and is dropped.

fn gives_ownership() -> String {                // function will move its return value
                                                // into the function that calls it 
                                                //
    let some_string = String::from("yours");    // some_string comes into scope 
                                                //

    some_string                                 // some_string is returned and moves out
                                                // to the calling function
}


fn takes_and_gives_back(a_string: String) -> String{    // a_string comes into scope
    a_string                                            // a_string is returned and 
                                                        // moves out to the calling 
                                                        // function
}
