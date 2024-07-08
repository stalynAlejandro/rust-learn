# The Rust Programming Language

https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## Crates.io
https://crates.io/

## Rust bot
https://www.youtube.com/watch?v=2CLHXQTR3d4&t=5494s

## Hello, Cargo!
Cargo is Rusts's build system and package manager.
Cargo handles a lot of tasks for you, suchs as building your code, downloading the libraries your code depends on, and building those libraries. **dependencies**.

## Building and Running a Cargo Project

`cargo build` this command creates an executable file in `target/debug/hello_cargo` rather than in your current directory. Because the default build is a debug build, Cargo puths the binary in a directory named `debug`. You can run the executable with this command: `./hello-cargo`

We can also use `cargo run` to compile the code and then run the resultant executable all in one command:


Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn't produce an executable.

`cargo check` is much faster than `cargo build` because it skips he step of producing an executable. If you're continually checking your work while writing the code, using `cargo check` will speed up the process of letting you know if your project is still compiling.
- `cargo new`. Create a project.
- `cargo build`. Build project.
- `cargo run`. Build and Run project.
- `cargo check`. Build without producing a binary.
- `cargo build --release`. Compile project with optimizations. Create a executable `target/release`

When we include an external dependency. Cargo fetches the latest version of everything that dependency needs from the *registry*, which is a copy of data from **Crates.io**.

**Crates.io** is where people in the Rust ecostystem post their open source Rust projects for others to use.


By default Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the *prelude*.

If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a `use` statement. Using `std::io` library provides the ability to accept user input.

`println!` is a macro that prints a string to the screen.

We use `let` statement to create the variable. `let apples = 5`.

Rust variables are immutable by default, meaning once we give the variable a value, the value won't change.
To make a variable mutable, we add `mut` before name: `let mut bannanas = 5`.

`String::new` returns a new instance of a String (type provided by the standard library) is growable.

The `::` syntanx indicates that `new` is an associated function of the `String` type.

An *associated function* is a function that's implemented on a type, in this case `String`.

This `new` function creates a new, empty string. `new` it's a common name for a function that makes a new value of some kind.

`let mut guess = String::new();` created a  mutable variable that is currently bound to new, empty instance of aString.

`.read_line(&mut guess)` calls the read_line method. We are passing `&mut guess` as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without ovewriting its contents), so we therefore pass that string as an argument. The **argument needs to be mutable so the method can change the string's content**.

The `&` indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References ara immutable by default. Hence you need to write `&mut guess` to make it mutable.


### Ensuring Reproducible Build with the Cargo.lock File

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code.

When you build a project for the first time, Cargo figures out the versions of the dependencies and then writes them to the *Cargo.lock* file.

This lets you have reproducible build atomatically. In other words, your project will remain at specific version until you explicitly upgrade.

## Upgrade a Crate to Get a New Version

`update` will ignore the *Cargo.lock* file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.

To use `rand` version 0.9.0 you'd have to update the Cargo.toml


## Variables and Mutability

Variables are immutable.

## Data Types

Every value in Rust is of a certain *data type*, which tells Rust what kind of data is beign specified so
it knows how to work with data. Type subset: scalar and compound.
Rust is **statically typed language**: it must know the types of all variables at compile time.
The compiler can  usually infer what type we want to use based on the value and how we use it.
In cases when many types are possible, such as when we converted a `String` to a numeric type using `parse` in
the "Comparing the Guess to the Secret number" section.

