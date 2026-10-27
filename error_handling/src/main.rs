// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// Recoverable errors like file not found -> we simply report it to the user
// Unrecoveratble errors: symptoms of bugs, accessing variables that are dropped, etc -> need
// to stop the program


// Instead of using exceptions, Rust uses two types to handle each kind of error
// Rust uses "Result<T , E>" Type for recoverable errors and "panic!" macro for
// unrecoverable errors. panic! macro stops the program when it encounters an unrecoverable
// error.









// ------------------------ Unrecoverable Errors with panic! --------------------------------
// There are two ways to cause a panic in practice: by taking an action that causes our code to
// panic (such as accessing an array past the end) or by explicitly calling the panic! macro.
// In both cases, we cause a panic in our program. By default, these panics will print a failure
// message, unwind, clean up the call stack, and quit. More precisely, he program starts unwinding,
// which means Rust walks back up the stack and cleans up the data from each function it encounters.


// Walking back and cleaning up is a lot of work. Rust therefore allows you to choose the
// alternative of immediately aborting, which ends the program without cleaning up.

// Memory that the program was using will then need to be cleaned up by the operating system.
// If in your project you need to make the resultant binary as small as possible, you can switch
// from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile]
// sections in your Cargo.toml file. For example, if you want to abort on panic in release mode,
// add this:

/*
// Add this in Cargo.toml
[profile.release]
panic = 'abort'
*/


// Let's try calling panic! macro in a program
/*
fn main() {
    panic!("Crash and burn!!!");
}
*/

/*
saeed@saeed-ROG-Strix-G18:~/Documents/LearningRust/RustProjects/error_handling$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/error_handling`

thread 'main' (9126) panicked at src/main.rs:47:5:
Crash and burn!!!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

// Notice that the second line of the error above shows where the panic occured. In real projects,
// these panicks could happen anywhere including a .rs file we did not write in a random line
// so this error structure helps a lot. 



// We can use the backtrace of the functions the panic! call came from to figure out the part
// of our code that is causing the problem. For example, in the code below, the panic does not
// directly come from calling panic! macro, but instead comes from a standard library bug!

/*
fn main() {
    let v = vec![1 , 2 , 4 , 3];

    v[99];
}
*/
// This in C would be undefined behavior. You might get whatever is at the location in memory that
// would correspond to that element in the data structure, even though the memory does not belong
// to that data structure. This is called *buffer overread* and can lead to security vulenrabilities
// if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be
// allowed to that is stored after the data structure.

// To protect your program from this sort of vulnerability, if you try to read an element at an
// index that doesn’t exist, Rust will stop execution and refuse to continue.

// To protect your program from this sort of vulnerability, if you try to read an element at
// an index that doesn’t exist, Rust will stop execution and refuse to continue.

// Let's try to get a backtrace by setting RUST_BACKTRACE *environment variable* to anything that is
// not 0.

/*
RUST_BACKTRACE=1 cargo run
*/

// ---------------------------- Recoverable Errors with Result ----------------------------------
// Recall that we introduced a particular enum called Result that had two variants: OK and Err
/*
enum Result<T , E> {
    OK(T),             // T and E are generic type parameters we'll discuss later
    Err(E),
}
*/
// T represents the type of the value that will be returned in a success case within the Ok
// variant, and E represents the type of the error that will be returned in a failure case
// within the Err variant.


// Let’s call a function that returns a Result value because the function could fail.
/*
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("Hey_there.txt");  // File::open() returns Result<T,E>
}
*/

// The generic parameter T has been filled in by the implementation of File::open with the type
// of the success value, std::fs::File, which is a file handle. The type of E used in the error
// value is std::io::Error. This return type means the call to File::open might succeed and
// return a file handle that we can read from or write to. The function call also might fail:
// For example, the file might not exist, or we might not have permission to access the file.
// The File::open function needs to have a way to tell us whether it succeeded or failed and at
// the same time give us either the file handle or error information. This information is
// exactly what the Result enum conveys.


// In the case where File::open succeeds, the value in the variable greeting_file_result will
// be an instance of Ok that contains a file handle. In the case where it fails, the value
// in greeting_file_result will be an instance of Err that contains more information about
// the kind of error that occurred.

