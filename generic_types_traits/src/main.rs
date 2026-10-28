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