```
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don'tn add the `: u32` type annotation shown in the preceding code, Rust will display the following error
which means the compiler needs more information from us to know which type we want to use.

`type must be known at this point`
`help: consider givin 'guess' an explicit type`

## Scalar Types

A *scalar type* represents a single value. Rust has four primary scalar types: integers, floating-point, number,
booleans and characters. You may recognize these from other programing langauges.

### Integer Types

An integer is a number without a fractional component.

```
length      signed      unsigned
8-bit       i8          u8
16-bit      i16         u16
32-bit      i32         u32
64-bit      i64         u64
128-bit     i128        u128
arc         isize       usize
```

Each variant can be either signed or unsigned and has an explicit size.

Integer types default to `i32`.


### Integer Overflow
Let's say you have a variable of type `u8` that can hold values between 0 and 255. If you try to change the
variable to a value outside that range, such as 256, *integer overflow* will occur.
Rust includes checks for integer overflow that cause your program to panic at runtime if this behaviour occurs.
Rust uses the term *panicking* when a program exits with an error.

When you're compiling in release mode with the `--release` flag, r doest not include checks for integer
overflow that causes panics. Instead, if overflow occurs, Rust performs two's complement wrapping. In short
values greater than the maximum value the type can hold "wrap around" to the minimum of the values the type
can hold: In case of `u8`, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won't panic
but the variable will have a value that probably isn't what you were expecting it to have.

Relying on integer overflow's wrapping behaviour is considered an error.

To explicitly handle the possibility of overflow, you can use these families of methods provided by the
standard library for primitive numeric types:

-   Wrap in all modes with the `wrapping_*` methods, suchs as `wrapping_add`.
-   Return the `None` value if there is overflow with the `checked_*` methods.
-   Return the value and a boolean indicating whether there was overflow with the *overflowing_** methods.
-   Saturate at the value's minimum or maximum values with the `saturating_*` methods.

## Compound Types

Can group multiple values into one type. Rust has two primitive compound types: **tuples** and **arrays**.

### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
Touples have a fixed length: once declared, they cannot grow or shrink in size.

```
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single compound element.

We can use pattern matching to destructure a tuple value:

```
fn main() {
    let tup = (500, 6.3, 1);

    let (x, y , z) = tup;

    println!("The value of y is: {y}");
}
```

This program first creates a tuple element directly by using a period `.`

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred  = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

The tuple without any values has a special name, **unit**. This value and its corresponding type are both
written `()` and represents an empty value or an empty return type.

### The array type

Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to
ensure you always have a fixed number of elements.
A *vector* is allowed to grow or shrink in size.
*Arrays* are more useful when you know the number of elements will not need to change.

Array type using square brackets with the type of each element, a semicolon and then the number of elements.

You can also initialize an array to contain the same value for each element by specifying the initial value.
Followed by a semicolon, and then the length of the array in square brackets, as shown here.
```
fn main(){
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 2] = [2, 5];

    let c = [3;5]; // [3, 3, 3, 3, 3]


    let first = a[0];
    let second = a[1];
}

```

## Understanding Ownership

Ownership is r most unique feature and has deep implications for the rest of the
language. It enables r to make *memory safety* guarantees without needing a garbage
collector, so it's important to understand how ownership works.

*Ownership* is a set of rules that govern how a r manages memory. In r memory is
managed through a system of ownership with a set of rules that the compiler checks.
If any of the rules are violated, the program won't compile. None of the features of
ownership will slow down your program while it's running.

When you understand ownership, you'll have a solid foundation for understanding the
features that make r unique.

**The Stack and the Heap**

Many programming languages don't require you to think about the stack and the heap
very often. But in a systems programming language like r, whether a value is on the
stack or the heap affects how the language behaves and why you have to make certain
decisions.

Both the stack and the heap are parts of memory available to your code to use at
runtime, but they are structured in a different ways.

The **stack** stores values in the order it gets them and removes the values in the opposite
order. This is refered to as *last in, first out*. Adding data is called *pushing onto
the stack*, and removing data is called *popping off the stack*. All the data stored
on the stack must have a known, **fixed size**.
Data with unknown size at compile time or a size that might change must be stored on
the heap instead.


The **heap** is less organized: when you put data on the heap, you request a certain
amount of space. The memory allocator finds an empty spot in the heap that is big enough,
marks it as being use, and returns a *pointer*, which is the address of that location.

This process is called *allocating on the heap* == *allocating*.