// Let's see how to take action depending on the result type we get on the code above

/*
use std::fs::File;   // fs is file system standard library

fn main() {
    let greeting_file_result = File::open("Hey_there.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
*/

// Note that, like the Option enum, the Result enum and its variants have been brought into
// scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants
// in the match arms.

// When the result is Ok, this code will return the inner file value out of the Ok variant,
// and we then assign that file handle value to the variable greeting_file. After the match,
// we can use the file handle for reading or writing.

// ----------------------- Matching on Different Errors -----------------------------
// We may not need a panic! when the file does not exist. We want to take different actions
// for different failure reasons. If File::open failed because the file doesn’t exist, we
// want to create the file and return the handle to the new file. If File::open failed for
// any other reason—for example, because we didn’t have permission to open the file—we still
// want the code to panic! in the same way it did before. For this purpose, we use an inner 
// match expression. Can you see how to write it?

/*
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hey_there.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Error(error) => match error.kind() {
            ErrorKind::NotFound => File::create("hey_there.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}")
            }

        }
    }
}
*/



// The type of the value that File::open returns inside the Err variant is io::Error, which
// is a struct provided by the standard library. This struct has a method, kind, that we
// can call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the
// standard library and has variants representing the different kinds of errors that might
// result from an io operation. The variant we want to use is ErrorKind::NotFound, which
// indicates the file we’re trying to open doesn’t exist yet. So, we match on
// greeting_file_result, but we also have an inner match on error.kind().


// This is very helpful but also very complicated. We will learn about closures using which
// we can write this code more succinctly.

// ---------------------- Shortcuts for Panic on Error --------------------------------------
// Using match works well enough, but it can be a bit verbose and doesn’t always communicate
// intent well. The Result<T, E> type has many helper methods defined on it to do various,
// more specific tasks. The unwrap method is a shortcut method implemented just like the match
// expression we wrote in the snippet above. If the Result value is the Ok variant, unwrap will
// return the value inside the Ok. If the Result is the Err variant, unwrap will call the
// panic! macro for us.


/*
use std::fs::File;

fn main() {
    let greeting_file = File::open("hey_there.txt").unwrap();
}
*/


// Similarly, the expect method lets us also choose the panic! error message. Using expect
// instead of unwrap and providing good error messages can convey your intent and make
// tracking down the source of a panic easier.

/*
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
*/



/*
********************************* Best Practice **********************************

    In production-quality code, most Rustaceans choose expect rather than unwrap
    and give more context about why the operation is expected to always succeed.

**********************************************************************************
*/


// ---------------------------- Propagating Errors --------------------------------------
// When a function’s implementation calls something that might fail, instead of handling the
// error within the function itself, you can return the error to the calling code so that
// it can decide what to do. This is known as propagating the error and gives more control
// to the calling code, where there might be more information or logic that dictates how the
// error should be handled than what you have available in the context of your code.

// For example, code below shows a function that reads a username from a file. If the file
// doesn’t exist or can’t be read, this function will return those errors to the code that
// called the function.

/*
use std::fs::File;
use std::io::{self, Read};  // brings io and io::Read into scope altogether

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // if successful, username_file will contain the file handle; else an instance of io::Error
    let mut username_file = match username_file_result {
        Ok(file) => file, 
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    // file.read_to_string(&mut username) reads the contents of the file into username
    // It also returns Result since it might fail
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
*/


// This function can be written in a much shorter way, but we’re going to start by doing a
// lot of it manually in order to explore error handling; at the end, we’ll show the shorter
// way. Let’s look at the return type of the function first: Result<String, io::Error>.
// This means the function is returning a value of the type Result<T, E>, where the generic
// parameter T has been filled in with the concrete type String and the generic type E has
// been filled in with the concrete type io::Error


// If this function succeeds without any problems, the code that calls this function will
// receive an Ok value that holds a String—the username that this function read from the
// file. If this function encounters any problems, the calling code will receive an Err
// value that holds an instance of io::Error that contains more information about what the
// problems were. We chose io::Error as the return type of this function because that
// happens to be the type of the error value returned from both of the operations we’re
// calling in this function’s body that might fail: the File::open function and the
// read_to_string method.

