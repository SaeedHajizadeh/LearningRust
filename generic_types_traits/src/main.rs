// Generics allow us to replace specific types with a placeholder that represents multiple types
// to remove code duplication

// Look at the code below
/*
fn main() {
    let nums = vec![34, 50, 25, 100, 65];

    let mut largest = &nums[0];

    for num in &nums {
        if num > largest {
            largest = num;
        }

    }

    println!("The largest element in nums is {largest}")
}
*/

// Suppose now we want to find the larges number in two different lists of numbers
/*
fn main() {
    let nums1 = vec![6 , 2 , 3 , 1];

    // -- look for the largest in nums1 using a for loop --

    let nums2 = vec![1 , 2 , 18 , 0];

    // -- look for the largest in nums2 using a for loop --

}
*/

// This is doable but long and error-prone and not scalable. We can zip/extract the for loop
// in a function called largest
/*
fn largest(vec: &Vec<i32>) -> &i32 {
    let mut largest = &vec[0];

    for num in vec {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn main() {
    let nums1 = vec![34, 50, 25, 100, 65];

    let result1 = largest(&nums1);
    println!("The largest number is {result1}");

    let nums2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result2 = largest(&nums2);
    println!("The largest number is {result2}");
}
*/

// Next, we’ll use these same steps with generics to reduce code duplication.
// Notice that we need to separate functions to find the largest element of a Vec<i32>
// and a Vec<f32>



// When defining a function that uses generics, we place the generics in the signature
// of the function where we would usually specify the data types of the parameters and
// return value.



// Let's write a code that finds the largest of a list of i32s and one that finds the 
// largest element of a list of chars
/*
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for ch in list {
        if ch > largest {
            largest = ch;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}
*/



// let’s eliminate the duplication by introducing a generic type parameter in a single function.

// To parameterize the types in a new single function, we need to name the type parameter, just
// as we do for the value parameters to a function. By convention, as in C++, type parameter 
// names are short one-letter capital letters

// When we use a parameter in the body of the function, we have to declare the parameter name
// in the signature so that the compiler knows what that name means. Similarly, when we use a
// type parameter name in a function signature, we have to declare the type parameter name
// before we use it.

// o define the generic largest function, we place type name declarations inside angle
// brackets, <>, between the name of the function and the parameter list
/*
fn largest<T> (list: &Vec<T>) -> &T {
    let mut largest: &T = &list[0];

    for num in list {
        if largest < num {
            largest = num;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
*/
// The code above does not get compiled as it has an error. The help text of the error
// mentions std::cmp::PartialOrd, which is a trait, and we’re going to talk about traits in
// the next section. For now, know that this error states that the body of largest won’t
// work for all possible types that T could be. Because we want to compare values of type T
// in the body, we can only use types whose values can be ordered. To enable comparisons,
// the standard library has the std::cmp::PartialOrd trait that you can implement on types.
// To fix the code, we can follow the help
// text’s suggestion and restrict the types valid for T to only those that implement
// PartialOrd. The listing will then compile, because the standard library implements
// PartialOrd on both i32 and char.


// ---------------------------------- In Struct Definitions ----------------------------------
// We can also define structs to use a generic type parameter in one or more fields using the
// <> syntax.

// definition of a struct with a type parameter
/*
struct Point<T> {
    x : T,
    y : T,
}

fn main() {
    let integer = Point { x: 1 , y : 3 };
    let float = Point { x: 1.0 , y: 3.0};
}
*/

// Note that because we’ve used only one generic type to define Point<T>, this definition
// says that the Point<T> struct is generic over some type T, and the fields x and y are
// both that same type, whatever that type may be.

/*

struct Point<T> {
    x : T,
    y : T,
}

fn main() {
    let will_not_work = Point { x : 1 , y : 3.0};
}

*/

// If you run this you will get
/*
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
*/



// As in C++, To define a Point struct where x and y are both generics but could have
// different types, we can use multiple generic type parameters.

/*

struct Point<T , U> {
    x : T ,
    y : U ,
}


fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

*/




