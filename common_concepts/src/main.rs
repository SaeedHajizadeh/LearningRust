// As we talked before, Rust variables are immutable by default
// When a variable is immutable, once a value is bound to a name, you can’t change that value.
/*
    fn main() {
        let x = 5;
        println!("The value of x {x}");

        x = 6;           // Compile error
        println!("The new value of x {x}");
    }
*/

// The following compiles successfully
/*
    fn main(){
        let mut x = 6;
        println!("The value of x: {x}");

        x = 11;
        println!("The value of x: {x}");
}
*/

// -------------------------------- Constants -------------------------------------
// Like immutable variables, constants are values that are bound to a name and are not allowed
// to change, but there are a few differences between constants and variables.

// 1. constants can NOT be made mutable. They are always immutable
// 2. The type of a constant MUST be annotated
// 3. constants can be declared in any scope, including global
// 4. The last difference is that constants may be set only to a constant expression,
//    not the result of a value that could only be computed at runtime.
// 5. They are declared by the keyword const

// Example
/*
    const DAYS_TO_WEDDING : u32 = (30 + 1) * 2
*/

/*
fn main(){
    const DAYS_TO_WEDDING : u32 = (30 + 1) * 2;
    println!("There are {DAYS_TO_WEDDING} days remining to our wedding.");
}
*/


// Constants are valid for the entire time a program runs, within the scope in which
// they were declared.

// -------------------------------- Shadowing ------------------------------------
// Rustaceans say that the first variable is shadowed by the second, which means that
// the second variable is what the compiler will see when you use the name of the variable.
// In effect, the second variable overshadows the first, taking any uses of the variable
// name to itself until either it itself is shadowed or the scope ends. We can shadow a
// variable by using the same variable’s name and repeating the use of the let keyword
// as follows:
/*
fn main(){
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is: {x}");
}
*/

// Shadowing is different from marking a variable as mut because we’ll get a compile-time
// error if we accidentally try to reassign to this variable without using the let keyword.
// By using let, we can perform a few transformations on a value but have the variable be
// immutable after those transformations have completed.

// The other difference between mut and shadowing is that because we’re effectively
// creating a new variable when we use the let keyword again, we can change the type of
// the value but reuse the same name. 
/*
fn  main(){
    let spaces = "     ";
    let spaces = spaces.len();
}
*/
// The first spaces variable is a string type, and the second spaces variable is a
// number type. Shadowing thus spares us from having to come up with different names,
// such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. 


// if we try to use mut for this, as shown here, we’ll get a compile-time error:
/*
    let mut spaces = "     ";
    let spaces = spaces.len();   // compile error
*/





// -------------------------------------------------------------------------------------
// -------------------------------- Data Types -----------------------------------------
// -------------------------------------------------------------------------------------

// Every value in Rust is of a certain data type

// There are two types of data type subsets: scalar and compound

// Keep in mind that Rust is a statically typed language, which means that it must know
// the types of all variables at compile time. The compiler can usually infer what type
// we want to use based on the value and how we use it. In cases when many types are
// possible, such as when we converted a String to a numeric type using parse in the
// “Comparing the Guess to the Secret Number”, we must add a type annotation, like this:
/*
    let guess: u32 = "42".parse().expect("Not a number!");
*/

// If we don’t add the : u32 type annotation shown in the preceding code, Rust will display
// an error. If you want to see the error, try running the following

/*
use std::io;
fn main(){
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    let guess = guess.trim().parse().expect("Please type a number!!!");

    println!("Value of guess {guess}")
}
*/

// ---------------------------------- Scalar Type ----------------------------------------
// A scalar type represents a single value. Rust has four primary scalar types:
// *********** integers, floating-point numbers, Booleans, and characters. **************

// Below is the table that 
/*

Length	                 Signed	    Unsigned
8-bit	                   i8	    u8
16-bit	                   i16	    u16
32-bit	                   i32	    u32
64-bit	                   i64	    u64
128-bit	                   i128	    u128
Architecture-dependent	   isize	usize
*/