// The code that calls this code will then handle getting either an Ok value that contains
// a username or an Err value that contains an io::Error. It’s up to the calling code to
// decide what to do with those values.

// This is called error propagation. We use ? operator to do it

// One thing

// --------------------------------- The ? Operator Shortcut ------------------------------
/*
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file_result = File::open("hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}
*/
// The ? placed after a Result value is defined to work in almost the same way as the
// match expressions that we defined to handle the Result


// We could further shorten this by chaining the methods calls immediately after the ? 
// operation
/*
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?; 
    Ok(username)
}
*/

// Note that read_to_string doesn’t hand you the text in the
// **File::open("hello.txt")?.read_to_string(&mut username)?** line. It writes the text
// into username and returns how many bytes it read, as Result<usize, io::Error>
// The ? unwraps that Result, leaving just the usize, e.g. 42. So if we return
// **File::open("hello.txt")?.read_to_string(&mut username)?** it'll return a usize but
// it should actually return a Result<String , io::Error> 


// Reading a file into a string is a fairly common operation, so the standard library
// provides the convenient fs::read_to_string function that opens the file, creates a
// new String, reads the contents of the file, puts the contents into that String,
// and returns it. Of course, using fs::read_to_string doesn’t give us the opportunity
// to explain all the error handling, so we did it the longer way first.

/*
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

*/


// ------------------------- Where to Use the ? Operator ---------------------------------
// The ? operator can only be used in functions whose return type is compatible with the
// value the ? is used on. This is because the ? operator is defined to perform an early
// return of a value out of the function, in the same manner as the match expression

// let’s look at the error we’ll get if we use the ? operator in a main function with a
// return type that is incompatible with the type of the value we use ? on.

/*
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
*/

// this code opens a file, which might fail. The ? operator follows the Result value
// returned by File::open, but this main function has the return type of (), not Result

// If you compile this code, you'll get: This error points out that we’re only allowed
// to use the ? operator in a function that returns Result, Option, or another type
// that implements FromResidual.

// To fix the error, you have two choices. One choice is to change the return type of
// your function to be compatible with the value you’re using the ? operator on as long
// as you have no restrictions preventing that. The other choice is to use a match or
// one of the Result<T, E> methods to handle the Result<T, E> in whatever way is appropriate.


// As with using ? on Result, you can only use ? on Option in a function that returns
// an Option. The behavior of the ? operator when called on an Option<T> is similar to
// its behavior when called on a Result<T, E>: If the value is None, the None will be
// returned early from the function at that point. If the value is Some, the value
// inside the Some is the resultant value of the expression, and the function continues.

// Example

/*
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

*/

// This function returns Option<char> because it’s possible that there is a character
// there, but it’s also possible that there isn’t. This code takes the text string
// slice argument and calls the lines method on it, which returns an iterator over the
// lines in the string. Because this function wants to examine the first line, it
// calls next on the iterator to get the first value from the iterator. If text is
// the empty string, this call to next will return None, in which case we use ? to
// stop and return None from last_char_of_first_line. If text is not the empty
// string, next will return a Some value containing a string slice of the first line in text.

// The ? extracts the string slice, and we can call chars on that string slice to get
// an iterator of its characters. We’re interested in the last character in this first line,
// so we call last to return the last item in the iterator. This is an Option because it’s
// possible that the first line is the empty string; for example, if text starts with a
// blank line but has characters on other lines, as in "\nhi". However, if there is a
// last character on the first line, it will be returned in the Some variant. The
// ? operator in the middle gives us a concise way to express this logic, allowing us to
// implement the function in one line. If we couldn’t use the ? operator on Option, we’d
// have to implement this logic using more method calls or a match expression.




// So far, all the main functions we’ve used return (). The main function is special
// because it’s the entry point and exit point of an executable program, and there are
// restrictions on what its return type can be for the program to behave as expected.

// Luckily, main can also return a Result<(), E>. Example

/*
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/


// The Box<dyn Error> type is a trait object which we will cover later



// --------------------------- To panic! or Not to panic! -----------------------------
// TBD