Because the pointer to the heap is a known, fixed size, you can store the pointer on
the stack, but when you want the actual data, you must follow the pointer.


------------

Pushing to the stack is faster than allocating on the heap because the allocator never
has to search for a place to store new data. That alocation is always at the top of the
stack.

Allocating space on the heap requires more work because the allocator must first find a
big enough space to hold the data and the perform bookeeping to prepare for the next
allocation.

Accessing data in the heap is slower than accessing data on the stack because you have
to follow a pointer to get there. Also processors are faster if they jump around less
in memory.

A processor can do its job better if it works on data that's close to other data (stack)
rather than farther away (heap).

When your code calls a function, the values passed into the function (including pointers
to data on the heap) and the function's local variables get pushed onto the stack.
When the function is over, those values get popped off the stack.

# Ownership Rules

-   Each value in Rust has a **owner**.

-   There can only be one owner at a time.

-   When the owner goes out of scope, the value will be dropped.

**Variable Scope**.

A scope is the range within a program fro which an item is valid.

The types covered previously are of known size, can be stored on the stack and popped
off the stack when their scope is over, and can be quickly and trivially copied to make
a new, independent instance if another part of code needs to use the same value in a
different scope.

But we want to look at data that is stored on the heap and explore how r knows when to
clean up the data, and the `String` type is a great example.

Can create a `String` from a string literal using the `from` function

```
let s = String::from("hello");
```

The double colon `::` operator allows us to namespace this particular `from` function
under the `String` type rather than using some sort of name like `string_from`.

This kind of String can be mutated:

```
let mut s = String::from("hello");

s.push_str(", world");

print({s}); // 'hello, world'

```

## Memory and Allocation

In case of string literal, we know the contents at compile time, so the text is
hardcoded directly into the final executable. This is why strings literals are fast
and efficient. But these properties only come from the strings literal's immutability.
Unfortunately, we can't put a blob of memory into the binary for each piece of text
whose size is unknown at compile time and whose size might change while running the
program.

With the `String` type, in order to support a mutable, growable piece of text, we
need to allocate an amount of memory on the heap, unknown at compile time, to hold the
contents.

-   The memory must be requested from the memory allocator at runtime.
-   We need a way of returning this memory to the allocator when we're done with our `String`

This first part is done by us: when we call `String::from`, its implementation requests
the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector, the GC
keeps track of and cleans up memeory that isn't being used anymore, and we don't need
to think about it.

R takes a different path: the memory is automatically returned once the variable that
owns it goes out of scope.

```
{
    let s = String::from("hello");      // s is valid from this point forward


}                                       // this scope is now over
                                        // and s is no longer valid
```

The is a natural point at which we can return the memory our `String` needs to the
allocator: when `s` goes out of scope. When a variable goes out of scope, r calls
a special function for us: `drop`. It's where the author of `String` can put the
code to return the memory. R calls `drop` automatically at the closing curly bracket.

Situations when we want to have multiple variables use the data we've allocated on the
heap.

## Variables and Data Interacting with Move.

```
let x = 5;
let y = x;

// Integers are simple values with known, fixed size. And these two 5 values are pushed
// onto the STACK.
```

```
let s1 = String::from("hello");
let s2 = s1;

// String is made up of three parts:
// 1. A pointer to the memory that holds the contents of the string.
// 2. A lenght.
// 3. A capacity.

// This group of data is stored on the STACK.
// The content of the string is stored on the HEAP.
```

The length is how much memory, in bytes, the contents of the `string` are currently
using. The capacity is the total amount of memory, in bytes, that the `string` has
received from the allocator.
The difference between length and capacity matters, but not in this context, so for
now, it's fine to ignore the capacity.

When we assign `s1` to `s2`, the data is copied, meaning we copy the pointer, the lenght
and the capacity that are on the stack. We do not copy the data on the heap that the
pointer refers to.

```
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!");
```