// Signed numbers are stored using two’s complement representation.

// Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive,
// where n is the number of bits that variant uses. So, an i8 can store numbers
// from −(27) to 27 − 1, which equals −128 to 127.

// Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers
// from 0 to 28 − 1, which equals 0 to 255.

// Additionally, the isize and usize types depend on the architecture of the computer
// your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits
// if you’re on a 32-bit architecture.

// **************************** Integer overflow *************************************
// If you overflow your integer, when compiling a Rust code in debug mode, Rust includes
// checks for integer overflow that cause your program to panic at runtime if this
// behavior occurs--We'll learn more about panic later.

// When you’re compiling in release mode with the --release flag, Rust does not include
// checks for integer overflow that cause panics. Instead, if overflow occurs, Rust
// performs two’s complement wrapping. In short, values greater than the maximum value
// the type can hold “wrap around” to the minimum of the values the type can hold. In
// the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The
// program won’t panic, but the variable will have a value that probably isn’t what you
// were expecting it to have. Relying on integer overflow’s wrapping behavior is
// considered an error.

// To explicitly handle the possibility of overflow, you can use these families of
// methods provided by the standard library for primitive numeric types:

/*
    1. Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    2. Return the None value if there is overflow with the checked_* methods.
    3. Return the value and a Boolean indicating whether there was overflow with the
    overflowing_* methods.
    4. Saturate at the value’s minimum or maximum values with the saturating_* methods.
*/



// ---------------------------- Floating-Point Types ------------------------------
// Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size,
// respectively. The default type is f64 because on modern CPUs, it’s roughly the
// same speed as f32 but is capable of more precision. All floating-point types are
// signed. 

// Example of declaring floats
/*
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
*/


// ---------------------------- Numeric Operations ---------------------------------
// similar to other languages
/*
    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        // remainder
        let remainder = 43 % 5;
    }
*/

// ----------------------------- Boolean ------------------------------------------
/*
    fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
*/



// ---------------------------------- Character type -----------------------------
/*
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
*/

// Note that we specify char literals with single quotation marks, as opposed to string
// literals, which use double quotation marks. Rust’s char type is 4 bytes in size and
// represents a Unicode scalar value, which means it can represent a lot more than just
// ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and
// zero-width spaces are all valid char values in Rust. Unicode scalar values range
// from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character”
// isn’t really a concept in Unicode, so your human intuition for what a “character”
// is may not match up with what a char is in Rust.




// ------------------------- Compound Types ---------------------------------------------
// Compound types can group multiple values into one type. Rust has two primitive
// compound types: tuples and arrays.

// ------------------------- Tuples ---------------------
// A tuple is a general way of grouping together a number of values with a variety of
// types into one compound type. Tuples have a fixed length: Once declared, they cannot
// grow or shrink in size.

// We create a tuple by writing a comma-separated list of values inside parentheses.
// Each position in the tuple has a type, and the types of the different values in the
// tuple don’t have to be the same.
/*
fn main(){
    let tup: (i32 , f64 , u8) = (-350 , 22.35 , 1);  
}
*/
// In the code above, you'll get a warning for an unused variable in tup


// To get the individual values out of a tuple, we can use pattern matching to
// destructure a tuple value, like this:
/*
fn main(){
    let tup = (-350 , 22.35 , 1);

    let (x , y , z) = tup;    // pattern matching aka destructuring

    println!("The value of x , y , z are: {x} , {y} , {z}");
}
*/

// We can also access a tuple element directly by a 0-index place using the . operator
/*
fn main(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let first = tup.0;
    let second = tup.1;

    println!("The first and second elements are {first} and {second}");
}
*/


// The tuple without any values has a special name, unit. This value and its
// corresponding type are both written () and represent an empty value or an
// empty return type.