// -------------------------------- An Enum Definition ------------------------------------
// We can define enums to hold generic data types in their variants
// Recall the Option<T> was an enum utilized by the standard library in Rust:
/*

enum Option<T> {
    Some: T,
    None,
}

*/
// This definition of enum now makes sense. As you can see, the Option<T> enum is generic
// over type T and has two variants: Some, which holds one value of type T, and a None variant
// that doesn’t hold any value. By using the Option<T> enum, we can express the abstract concept
// of an optional value, and because Option<T> is generic, we can use this abstraction no matter
// what the type of the optional value is.


// Enums can use multiple generic types
/*

enum Result<T , E> {
    Ok(T),
    Err(E),
}

*/

// The Result enum is generic over two types, T and E, and has two variants: Ok, which holds
// a value of type T, and Err, which holds a value of type E. This definition makes it convenient
// to use the Result enum anywhere we have an operation that might succeed (return a value of
// some type T) or fail (return an error of some type E).



// -------------------------------- In Method Definitions -----------------------------------
// We can implement methods on structs and enums (as we did in Chapter 5) and use generic
// types in their definitions too.

// Example: Define a point in R^2 using a struct and implement a function on that so it returns
// the mirror of that point w.r.t. the origin
/*


struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T>{
    fn mirror_origin(&self) -> Self {               // impl<T> Point<T> { ... }        // Self ≡ Point<T>
        Self { x : -self.x , y: -self.y  }
    }
}

fn main() {
    
}


*/
// This needs to be unbugged when we learn traits. Rust does not know if the type T supports
// negation.

// An easier example


/*

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn map_on_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.map_on_x());
}

*/



// Note that we have to declare T just after impl so that we can use T to specify that
// we’re implementing methods on the type Point<T>. By declaring T as a generic type
// after impl, Rust can identify that the type in the angle brackets in Point is a
// generic type rather than a concrete type. We could have chosen a different name
// for this generic parameter than the generic parameter declared in the struct
// definition, but using the same name is conventional.


// We can also specify constraints on generic types when defining methods on the type.
// We could, for example, implement methods only on Point<f32> instances rather than
// on Point<T> instances with any generic type
/*
struct Point<T>{
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main(){
    let p = Point { x : 1.1 , y : 1.23};
    println!("The distance of this point from origin is {:?}" , p.distance_from_origin())
    
}
*/




// This code means the type Point<f32> will have a distance_from_origin method; other
// instances of Point<T> where T is not of type f32 will not have this method defined.


// This is interesting. Generic type parameters in a struct definition aren’t always the
// same as those you use in that same struct’s method signatures. The code below
// uses the generic types X1 and Y1 for the Point struct and X2 and Y2 for the mixup
// method signature to make the example clearer. The method creates a new Point instance
// with the x value from the self Point (of type X1) and the y value from the passed-in
// Point (of type Y2).

/*
struct Point<X1 , Y1> {
    x : X1,
    y : Y1,
}

impl<X1 , Y1> Point<X1 , Y1> {
    fn mixup<X2 , Y2>(self , other: Point<X2 , Y2>) -> Point<X1 , Y2> {
        Point {                // we could also use turbofishing here Point::<X1 , Y2>
            x : self.x,
            y : other.y,
        }
    }
}




fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
*/



// -------------------------- Performance of Code Using Generics ----------------------------------
// Using generic types won’t make your program run any slower than it would with concrete types.

// Rust accomplishes this by performing monomorphization of the code using generics at
// compile time. Monomorphization is the process of turning generic code into specific
// code by filling in the concrete types that are used when compiled.

// The compiler looks at all the places where generic code is called and generates code for
// the concrete types the generic code is called with.


// let's look at this using Option<T>

/*
    let integer = Some(5);
    let float = Some(5.0);

*/


// When Rust compiles this code, it performs monomorphization. During that process, the
// compiler reads the values that have been used in Option<T> instances and identifies
// two kinds of Option<T>: One is i32 and the other is f64. As such, it expands the
// generic definition of Option<T> into two definitions specialized to i32 and f64,
// thereby replacing the generic definition with the specific ones.

// The monomorphized version of the code looks similar to the following (the compiler
// uses different names than what we’re using here for illustration):
/*
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
*/








// ---------------------- Defining Shared Behavior with Traits -----------------------------
// A trait defines the functionality a particular type has and can share with other types
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// --------------------------- Defining a Trait ------------------------------------------
// Definition: A trait is a named set of methods that a type can promise to implement. It
// defines behavior that types can share, without saying how each type does it.


