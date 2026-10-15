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


// -------------------------- References and Borrowing -------------------------
// The issue with the tuple code in Listing 4-5 is that we have to return the String to
// the calling function so that we can still use the String after the call to
// calculate_length, because the String was moved into calculate_length. Instead, we can
// provide a reference to the String value. A reference is like a pointer in that it’s an
// address we can follow to access the data stored at that address; that data is owned by
// some other variable. Unlike a pointer, a reference is guaranteed to point to a valid
// value of a particular type for the life of that reference.

// Here is how to pass a reference to the calculate_length function rather than passing
// ownership of the String:
/*
fn main() {
    let s1 = String::from("Saeed");

    let len = calculate_length(s1); // &s1 is a reference to s1
    println!("The length of '{s1}' is {len}."); // This will print "The length of 'Saeed' is 5."
}

fn calculate_length(s: String) -> usize { // s is a reference to a String
    s.len() // len() returns the length of a String; this returns s's length
}  // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.
*/



// Notice that instead of using the tuple we are using the referencing operator & to pass
// a reference to the String value. Here, s is a reference to the owner of the String value s1

// Note: The opposite of referencing by using & is dereferencing, which is accomplished with
// the dereference operator, *.


// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own
// it. Because the reference does not own it, the value it points to will not be dropped
// when the reference stops being used.




// The scope of a referenced function parameter is the same as the scope of the function body.
// However, here since s does not own the value it refers to, the value will not be dropped
// when s goes out of scope. We also do not need to return the String value from the function,
// because we did not have the ownership of the String value in the first place. 
// We call the action of creating a reference borrowing.
// So what happens if we try to modify the value we are borrowing? 

/*
fn main() {
    let s = String::from("Saeed");

    change(s); // &s is a reference to s
}

fn change(some_string: &String) {
    some_string.push_str(" Hajizadeh"); // This will cause a compile-time error
}
*/


// ------------------ Mutable References -------------------------
// The code above will not compile because we are trying to modify a value through an
// immutable reference. By default, references are immutable, meaning that you cannot
// change the value they refer to. If we want to allow a reference to be mutable,
// we can use the mut keyword to declare a mutable reference:
/*

fn main() {
    let mut s = String::from("Saeed");

    change(&mut s); // &s is a reference to s
}

fn change(some_string: &mut String) {
    some_string.push_str(" Hajizadeh"); // This will cause a compile-time error
}

*/

// Here are the changes to the code above:
/*
    1. We chaneg s to be mutable by using the mut keyword.
    2. When we call the function change, we pass a mutable reference to s by using &mut s.
    3. We change the type of the reference in the function signature (definition) to be a
       mutable reference
*/

// This makes it very clear that change function will mutate the value it borrows.



// Mutable references have one big restriction: If you have a mutable reference to a
// value, you can have no other references to that value. This code that attempts to
// create two mutable references to s will fail. The following code will not compile:
/*
     let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
*/

// This error says that this code is invalid because we cannot borrow s as mutable more
// than once at a time. The first mutable borrow is in r1 and must last until it’s used
// in the println!, but between the creation of that mutable reference and its usage,
// we tried to create another mutable reference in r2 that borrows the same data as r1.

// The restriction preventing multiple mutable references to the same data at the same
// time allows for mutation but in a very controlled fashion. This prevents data races
// at compile time. A data race is similar to a race condition and happens in one of the
// following three situations:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.

// Data races cause undefined behavior and can be difficult to diagnose and fix when
// you’re trying to track them down at runtime; Rust prevents this problem by refusing
// to compile code with data races!

// As always, we can use curly brackets to create a new scope, allowing for multiple
// mutable references, just not simultaneous ones:

/*
      let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
*/

// Rust enforces a similar rule for combining mutable and immutable references. This
// code results in an error:

/*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
*/

 // Next example
/*
     let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
*/



// In languages with pointers, it’s easy to erroneously create a dangling pointer—a
// pointer that references a location in memory that may have been given to someone
// else—by freeing some memory while preserving a pointer to that memory. In Rust,
// by contrast, the compiler guarantees that references will never be dangling
// references: If you have a reference to some data, the compiler will ensure that
// the data will not go out of scope before the reference to the data does.


// The following code will give you a compile-time error

/*
    fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

*/


// The problem is that the String s is created inside the function dangle and will be
// dropped once the function ends. However, we are returning a reference to that String,
// which will be pointing to invalid/free memory. Rust prevents this situation from
// happening by refusing to compile the code.


// ---------------------------------------------------------------------------
// --------------------------- The Slice Type -------------------------
// ---------------------------------------------------------------------------
// Slices let you reference a contiguous sequence of elements in a collection. A slice
// is a kind of reference, so it does not have ownership.


// A programming challenge:
// Write a function that takes a string of words separated by spaces and returns the
// first word it finds in that string. If the function doesn’t find a space in the
// string, the whole string must be one word, so the entire string should be returned.

