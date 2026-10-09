// Ownership is a set of rules that govern how a Rust program manages memory and all languages
// need to manage memory.
// ome languages have garbage collection that regularly looks for no-longer-used memory as the
// program runs; in other languages, the programmer must explicitly allocate and free the
// memory. Rust uses a third approach: 
// Memory is managed through a system of ownership with a set of rules that the compiler checks.

// ------------------------- The Stack and the Heap -------------------------
//  in a systems programming language like Rust, whether a value is on the stack or the
// heap affects how the language behaves and why you have to make certain decisions. 

// The stack stores values in the order it gets them and removes the values in the opposite
// order. This is referred to as last in, first out (LIFO). 

// Adding data is called pushing onto the stack, and removing data is called popping off
// the stack. All data stored on the stack must have a known, fixed size. Data with an
// unknown size at compile time or a size that might change must be stored on the heap instead.

// The heap is less organized: When you put data on the heap, you request a certain amount
// of space. The memory allocator finds an empty spot in the heap that is big enough,
// marks it as being in use, and returns a pointer, which is the address of that location.
// This process is called allocating on the heap and is sometimes abbreviated as just
// allocating (pushing values onto the stack is not considered allocating). Because the
// pointer to the heap is a known, fixed size, you can store the pointer on the stack, but
// when you want the actual data, you must follow the pointer. 

// Pushing to the stack is faster than allocating on the heap because the allocator never
// has to search for a place to store new data; that location is always at the top of
// the stack. Comparatively, allocating space on the heap requires more work because
// the allocator must first find a big enough space to hold the data and then perform
// bookkeeping to prepare for the next allocation.

// Accessing data in the heap is generally slower than accessing data on the stack because
// you have to follow a pointer to get there. Contemporary processors are faster if they
// jump around less in memory.

// When your code calls a function, the values passed into the function (including,
// potentially, pointers to data on the heap) and the function’s local variables get
// pushed onto the stack. When the function is over, those values get popped off the stack.

// Keeping track of what parts of code are using what data on the heap, minimizing
// the amount of duplicate data on the heap, and cleaning up unused data on the heap
// so that you don’t run out of space are all problems that ownership addresses.
// Once you understand ownership, you won’t need to think about the stack and the
// heap very often.

// ------------------------- Ownership Rules -------------------------
/*
    1. Each value in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
*/

// ------------------------- Variable Scope -------------------------
/*
    {                      // s is not valid here, since it's not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
*/


// ------------------------- The String Type -------------------------
// In order to understand ownership, you need a data type that is more complex than the scalar
// types that we have seen so far and are stored on the stack. The String type is one such
// complex data type, and it is stored on the heap.

// We have string literals, which are immutable fixed-length string somewhere in the
// binary of our program. We also have the String type, which is a growable, heap-allocated
// data structure that stores a string. The String type is useful when you need to modify the
// string or when you don’t know the content/size of the string at compile time.

// You can create a String from a string literal by calling the to_string method on the
// string literal, or by using the String::from function. Both of these will create a
// String that contains the text represented by the string literal.
/*

let s = "Saeed".to_string();
let s = String::from("Saeed");

// This kind of string can be mutated
let mut s = String::from("Saeed");
s.push_str(" Hajizadeh"); // push_str() appends a literal to a String
println!("{}", s); // This will print "Saeed Hajizadeh"

*/
// Why can Strings be mutated while string literals are not? The reason is how they deal with 
// memory.

// -------------------------- Memory and Allocation -------------------------
// In the case of a string literal, we know the contents at compile time, so the text is
// hardcoded directly into the final executable. This is why string literals are fast and
// efficient. But these properties only come from the string literal’s immutability.

// With the String type, in order to support a mutable, growable piece of text, we need
// to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

/*
    1. The memory must be requested from the memory allocator at runtime.
    2. We need a way of returning this memory to the allocator when we’re done with our String.
*/


// That first part is done by us: When we call String::from, its implementation requests the
// memory it needs. This is pretty much universal in programming languages.

// However, the second part is different. In languages with a garbage collector (GC), the
// GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need
// to think about it. In most languages without a GC, it’s our responsibility to identify
// when memory is no longer being used and to call code to explicitly free it, just as we
// did to request it. Doing this correctly has historically been a difficult programming
// problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid
// variable. If we do it twice, that’s a bug too. We need to pair exactly one *allocate* with
// exactly one *free*.

// Consider the following code
/*
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
*/

// There is a natural point at which we can return the memory our String needs to the
// allocator: when s goes out of scope. When a variable goes out of scope, Rust calls
// a special function for us. This function is called drop, and it’s where the author
// of String can put the code to return the memory. Rust calls drop automatically at
// the closing curly bracket.

// Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime
// is sometimes called Resource Acquisition Is Initialization (RAII)

// ------------------------- Ways Variables and Data Interact: Move -------------------------
// Let's consider the following snippet of code:

/*
    let x = 5;
    let y = x;
*/

// We can probably guess what this is doing: “Bind the value 5 to x; then, make a copy
// of the value in x and bind it to y.” We now have two variables, x and y, and both
// equal 5. This is indeed what is happening, because integers are simple values with
// a known, fixed size, and these two 5 values are pushed onto the stack.

// Now let's consider a similar snippet of code, but with a String instead of an integer:
/*
    let s1 = String::from("hello");
    let s2 = s1;
*/