// Example
/*
// 1. Define the trait: the "axioms"
pub trait Shape {
    fn area(&self) -> f64;  // required: every implementor must define it

    fn describe(&self) -> String {   // default: free unless overridden
        format!("A shape with area {:.2}", self.area())
    }
}



// Types
struct Circle { r:  f64 }
struct Rect { w: f64 , h: f64 }


// 3. Implement the trait for each type
impl Shape for Circle {
    // We are requried to implement area for circle
    fn area(&self) -> f64 {
        3.14159 * self.r * self.r
    }
}

impl Shape for Rect {
    fn area(&self) -> f64{
        self.w * self.h
    }

    fn describe(&self) -> String {
        format!("a {}x{} rectangle", self.w, self.h)  // overriden
    }
}


// 4. Generic function with a trait bound: ∀T ∈ {Shapes}
fn print_area<T: Shape>(s: &T) {
    println!("{}", s.describe());
}

fn main() {
    print_area(&Circle {r : 0.5});
    print_area(&Rect {w: 5.0 , h: 4.0});
}
*/

// Note the syntax: impl Shape for Circle means “Circle satisfies the Shape axioms.”
// In that block, Self ≡ Circle, same rule as before: Self is the type after for


/*
    A good analogy (not fully) to traits is abstract classes in Python
    from abc import ABC, abstractmethod

    class Shape(ABC):
        @abstractmethod
        def area(self) -> float: ...          # required, like a trait method with no body

        def describe(self) -> str:            # default, like a trait method with a body
            return f"a shape with area {self.area():.2f}"

    class Circle(Shape):
        def __init__(self, r): self.r = r
        def area(self): return 3.14159 * self.r ** 2
*/




// Let's have an example over two libraries. In one library, the "aggregator" library
// we define the structs and methods and in the other we use it. We need these pub keywords
// so that these traits and structs and their variants are available in other crates like
// main.rs


/*
Here’s the full project, with every file’s contents.

aggregator/                 ← package folder (cargo new aggregator)
├── Cargo.toml
└── src/
    ├── lib.rs              ← library crate "aggregator"
    └── main.rs             ← binary crate, uses the library
*/


/*
Cargo.toml

toml
[package]
name = "aggregator"      # ← this is where the name `aggregator` comes from
version = "0.1.0"
edition = "2024"

[dependencies]
*/





/*
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// src/main.rs
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
*/


// Then run cargo build and cargo run from inside aggregator/

// Other crates that depend on the aggregator crate can also bring the Summary trait
// into scope to implement Summary on their own types.

// Consider a new package next to aggregator with one main crate
/*
projects/
├── aggregator/             ← the package above
└── my_app/                 ← a different package
    ├── Cargo.toml          ← [dependencies] aggregator = { path = "../aggregator" }
    └── src/
        └── main.rs         ← use aggregator::Summary; impl Summary for MyOwnType { ... }

*/


// One restriction to note is that we can implement a trait on a type only if either
// the trait or the type, or both, are local to our crate. For example, we can
// implement standard library traits like Display on a custom type like SocialPost as
// part of our aggregator crate functionality because the type SocialPost is local to
// our aggregator crate. We can also implement Summary on Vec<T> in our aggregator
// crate because the trait Summary is local to our aggregator crate.


// But we can’t implement external traits on external types. For example, we can’t
// implement the Display trait on Vec<T> within our aggregator crate, because Display
// and Vec<T> are both defined in the standard library and aren’t local to our
// aggregator crate. This restriction is part of a property called coherence, and
// more specifically the orphan rule, so named because the parent type is not present.
// This rule ensures that other people’s code can’t break your code and vice versa.


// ------------------------ Using Default Implementations -------------------------------
// As mentioned in the first example, sometimes we create an implementation of a method
// inside a trait and if we need to override it for a particular type, we can choose to
// do so. We saw one example on the Circle and Shape example


// Example 2:
/*

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}


impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());


*/

// This code prints 1 new post: (Read more from @horse_ebooks...).


// -------------------------- Using Traits as Parameters ------------------------------
// We can explore how to use traits to define functions that accept many different types.