// ------------------------------- The Array Type -----------------------------------
// Another way to have a collection of multiple values is with an array. Unlike a
// tuple, every element of an array must have the same type. Unlike arrays in some
// other languages, arrays in Rust have a fixed length.
/*
fn main() {
    let a = [1, 2, 3, 4, 5];
}
*/


// Arrays are useful when you want your data allocated on the stack, the same as
// the other types we have seen so far, rather than the heap or when you want
// to ensure that you always have a fixed number of elements. An array isn’t as
// flexible as the vector type, though. A vector is a similar collection type provided
// by the standard library that is allowed to grow or shrink in size because its
// contents live on the heap. If you’re unsure whether to use an array or a vector,
// chances are you should use a vector.


// ***********************************************************************************
/*
    1. Arrays and tuples are both of a fixed length
    2. Elements of tuples need NOT have the same type
    3. Elements of the arrays MUST have the same type
    4. If you want a compound data type of variable length, use a vector from standard
       library
*/
// ***********************************************************************************

// Since arrays are of fixed length and fixed data type, you can determine both when you
// are declaring the variable.
/*
fn main(){
    let a:[i32 , 5] = [1 , 2 , 3 , 4 , 5];
}
*/
// The declaration of the array is as follows 
/*
    let <array_name>:[<array_type> , <length>] = [<initial_values>];    
*/


// You can also initialize an array to contain the same value for each element by
// specifying the initial value, followed by a semicolon, and then the length of
// the array in square brackets, as shown here:

/*
    let a = [3 ; 5];  // a = [3 , 3 , 3 , 3 , 3]
*/

// ------------------------------ Array Element Access ---------------------------------
// An array is a single chunk of memory of a known, fixed size that can be allocated on
// the stack. You can access elements of an array using indexing
/*
fn main(){
    let a = [1 , 2 , 3 , 4 , 5];

    let first = a[0];
    let second = a[1];

    println!("First and second elements are {first} and {second}");
}
*/




// Question: write a program that receives the index of an array from the user and
// prints the element in that position. You can define an array of size 10 initially
/*
use std::io;   // to receive input from the user

fn main(){
    let a = [1 , 2 , 3 , 4 , 5];

    println!("Please enter the index of the element you want:");

    // create a string to store the index
    let mut index = String::new();

    // Read the line from standard input
    io::stdin().read_line(&mut index).expect("Failed to read line!");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
*/

// This code compiles successfully. If you run this code using cargo run and enter
// 0, 1, 2, 3, or 4, the program will print out the corresponding value at that
// index in the array. If you instead enter a number past the end of the array,
// such as 10, you’ll see "index out of bounds" error


// The program resulted in a runtime error at the point of using an invalid value
// in the indexing operation. The program exited with an error message and didn’t
// execute the final println! statement. When you attempt to access an element using
// indexing, Rust will check that the index you’ve specified is less than the array
// length. If the index is greater than or equal to the length, Rust will panic.
// This check has to happen at runtime, especially in this case, because the compiler
// can’t possibly know what value a user will enter when they run the code later.

// This is an example of Rust’s memory safety principles in action. In many low-level
// languages, this kind of check is not done, and when you provide an incorrect index,
// invalid memory can be accessed. Rust protects you against this kind of error by
// immediately exiting instead of allowing the memory access and continuing.
// We will see more of Rust’s error handling and how you can write readable, safe code
// that neither panics nor allows invalid memory access.



// ----------------------------------------------------------------------------------
// ----------------------------------- Functions ------------------------------------
// ----------------------------------------------------------------------------------

// Functions in rust are declared by the fn keyword
// Rust code uses snake case as the conventional style for function and variable names,
// in which all letters are lowercase and underscores separate words. 
/*
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
*/


// Note that we defined another_function *after* the main function in the source code;
// we could have defined it before as well. Rust doesn’t care where you define your
// functions, only that they’re defined somewhere in a scope that can be seen by the caller.