R after the line `let s2 = s1;`, r considers `s1` as no longer valid. R doesn't need to
free anything when `s1` goes out of scope.

R prevents you from usign the invalidated reference.

If you've heard the terms *shallow* and *deep copy* while working with other languages,
the concept of copying the pointer, length, and capacity without copying the data
probably sounds like making a shallow copy. But because R also invalidates the first
variable, instead of being called a shallow copy, it's known as *move*. In this example
we would say that `s1` *was moved* into `s2`.

With only `s2` valid, when it goes out of scope it alone will free the memory, and we're
done.

In addition R will never automatically create *deep* copies of your data. Therefore any
*automatic* copying can be assumed to be inexpensive in terms of runtime performance.

## Variables and Data Interacting with Clone

If we do want to deeply copy the heap of the `String`, not just the stack data, we can
use a common method called `clone`.

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

This works just fine and explicitly produces the behaviour where the heap data *does*
*get copied*.

When you see a call to `clone`, you know that some arbitrary code is being executed
and that code may be expensive. It's a visual indicator that something different is
going on.

## Stack-Only Data: Copy

There's another wrinkle we haven't talked about yet. This code using integers - part of
which was shown in Listing.

```
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```

We don't have to call to `clone`, but `x` is still valid and wasn't moved into `y`.

The reason is that types such as integers that have a known size at compile time are
stored entirely on the stack, so copies of the actual values are quick to make.

That means there's no reason we would want to prevent `x` from being valid after we
create the variable `y`. In other words, there's no difference between deep and shallow
copying here, so calling `clone` wouldn't do anything different from the usual shallow
copying, and we can leave it out.

R has a special annotation called the `copy` trait that we can place on types that are
stored on the stacks, as integers are.

If a type implements the `copy` trait, variables that use it do not move, but rather are
trivially copied, making them still valid after assignment to another variable.

R won't let us annotate a type with `Copy` if the type, has implemented the `Drop` trait.
If the type needs something special happen when the value goes out of scope and we add
the `copy` annotation to that type, we'll get a compile-time error.


So, what types implements the `copy` trait? You can check the documentation for the given
type to be sure, but as a general ruel, any group of simple scalar values can implement
`Copy`, and nothing that requires allocation or is some form of resource can implement
`Copy`.

Here are some types that implement `Copy`:

-   All the integers types, such as `u32`.
-   The Boolean type, `bool`, with values `true` and `false`.
-   All the floating-point types, such as `f64`.
-   The character type `char`.
-   Tuples, if they only contain types that also implement `copy`. (`i32`, `i32`)
implements `Copy`, but (`i32`, `String`) does not.

## Ownership and Functions

The mechanics of passing a value to a function are similiar to those when assigning a
value to a variable. Passing a variable to a function will move or copy, just as
assignment does.

```
fn main(){
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String){
    println!("{some_string}");
}

fn makes_copy(some_integer: i32){
    println!("{some_integer}");
}
```

## Return Values and Scope

Returning values can also transfer ownership.

```
fn main() {
    let s1 = gives_ownership();

    let s2 = String::("hello");

    let s3 = takes_and_gives_back(s2);

}


fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

The ownership of a variable follows the same pattern every time: assigning a value to
another variable moves it. When a variable that includes data on the heap goes out of
scope, the value will be cleaned up by `drop` unless ownership of the data has been
moved to another variable.


While this works, taking ownership and then returning ownership with every function is
bit tedious. What if we want to let a function use a value but not take ownership?
It's quite annoying that anything we pass in also needs to be passed back if we want
to use it again, in addition to any data resulting from the body of the function that
we might want to return as well.

R does let us return multiple values using a tuple.

```
fn main(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.leng();  // len() returns the length of a String

    (s, length)
}