// We’ll use the Summary trait we implemented on the NewsArticle and SocialPost types
// in code below to define a notify function that calls the summarize method on its
// item parameter, which is of some type that implements the Summary trait. To do this,
// we use the *impl Trait* syntax, like this:

/*
pub fn notify(item: &impl Summary) {
    println!("Breaking new {}", item.summarize());
}
*/

// Instead of a concrete type for the item parameter, we specify the impl keyword and
// the trait name. This parameter accepts any type that implements the specified trait.
// In the body of notify, we can call any methods on item that come from the Summary
// trait, such as summarize. We can call notify and pass in any instance of NewsArticle
// or SocialPost. Code that calls the function with any other type, such as a String or
// an i32, won’t compile, because those types don’t implement Summary.


// -------------------------- Trait Bound Syntax --------------------------------------
// The impl Trait syntax works for straightforward cases but is actually syntax sugar
// for a longer form known as a trait bound; it looks like this:

/*
pub fn notify<T: Summary>(item: &T){
    println!("Breaking new {}", item.summarize());
}
*/

// This longer form is equivalent to the example in the previous section but is more
// verbose. We place trait bounds with the declaration of the generic type parameter
// after a colon and inside angle brackets.


// The impl Trait is a simpler syntax that is good for simple cases, but if, say, we
// have two parameters that implement Summary, our code using impl Trait looks like
/*
pub fn notify(item1: &impl Summary , item2: &impl Summary) {

}
*/
// Using Trait bound syntax, if the two items have the same type, we can implement the
// above as follows
/*
pub fn notify<T: Summary>(item1 : &T , item2: &T){}
*/



// ------------------- Multiple Trait Bounds with the + Syntax -----------------------

// We can also specify more than one trait bound. Say we wanted notify to use display
// formatting as well as summarize on item: We specify in the notify definition that
// item must implement both Display and Summary. We can do so using the + syntax:


// Using impl and Trait bounds, this + syntax is possible
/*
pub fn notify(item: &(impl Summary + Display)){}
*/
// Using Trait bound
/*
pub fn notify<T: Summary + Display>(item: &T){}
*/
// With the two trait bounds specified, the body of notify can call summarize and use
// {} to format item.



// ---------------------- Clearer Trait Bounds with where Clauses -----------------------------
// Using too many trait bounds has its downsides. Each generic has its own trait bounds,
// so functions with multiple generic type parameters can contain lots of trait bound
// information between the function’s name and its parameter list, making the function
// signature hard to read. For this reason, Rust has alternate syntax for specifying trait
// bounds inside a where clause after the function signature. So, instead of writing this:

/*
pub fn some_function<T: Display + Clone , U: Clone + Debug>(t: &T, u: &U) -> i32 {}
*/

// We can use a where clause as in

/*
fn some_function<T , U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
    {}
*/


// -------------------------- Returning Types That Implement Traits --------------------------
// We can also use the impl Trait syntax in the return position to return a value of some type
// that implements a trait
/*
fn returns_summarize() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
*/

// By using impl Summary for the return type, we specify that the returns_summarizable function
// returns some type that implements the Summary trait without naming the concrete type.
// In this case, returns_summarizable returns a SocialPost, but the code calling this function
// doesn’t need to know that.

// The ability to specify a return type only by the trait it implements is especially useful
// in the context of closures and iterators, which we cover in Chapter 13. Closures and
// iterators create types that only the compiler knows or types that are very long to
// specify. The impl Trait syntax lets you concisely specify that a function returns some
// type that implements the Iterator trait without needing to write out a very long type.


// ------------- Using Trait Bounds to Conditionally Implement Methods ---------------------
// By using a trait bound with an impl block that uses generic type parameters, we can
// implement methods conditionally for types that implement the specified traits. For
// example, the type Pair<T> in Listing 10-15 always implements the new function to return
// a new instance of Pair<T> (recall from the “Method Syntax” section of Chapter 5 that
// Self is a type alias for the type of the impl block, which in this case is Pair<T>).
// But in the next impl block, Pair<T> only implements the cmp_display method if its inner
// type T implements the PartialOrd trait that enables comparison and the Display trait
// that enables printing.

/*
use std::format::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T , y:T) -> Self {
        Self { x , y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
*/