// A String value contains a pointer (address) to its heap data, its length, and its
// capacity as a tuple. The heap actually stores the contents of the string, and the
// pointer, length, and capacity are stored on the stack. The pointer tells us where
// the contents are stored, the length tells us how much of that memory is currently
// being used, and the capacity tells us how much memory has been allocated for this
// string. When we assign s1 to s2, Rust copies the pointer, length, and capacity from 
// s1 to s2 creating a new stack frame but not the heap data itself. Assigning s1 to
// s2 copies these three fields, not the heap data itself; ownership of that data
// moves to s2, so s1 can no longer be used.


// Earlier, we said that when a variable goes out of scope, Rust automatically calls
// the drop function and cleans up the heap memory for that variable. But discussion
// above shows both data pointers pointing to the same location. This is a problem:
// When s2 and s1 go out of scope, they will both try to free the same memory. This
// is known as a double free error and is one of the memory safety bugs we mentioned
// previously. Freeing memory twice can lead to memory corruption, which can potentially
// lead to security vulnerabilities.

// To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer
// valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check
// out what happens when you try to use s1 after s2 is created; it won’t work:

/*
fn main() {
    let s1 = String::from("Saeed");
    let s2 = s1; // ownership of the heap data is moved to s2 and s1 is no longer valid

    println!("s2: {}", s2); // This will print "s2: Saeed"

    println!("s1: {}", s1); // This will cause a compile-time error because s1 is no longer valid

}
*/


// Normally this kind of copying of data is called a shallow copy, because only the stack
// data is copied, not the heap data. There is also a deep copy, which would copy both the
// stack data and the heap data. In Rust, however, this copy is called a move, instead of a
// shallow copy since s1 is no longer available after being copied. Hence, Rust calls this  
// a move and Rust won’t let us use the original variable after the move.
// Hence we say s1 is moved into s2. The ownership of the heap data is moved to s2, and s1
// is no longer valid. 

// With this in mind, whenever s1 is moved into s2, s1 is no longer valid, and 
// whenever s2 goes out of scope, Rust only calls drop on s2, which will free the heap memory.
// This ensures that the heap memory is freed *exactly once*, preventing double free errors
// and ensuring memory safety.

// THIS ALSO HAPPENS WITH DROP
// Consider the following code snippet:
/*
fn main() {
    let mut s = String::from("Saeed");

    s = String::from("Hajizadeh"); 
    println!("{}", s); // This will print "Hajizadeh"
}
*/
// In this case, the first String value that s which consisted of the stack data and the heap data
// (remember that the stack data is the pointer, length, and capacity of the heap data and the 
// heap data is the actual string data, in this case "Saeed")
// After the move, the initial heap that contained "Saeed" has nothing being pointed to it,
// and the stack data that s has points to the new heap data that contains "Hajizadeh".
// The old heap data that s was pointing to is now freed when s is assigned to point to
// the new heap data. In other words, Rust automatically calls drop on the first String value's
// heap data, freeing the heap memory before s is assigned to point to the new heap data ("Hajizadeh").



// -------------------------- Ways Variables and Data Interact: Clone -------------------------
// If we want to deeply copy the heap data as well, we can use the clone method

/*

fn main() {
    let s1 = String::from("Saeed");

    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2); // This will print "s1: Saeed, s2: Saeed"
}

*/
// In this case, both the stack data and the heap data are copied, so both s1 and s2 are
// valid and own their own heap data. When s1 and s2 go out of scope, Rust will call separate
// drop functions on both of them, freeing their respective heap data.

// Note: The clone method is often more expensive than a move, so you should only use it when
// you really need to deeply copy the heap data. In general, you should prefer moves over
// clones, and only clone when you need to have two independent copies of the data.

// -------------------------- Stack-Only Data: Copy -------------------------
// There’s another wrinkle we haven’t talked about yet. Consider the code below
/*
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
*/

// This code runs fine, which contradicts what we saw with the String type. The reason is
// that types like integers that have a fixed size known at compile time can be stored entirely
// on the stack, so assignments of these types don’t involve copying heap data. In other words,
// there is no difference between a deep copy and a shallow copy for these types.

// -------------------------- Ownership and Functions -------------------------
// The mechanics of passing a value to a function are similar to those when assigning a value
// to a variable. Passing a variable to a function will move or copy, just as assignment does. 
/*
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("s: {}", s); // This will cause a compile-time error because s is no longer valid

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

    println!("x: {}", x); // This will print "x: 5"
} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
*/





// -------------------------- Return Values and Scope -------------------------
// Returning values can also transfer ownership. Consider the following code:

/*
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
*/


// The ownership of a variable follows the same pattern every time: Assigning a value to
// another variable moves it. When a variable that includes data on the heap goes out of
// scope, the value will be cleaned up by drop unless ownership of the data has been moved
// to another variable.

// While this works, taking ownership and then returning ownership with every function is
// a bit tedious. What if we want to let a function use a value but not take ownership?
// It’s quite annoying that anything we pass in also needs to be passed back if we want
// to use it again, in addition to any data resulting from the body of the function that
// we might want to return as well.

// **************** Rust does let us return multiple values using a tuple *******************

/*
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
*/




// But this is too much ceremony and a lot of work for a concept that should be common.
// Luckily for us, Rust has a feature for using a value without transferring ownership:
// references.