// Let's first try to write this function using the String type. We will call this
// function first_word. This will make it clear why we need to use slices instead of
// the String type.

// fn first_word(s: &String) -> ?

// A few comments already:
// 1. We do not need to take ownership of the String, so we use a reference 
// 2. We need to return a value that is part of the input String but we still do not know
//    what type to use for a "part" of a String. 
// 3. One way is to return the index of the end of the first word

/*

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

*/
// Because we need to go through the String element by element and check whether a value
// is a space, we’ll convert our String to an array of bytes using the as_bytes method.
/*
     let bytes = s.as_bytes();
*/

// Next, we create an iterator over the array of bytes using the iter method:
/*
    for (i, &item) in bytes.iter().enumerate() 
*/

// For now, know that iter is a method that returns each element in a collection and that
// enumerate wraps the result of iter and returns each element as part of a tuple instead.
// The first element of the tuple returned from enumerate is the index, and the second
// element is a reference to the element.


// Because the enumerate method returns a tuple, we can use patterns to destructure that
// tuple. We’ll be discussing patterns later.

// In the for loop, we specify a pattern that has i for the index in the tuple and &item
// for the single byte in the tuple. Because we get a reference to the element from
// .iter().enumerate(), we use & in the pattern. that is
/*
    for (i , &\item) in s.iter().enumerate() ...
*/

// Inside the for loop, we search for the byte that represents the space by using the byte
// literal syntax. If we find a space, we return the position. Otherwise, we return the
// length of the string by using s.len().

// There are two problems with this approach
// 1. We are returning a usize on its own, whereas it is only a meaningful value in the
//    context of the &String. If we set the string to "" after returning the usize value,
//    the usize is still active whereas the String is now a different value
// 2. If we are looking for the, say, second word in s, then the output must look like 
//    (usize, usize) making it more complex

// Rust has a solution to this problem

// ---------------------------------- String Slices ---------------------------------------
// A string slice is a reference to a contiguous sequence of the elements of a String
// A String slice looks like this
/*

    let s = String::from("Saeed Hajizadeh");

    let Saeed = &s[1..5];
    let Hajizadeh = &s[6..15];

*/
// We create slices using a range within square brackets by specifying
// [starting_index..ending_index], where starting_index is the first position in the slice
// and ending_index is one more than the last position in the slice. Internally, the slice
// data structure stores the starting position and the length of the slice, which corresponds
// to ending_index minus starting_index. So, in the case of let Hajizadeh = &s[6..15];,
// Hajizadeh would be a slice that contains a pointer to the byte at index 6 of s with a
// length value of 9.


// Similar to Python, With Rust’s .. range syntax, if you want to start at index 0, you can
// drop the value before the two periods. In other words, these are equal:

/*
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
*/

// By the same token, if your slice includes the last byte of the String, you can drop
// the trailing number. The following slices are equal

/*
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
*/


// With all this information in mind, let’s rewrite first_word to return a slice.
/*
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]           // If there is not a single space, return the entire String
}
*/


// Now the entire program in one place
/*

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn  main() {
    let mut s = String::from("Saeed Hajizadeh");

    let word = first_word(&s);

    s.clear();  // This is an error since there is a reference to s

    println!("The first word is {word}");
}

*/
// The reason for s.clear() to fail is that when clear needs to truncate the String, it
// needs to get a mutable reference. Moreover, word is a mutable reference to s (&s[0..i])
// so we have a mutable and an immutable reference to something at the same time and we saw
// in ownership that this is a compile-time error

/*
    A *binary* (often called an executable) is a file containing compiled machine code that
    a computer's CPU can execute directly. 
*/

// String literals are stored inside a compiled binary. We we define a string literal like below
/*
    let s = "Saeed Hajizadeh";
*/

// The type of the string literal s here is &str. It’s a slice pointing to that specific
// point of the binary. This is also why string literals are immutable; &str is an
// immutable reference.

// One more improvement to first_word signature:
// We can choose to use a &str type (slice type) as the parameter of the function in an 
// effort to make it more robust as it allows us to use the same function on both &String
// values and &str values.
/*
    fn first_word(s: &str) -> &str {
*/


// If we have a string slice, we can pass that directly. If we have a String, we can pass
// a slice of the String or a reference to the String.


// Defining a function to take a string slice instead of a reference to a String makes our
// API more general and useful without losing any functionality:


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn main() {
    let my_string = String::from("Saeed Hajizadeh");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    println!("word: {word}");
    let word = first_word(&my_string[..]);
    println!("word: {word}");
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);
    println!("word: {word}");

    let my_string_literal = "Saeed Hajizadeh";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("word: {word}");
    let word = first_word(&my_string_literal[..]);
    println!("word: {word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word: {word}");
}


// ------------------------------- Other slices ---------------------------------
// Array slices
/*
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice , &[2 , 3]);
*/


// This slice has the type &[i32]. It works the same way as string slices do, by storing
// a reference to the first element and a length.