// *************************** blanket implementation *************************************
// A blanket implementation in Rust is a way to implement a trait for all types that satisfy
// a specific condition, using generic parameters. Instead of writing the same trait
// implementation manually for every individual struct or enum, you write it once for any
// type matching your bounds.


// For example, the standard library implements the ToString trait on any type that implements
// the Display trait. The impl block in the standard library looks similar to this code:
/*
impl<T: Display> ToString for T{
    // ----- snip -----
}
*/



// ------------------------ Validating References with Lifetimes ------------------------------
// lifetimes ensure that references are valid as long as we need them to be.

// every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// Most of the time, lifetimes are implicit and inferred, just like most of the time, types are
// inferred. We are only required to annotate types when multiple types are possible. In a
// similar way, we must annotate lifetimes when the lifetimes of references could be related in
// a few different ways. Rust requires us to annotate the relationships using generic lifetime
// parameters to ensure that the actual references used at runtime will definitely be valid.

// This lifetime annootation is unique to Rust and might look unfamiliar to you.

// ------------------------- Dangling References ----------------------------------------------
// The main aim of lifetimes is to prevent dangling references, which, if they were allowed to
// exist, would cause a program to reference data other than the data it’s intended to reference
// Example:
/*
fn main() {
    let r;

    {
        let x: i32 = 5;
        r = &x;  // error: `x` does not live long enough
    }

    println!("r: {}", r); 
}
*/
// The reason is that x will be out of scope when the inner scope ends at the end of the block
// But r is still valid for the outer scope; because its scope is larger, we say that
// it “lives longer.” If Rust allowed this code to work, r would be referencing memory that was
// deallocated when x went out of scope, and anything we tried to do with r wouldn’t work
// correctly. So, how does Rust determine that this code is invalid? It uses a borrow checker.


// ------------------------------- The Borrow Checker ------------------------------------------
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows
// are valid. Let's look at the code above differently

/*
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+
*/

// Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can
// see, the inner 'b block is much smaller than the outer 'a lifetime block. At compile time,
// Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that
// it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter
// than 'a: The subject of the reference doesn’t live as long as the reference.


// One fix is to remove the inner scope so that x and r have the same lifetime


// ------------------------- Generic Lifetimes in Functions --------------------------------
// We’ll write a function that returns the longer of two string slices.
/*
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
*/


// Note that we want the function to take string slices, which are references, rather than
// strings, because we don’t want the longest function to take ownership of its parameters.

// If we were to write the function signature like below, it would not compile because Rust
// would not know how the lifetimes of the references relate to each other. The compiler
// would not be able to guarantee that the returned reference is valid for as long as the
// input references are valid.

/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
*/


// The help text reveals that the return type needs a generic lifetime parameter on it
// because Rust can’t tell whether the reference being returned refers to x or y.
// Actually, we don’t know either, because the if block in the body of this function
// returns a reference to x and the else block returns a reference to y!


// The borrow checker can’t determine this either, because it doesn’t know how the
// lifetimes of x and y relate to the lifetime of the return value. To fix this error,
// we’ll add generic lifetime parameters that define the relationship between the
// references so that the borrow checker can perform its analysis.


// -------------------------- Lifetime Annotation Syntax -------------------------------
// Lifetime annotations don’t change how long any of the references live. Rather, they
// describe the relationships of the lifetimes of multiple references to each other
// without affecting the lifetimes. Just as functions can accept any type when the
// signature specifies a generic type parameter, functions can accept references with
// any lifetime by specifying a generic lifetime parameter.


// Lifetime annotations have a slightly unusual syntax: The names of lifetime parameters
// must start with an apostrophe (') and are usually all lowercase and very short, like
// generic types. Most people use the name 'a for the first lifetime annotation. We
// place lifetime parameter annotations after the & of a reference, using a space to
// separate the annotation from the reference’s type.


/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

// One lifetime annotation by itself doesn’t have much meaning, because the annotations
// are meant to tell Rust how generic lifetime parameters of multiple references relate
// to each other.


// ------------------------- In Function Signatures --------------------------------
// To use lifetime annotations in function signatures, we need to declare the generic
// lifetime parameters inside angle brackets between the function name and the
// parameter list, just as we did with generic type parameters.

// Note that the question it tries to answer is whether the returned reference is valid
// for as long as the input references are valid.
/*

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

*/


