// structs are a generalized version of tuples where each piece has its own name
// structs are defined using the *struct* keyword followed by the name of the struct
// then in the brackets we write the name and type of pieces that comprise the struct
// Each comprising ppiece is called a *field*. The following struct has 4 fields




// To use a struct after we’ve defined it, we create an instance of that struct by
// specifying concrete values for each of the fields. We create an instance as below



// To get a specific value from a struct, we use dot notation. 
/*

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count:  u64,   // notice the comma at the final row
}
fn main() {

    let user1 = User {
    email : String::from("Saeed@gmail.com"),
    username : String::from("Saeed_Hajizadeh"),
    active : true,
    sign_in_count : 1,
    };
    println!("The username of user1 is {}" , user1.username);
}

*/

// We can also make a struct mutable which allows us to modify the fields
/*
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count:  u64,   // notice the comma at the final row
}
fn main() {

    let mut user1 = User {
                        email : String::from("Saeed@gmail.com"),
                        username : String::from("Saeed_Hajizadeh"),
                        active : true,
                        sign_in_count : 1,
    };
    println!("The email of user1 is {}" , user1.email);

    user1.email = String::from("Saeed_Hajizadeh@gmail.com");
    println!("The new email of user1 is {}" , user1.email);
}
*/



// We can also return a User instance as the output of a function
/*

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count:  u64,   // notice the comma at the final row
}


fn build_user(username: String, email: String) -> User {
    User{
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("Saeed Hajizadeh") , String::from("Saeed@gmail.com"));
    println!("The email of user1 is {}" , user1.email);

    user1.email = String::from("Saeed_Hajizadeh@gmail.com");
    println!("The new email of user1 is {}" , user1.email);
}

*/

// -------------------------- Using the Field Init Shorthand ----------------------------
// Because the parameter names and the struct field names are exactly the same above,
// we can use the field init shorthand syntax to rewrite build_user

/*
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
*/

// ------------------------- Creating Instances with Struct Update Syntax ------------------------
// It’s often useful to create a new instance of a struct that includes most of the values
// from another instance of the same type, but changes some of them. You can do this using
// struct update syntax.

/*
    fn main() {
    // --snip--

        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
    }
*/

// The syntax .. specifies that the remaining fields not explicitly set should have the
// same value as the fields in the given instance.
/*
fn main() {
    // --snip--
    let user2 = User{
        email: String::from("sh123@gmail.com"),
        ..user1
    };
}
*/

// user2 above has a different email than user1 but in all other fields share the same
// value with user1

// In this example, we can no longer use user1 after creating user2 because the String in
// the username field of user1 was moved into user2. If we had given user2 new String values
// for both email and username, and thus only used the active and sign_in_count values from
// user1, then user1 would still be valid after creating user2. Both active and sign_in_count
// are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only
// Data: Copy” section would apply. We can also still use user1.email in this example, because
// its value was not moved out of user1.
// Remember that Strings are stored on a heap whereas integers and booleans are stored in a stack
// so copying them does not move them


// ------------------------ Creating Different Types with Tuple Structs --------------------------
// To define a tuple struct, start with the struct keyword and the struct name followed by the types
// in the tuple.
/*
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
*/

// You can destruct them above tuple structs or you can use a . find a particular index. The
// difference is, however, you use the type of the tuple struct to destructure
/*
    let Point(x , y , z) = black
*/

// -------------------------------- Defining Unit-Like Structs ---------------------------------
// You can also define structs that don’t have any fields! These are called unit-like structs
// because they behave similarly to (), the unit type that we mentioned
/*
    struct AlwaysEqual;
    fn main() {
        let instance_AE = AlwaysEqual;
    }
*/

// ----------------------- An Example Program Using Structs -------------------------------
// Let's write a program that computes the area of a rectangle
/*

struct Rectangle {
    height: u32,
    width: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.height * r.width
}

fn main() {
    let r = Rectangle {
        height : 6,
        width : 5,       
    };

    // We use references to avoid giving away the ownership from r to the function area
    println!("The area of the rectangle is {}" , area(&r)); 

    // Here, the rectangle r is still useable
}

*/




/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(" The width is {}" , rect1.width);
    rect1.width = 22;
    println!(" The width is {}" , rect1.width);
}
*/





// -------------------------- Funcationality with Structs --------------------------------
// It’d be useful to be able to print an instance of Rectangle while we’re debugging our program
// The println! macro can do many kinds of formatting, and by default, the curly brackets
// tell println! to use formatting known as Display

// With structs, the println! macro would not know how to print. Part or all of information, 
// should use comma or separate them by something else. It does not want to guess what we want
// so it raises an error
/*
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1}");
}
*/

// Let's try :? specifier. Putting the specifier :? inside the curly brackets tells println!
// we want to use an output format called Debug. The Debug trait enables us to print our
// struct in a way that is useful for developers --> This also gives an error

// Rust does include functionality to print out debugging information, but we have to
// explicitly opt in to make that functionality available for our struct. To do that,
// we add the outer attribute #[derive(Debug)] just before the struct definition
/*

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}

*/

// This runs very well
/*
$ cargo run
warning: fields `width` and `height` are never read
   --> src/main.rs:264:5
    |
263 | struct Rectangle {
    |        --------- fields in this struct
264 |     width: u32,
    |     ^^^^^
265 |     height: u32,
    |     ^^^^^^
    |
    = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally
    ignored during dead code analysis
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `structs` (bin "structs") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
*/


// Changing the specifier from :? to :#? makes the output look a little nicer that is good
// for structs with many fields
/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");
}
*/