// ------------------------------- Parameters ------------------------------------
// In function signatures, you must declare the type of each parameter. This is a
// deliberate decision in Rust’s design: Requiring type annotations in function
// definitions means the compiler almost never needs you to use them elsewhere in
// the code to figure out what type you mean. The compiler is also able to give
// more-helpful error messages if it knows what types the function expects.
/*

use std::io;
fn main(){
    let x = 5;
    let y = 3;

    add(x , y);

    println!("Please enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");

    println!("Please enter your age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line!");
    let age: i32 = age.trim().parse().expect("Age entered was not a number");
    
    show_name_age(name , age);
}

fn add(x: i32 , y: i32){
    let sum = x + y;
    println!("The sum of {x} and {y} is {sum}");
}

fn show_name_age(name: String , age: i32){
    println!("Your name is {name} and you are {age} years old!!!")
}

*/




// ------------------------- Statements and Expressions -----------------------------
/*
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value.
*/

// Function definitions are also statements; the entire preceding example is a
// statement in itself.

// Calling a function is not an statement, though.

// Statements do not return values. Therefore, you can’t assign a let statement to
// another variable, as the following code tries to do; you’ll get an error:
/*
    fn main() {
    let x = (let y = 6);
}
*/

/*
    The let y = 6 statement does not return a value, so there isn’t anything for x
    to bind to. This is different from what happens in other languages, such as
    C and Ruby, where the assignment returns the value of the assignment. In those
    languages, you can write x = y = 6 and have both x and y have the value 6;
    that is not the case in Rust.
*/

// Expressions evaluate to a value and make up most of the rest of the code that
// you’ll write in Rust. Consider a math operation, such as 5 + 6, which is an
// expression that evaluates to the value 11.

// Examples of expressions
// Calling a function is an expression.
// Calling a macro is an expression.
// A new scope block created with curly brackets is an expression, 

// Try to guess what the value of y will be in the following code
/*

fn main(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}

*/



/*
    {
    let x = 3;
    x + 1
}

    is a block that evaluates to 4
*/

/*
    Expressions do not include ending semicolons. If you add a semicolon to the end of
    an expression, you turn it into a statement, and it will then not return a value.
    Keep this in mind as you explore function return values and expressions next.
*/


// --------------------------- Functions with Return Values -----------------------------
/*
    1. we must declare their type after an arrow (->)
    2. In Rust, the return value of the function is synonymous with the value of the final
       expression in the block of the body of a function.
    3. You can return early from a function by using the return keyword and specifying a
       value, but most functions return the last expression implicitly.
*/

// Write a program that receives two numbers and multiples them

use std::io;   // we need stdin() from here

fn five() -> i32{
    5       // notice there is NO semicolon after 5 so the function implicitly returns 5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


fn multiply(x: f32 , y: f32) -> f32 {
    return x * y;
}


fn main(){
    println!("Please enter two numbers to multiply!");
    println!("Please enter the first number:");
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line!");
    let x: f32 = x.trim().parse().expect("Please type a number!");

    println!("Please enter the second number:");
    let mut y = String::new();

    io::stdin().read_line(&mut y).expect("Failed to read line!");
    let y: f32 = y.trim().parse().expect("Please enter a number!");

    let z = multiply(x , y);

    println!("The multiplication of {x} and {y} is {z}");

    let t = five();
    println!("The value the function five() returns is {t}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}


// Now let us add a semicolon at the end of x + 1 in the plus_one function above

/*
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;                returns an error
}
*/


// A note on the last function plus_one above: if we put a semicolon at the end after x,
// the program returns an error. The main error message, mismatched types, reveals the
// core issue with this code. The definition of the function plus_one says that it will
// return an i32, but statements don’t evaluate to a value, which is expressed by (),
// the unit type. Therefore, nothing is returned, which contradicts the function
// definition and results in an error.