```

But this is too much ceremony and a lot of work for a concept that should be common.

Luckily for us, R has a feature for using a value without transferring ownership, called
**references**.

## References and Borrowing

https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

The issue with the tuple code is that we have to return the `String` to the calling function so we can still use the `String` after the call to `calculate_length`, because the `String` was moved into `calculate_length`.

Instead, we can provide a reference to the `String` value.

A reference is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable.

Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.


```
fn main(){
  let s1 = String::from ("hello");
  let len = calculate_length(&s1);

  println!("The length of {s1} is {len}.");
}

fn calculate_length(s: &String) -> usize{
  s.leng()
}
```

First, notice that all the tuple code in the variable declaration and the function return value is gone.

Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.

These ampersands `&` represents *reference*, and they allow you to refere to some value without taking ownership of it.

Let's take a closer look at the function call here:

```
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

The `&s1` syntax lets us create a reference that *refers* to the value of `s1`, but does not own it.

Because it does not own it, the value it points to will not be dropped when the reference stops beign used.

Likewise, the signature of the function uses `&` to indicate that the type of the parameter `s` is a reference.

```
// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
}
// Here s goes out of scope. But because it does not have ownership of what
// it referes to, it is not dropped.
```

When functions have references as parameters instead of the actual values, we won't need to return values in order to give back ownership, because we never had ownership.

Just as variables are immutable by default, so are references. We're not allowed to modify something we have reference to.

## Mutable Refernces

We can fix the code to allow us to modify a borrowed value with just a few small tweaks that use, instead a **mutable reference**.

```
fn main(){
    let mus s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First we change `s` to be `mut`. Then can we create a mutable reference with `&mut s` where we call the `change` function, and update the function signature to accept a mutable reference with `some_string: &mut String`.

Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to `s` will fail.

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

// Error: First mutabe borrow occurs here
//second mutable borrow occurs here
```

This error says that this code is invalid because we cannot borrow `s` as mutable more than once at a time.

The restriction preventing multiple references to the same data at the same time,  allows for mutation but in a very controlled fashion.

The benefit of having this restriction is that R can prevent data races at compile time.

A **data race** is similar to a race condition and happens when these three behaviours occur:
-   Two or more pointers access the same data at the same time.
-   At least one of the pointers is being used to write to the data.
-   There's no mechanism beign used to synchronize access to the data.

As always, we can curly brackets to create a new scope, allowing for multiple references, just not *simultaneous* ones:

```
let mut s = String::from("hello");

{
    let r1 = &mut s;
}   // r1 goes out of scope here, so we can make a new ref. with no problems

let r2 = &mut s;
```

We also cannot have mutable references while we have an immutable one to the same value.

```
let mut s = String::from("hello"); // mutable `s`

let r1 = &s;        // r1. (immutable) reference to value `s`
let r2 = &s;        // r2. (immutable) reference to value `s`
let r3 = &mut s;    // r3. Mutable reference to value `s`

println!("{}, {} and {}" r1, r2, r3); // Error: r3 immutable reference ...
```

Multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used.

```
// This code compiles because the las usuage of the immutable references,
// occurs before the Mutable Reference is introduced.

let mut s = String::from("hello");      // Mutable s
let r1 = &s;                            // Immutable ref to s
let r2 = &s;                            // Immutable ref to s
println!("{r1} and {r2}");              // Usage of ref r1 and r2
let r3 = &mut s;                        // Create Mutable ref to s
println!("{r3}");                       // Usage of ref 3
```

## Dangling Refernces

*Dangling Pointer*: a pointer that references a location in memory that may have been given to someone else.

In R the compiler guarantees that references will never be dangling references:
if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the refenrence to the data does.

```
// creating a dangling reference
fn main(){
    let reference_to_nothing = dangle();
}

fn dangle() -> &String{
    let s = String::from("hello");
    &s
}

// Error: this function's return type contains a borrowed value, for it to be
// borrowed from
```
Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. R won't let us do this.

**The Rules of References**
-   At any given time, you can have either one mutable reference or any number of immutable references.
-   References must always be valid.

-   At any given time, you can have either one mutable reference or any number of immutable references.
-   References must always be valid.