// The function signature now tells Rust that for some lifetime 'a, the function takes
// two parameters, both of which are string slices that live at least as long as
// lifetime 'a. The function signature also tells Rust that the string slice returned
// from the function will live at least as long as lifetime 'a



// Remember, when we specify the lifetime parameters in this function signature, we’re
// not changing the lifetimes of any values passed in or returned. Rather, we’re
// specifying that the borrow checker should reject any values that don’t adhere to
// these constraints. Note that the longest function doesn’t need to know exactly how
// long x and y will live, only that some scope can be substituted for 'a that will
// satisfy this signature.


/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    
    println!("The longest string is {result}");
}
*/

// This is a compile time error. longest<'a> says the output lives as long as 'a,
// and both inputs must live at least 'a. So the compiler picks 'a as the overlap
// of the two input lifetimes, which means the shorter one.

// string1 lives until the end of main, but string2 dies at the inner }. So 'a
// ends at that inner }.

// The reference stored in result therefore has lifetime 'a and is only valid up to
// the inner }.

// println! uses it after that, which would be a use of a potentially dangling pointer.

// The compiler doesn’t look at runtime values. It doesn’t know that x (the longer
// string) would actually be returned. The signature says the result might point into
// string2, and that’s enough to reject the code.



// ------------------------------- Relationships ----------------------------------------
// Remember that lifetime annotations are used by the compiler to make sure that a reference
// is valid as long as it’s used. If the function output in the longest function above only
// depends on the first parameter, then only the first parameter needs to be valid for as long
// as the returned reference is valid. Hence, the compiler will only require annotation for the
// first parameter. 

/*
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
*/


// Writing y: &str gets a fresh anonymous lifetime from elision, so your signature is equivalent to:

/*
fn first_element<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }
*/

// Since 'b appears nowhere in the output, it’s unconstrained relative to the result, and
// there’s no reason to spell it out.


// You’re correct: result owns a heap buffer, it gets dropped when the function returns,
// and result.as_str() would point into freed memory. The compiler rejects it with:

/*
error[E0515]: cannot return value referencing local variable `result`
*/

// General rule: a function can’t return a reference to something it created. The returned
// reference has to borrow from an input or from 'static data. If you create the data
// inside the function, return the owned value and move it out:

/*
fn longest(_x: &str, _y: &str) -> String {
    String::from("really long string")
}
*/


// or 

/*
fn longest<'a>(_x: &str, _y: &str) -> &'a str {
    "really long string" // &'static str, valid for any 'a
}
*/


// ----------------------------- In Struct Definitions -----------------------------------
// So far, the structs we’ve defined all hold owned types. We can define structs to hold
// references, but in that case, we would need to add a lifetime annotation on every
// reference in the struct’s definition.

struct ImportantExcerpt<'a> {  // We declare the generic lifetime parameter 'a
                               // inside angle brackets after the name of the struct
                               // so that we can use the lifetime parameter in the body
                               // of the struct definition
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}



// This annotation means an instance of ImportantExcerpt can’t outlive the reference it
// holds in its part field. This is why this code compiles:
// The main function here creates an instance of the ImportantExcerpt struct that holds
// a reference to the first sentence of the String owned by the variable novel. The data
// in novel exists before the ImportantExcerpt instance is created. In addition, novel
// doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the
// reference in the ImportantExcerpt instance is valid.


// ------------------------- Lifetime Elision -------------------------------------------
// The compiler uses three rules to figure out the lifetimes of references when we don’t
// specify them. These rules are applied to the function’s signature, not its body. The
// first rule applies to input lifetimes, and the second and third rules apply to output
// lifetimes. The rules are as follows:


// Read about Elision rules here:
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision-rules

// ----------------- Generic Type Parameters, Trait Bounds, and Lifetimes -------------------
// Let's look at what we learned in one function

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,   // i.e. T implements the Display trait
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}


// This is the longest function from Listing 10-21 that returns the longer of two string
// slices. But now it has an extra parameter named ann of the generic type T, which can
// be filled in by any type that implements the Display trait as specified by the where
// clause. This extra parameter will be printed using {}, which is why the Display trait
// bound is necessary. Because lifetimes are a type of generic, the declarations of the
// lifetime parameter 'a and the generic type parameter T go in the same list inside the
// angle brackets after the function name.