// Another way to print out a value using the Debug format is to use the dbg! macro,
// which takes ownership of an expression (as opposed to println!, which takes a
// reference), prints the file and line number of where that dbg! macro call occurs
// in your code along with the resultant value of that expression, and returns ownership
// of the value.

/*

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

*/
// #[derive(Debug)] tells Rust to automatically implement the Debug trait for Rectangle.







// ------------------------------ Methods ---------------------------------------
// Methods are similar to functions: We declare them with the fn keyword and a name,
// they can have parameters and a return value, and they contain some code that’s run
// when the method is called from somewhere else.

// Methods' first parameter is always self

// Let’s change the area function that has a Rectangle instance as a parameter and
// instead make an area method defined on the Rectangle struct

// Let's change the area Rectangle area function and implement it as a method which
// means its parameter is an instance of Rectangle not the struct itself

/*


#[derive(Debug)] 
struct Rectangle {
    height: u8,
    width: u8,
}

impl Rectangle {
    fn area(&self) -> u8 {
        self.height * self.width
    }
}

fn main() {
    let r = Rectangle {
        height: 5,
        width: 6,
    };

    println!("Area of the rectangle is {:?}" , r.area())  // println is the last expression in 
    // the block so we do not need semicolons at the end
}


*/

// To define the function within the context of Rectangle, we start an impl (implementation)
// block for Rectangle. Everything within this impl block will be associated with the
// Rectangle type. Then, we move the area function within the impl curly brackets and
// change the first (and in this case, only) parameter to be self in the signature and
// everywhere within the body. In main, where we called the area function and passed
// rect1 as an argument, we can instead use method syntax to call the area method on
// our Rectangle instance. The method syntax goes after an instance: We add a dot followed
// by the method name, parentheses, and any arguments.

// In the signature for area, we use &self instead of rectangle: &Rectangle. The &self
// is actually short for self: &Self. Within an impl block, the type Self is an alias
// for the type that the impl block is for. Methods must have a parameter named self of
// type Self for their first parameter, so Rust lets you abbreviate this with only the
// name self in the first parameter spot. Note that we still need to use the & in front
// of the self shorthand to indicate that this method borrows the Self instance, just as
// we did in rectangle: &Rectangle. Methods can take ownership of self, borrow self
// immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

// Write a struct that defines an ellipse and a method stretched_area to compute its area when
// the larger radius is amplified by scale, which is a parameter of the method
/*


#[derive(Debug)]
struct Ellipse {
    r1: f32,
    r2: f32,
}

impl Ellipse {
    fn stretched_area(&self , scale: f32) -> f32 {
        let pi = std::f32::consts::PI;
        
        let mut r1 = if self.r1 > self.r2 { self.r1 } else { self.r2 };
        let r2 = if self.r1 > self.r2 { self.r2 } else { self.r1 };
        r1 *= scale;
        pi * r1 * r2
    }
}

fn main() {
    let e = Ellipse {
        r1: 0.6,
        r2: 3.0,
    };
//    let test = std::f32::consts::PI * 0.6 * 3.0;

    println!("The area of e is {:?} and if we stretch e, its area is {:?}" ,
                                e.stretched_area(1.0), e.stretched_area(2.0))
}


*/



// Exercise 2.0: 
// Implement a Rectangle data structure and its associated API to handle creation,
// measurement, and modification of geometric dimensions like we did
// 1. Data Structure Design : use struct
// 2. API implementation: (impl block) Design Constructor (new) , Area calculator (area), 
//    and the dimension scaler (scale)
// 3. Implement Client Verification in main


/*

#[derive(Debug)]
struct Rectangle {
    height : u32,
    width : u32,
}

impl Rectangle {
    // (i) Associated Function (Constructor)
    // Does not take 'self' as a parameter. Called using Rectangle::new(...)
    fn new(h: u32 , w: u32) -> Self {
        Self {
            height: h,
            width: w,
        }
    }
    // (ii) Immutable Method
    // Borrows 'self' read-only. Cannot modify the struct.
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 3. Mutable scale method
    // Borrow self mutably. Can modify fields
    fn scale(mut self , factor : u32) -> Self {
        self.height *= factor;
        self.width *= factor;
        self
    }
}

fn main() {
    let r = Rectangle::new(5 , 10);

    // Calling an immutable method
    println!("The area of the rectangle is {} square pixels." , r.area());

    // Calling a mutable method (requires the instance to be 'let mut')
    let r = r.scale(2);
    println!("New width: {}, New height: {}", r.width, r.height)

}

*/



// One thing to mention: below we let r be immutable and then change scale method to 
// take ownership mutably, modify the instance, and returns it as a Self type.
// We could alternatively define the scale method to only modify and has no return
// type and define r as mutable and input it as a mutable reference inside scale


/*

#[derive(Debug)]
struct Rectangle {
    height : u32,
    width : u32,
}

impl Rectangle {
    // (i) Associated Function (Constructor)
    // Does not take 'self' as a parameter. Called using Rectangle::new(...)
    fn new(h: u32 , w: u32) -> Self {
        Self {
            height: h,
            width: w,
        }
    }
    // (ii) Immutable Method
    // Borrows 'self' read-only. Cannot modify the struct.
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // 3. Mutable scale method
    // Borrow self mutably. Can modify fields
    fn scale(&mut self , factor : u32) {
        self.height *= factor;
        self.width *= factor;
    }
}

fn main() {
    let mut r = Rectangle::new(5 , 10);

    // Calling an immutable method
    println!("The area of the rectangle is {} square pixels." , r.area());

    // Calling a mutable method (requires the instance to be 'let mut')
    r.scale(2);
    println!("New width: {}, New height: {}", r.width, r.height)

}

*/



// ------------------------------ Methods with more parameters ------------------------------

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self , r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height 
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}