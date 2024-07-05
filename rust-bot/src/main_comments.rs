use std::io;
use std::cmp::Ordering;         
use rand::Rng;

fn main() {

    let rnumber: u32 = rand::thread_rng().gen_range(1..=10);   // Generate random number between 0 and 10 inclusive.
                                    
                                                // let mut guess will introduce a mutable variable named guess.
    let mut guess = String::new();              // Creating a variable to store user input.
                                                // let creates a variable, they are immutable by default .
                                                // to make mutable add mut .

    loop {


    io::stdin()                                 // We call the stdin function from the io module
        .read_line(&mut guess)                  // std::io::stdin represents a handle of the standard input for your terminal 
        .expect("Failed to read line!");        // We are passing &mut guess as the argument to read_line to tell it what 
                                                // string to store the user input. 
                                            
                                                // The & indicates that this argument is a refenrence,
                                                // which gives you a way to let multiple parts of your
                                                // code access one piece of data without needing to
                                                // copy that data into memory multiple times.
                                                // References are immutable by default. Hence you need
                                                // to write &mut guess rather than &guess to make it
                                                // mutable. 
                                            
                                                // We could have written this code as: 
                                                // io::stdin().read_line(&mut guess).expect("Failed to read line")
                                                // However its best to divide it. 
                                                
                                                // read_line puts whatever the user enters into the
                                                // string we pass to it, but also returns a Result
                                                // value. Result is an enumeration, often called an
                                                // enum. Result's variants are Ok and Err. 
                                                // The Ok variant indicates the operation was
                                                // successful, and inside Ok is the successfully
                                                // generated value. 
                                                // The Err variant means the operation failed, and Err
                                                // contains information about how or why the operation
                                                // failed. 

    let guess: u32 = guess.trim().parse().expect("Please type a number");   // The parse method will only work for
                                                                            // characters that can logically be
                                                                            // converted into numbers. 



    match guess.cmp(&rnumber){                      // Them cmp method compares two values and can be called on anything that 
        Ordering::Less => println!("Too small"),    // can be compared. 
        Ordering::Greater => println!("Too big"),   
        Ordering::Equal => println!("You win"),     //  We use match expression to decide what to do next based on which variant 
    }                                               //  of Ordering was returned from the call to cmp with the values in guess and 
                                                    //  secret number. 
                                                    //  Shadowing

                                                    
    println!("Random Number {rnumber}");                                                    
    }
                                                    //  A match expression is made up of arms. An
                                                    //  arms consists of a pattern to match agains,
                                                    //  and the code that should be run if the
                                                    //  value given to match fits that arm's
                                                    //  pattern. 
                                                    //  Rust takes the value given to match and
                                                    //  looks through each arm's pattern in turn. 
                                            
}
