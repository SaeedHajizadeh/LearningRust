// ---------------------------- Enums and Pattern Matching ---------------------------------------

// -------------------------------- Defining an Enum ------------------------------------
// Where structs give you a way of grouping together related fields and data, like a Rectangle
// with its width and height, enums give you a way of saying a value is one of a possible set
// of values.

// Suppose we are working with IP addresses. Currently, there are two standards for used for
// IP addresses: version 4 and version 6. We will need to *enumerate* all possibilities (here
// only two) for IP addresses using enums.

// In other words, we need to define a new "type" for IP addresses consisting of V4 and V6

/*
enum IpAddKind {
    V4,
    V6,
}
*/

// In the code above, V4 and V6 are *enum variants* that represent IPv4 and IPv6

/*
fn main() {
// create instances of enums
let four = IpAddKind::V4;
let six = IpAddKind::V6;
}
*/


// We can define a function that receives IpAddKind as an input
/*
fn route (ip: IpAddKind) {}
*/

// and call this function on the variants
/*
route(IpAddKind::V4);
route(IpAddKind::V6);
*/


// Now in order to represent particular IP addresses, we need to identify the *kind* as well
// as the address of each individual IP themseleves so we might be tempted to use a combination
// of enum to take care of the kind and a struct to take care of the actual IP address like below
/*

#[derive(Debug)]
enum IpAddKind{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddKind,
    address: String,
}

impl IpAddress {
    fn new_ip_address(k : IpAddKind, s: String) -> Self {
        Self{
            kind: k,
            address: s,
        }
    }
}

fn main() {
    // IP address does not change in a connection
    let home = IpAddress::new_ip_address(IpAddKind::V4 , String::from("127.0.0.1"));

    println!("This IP Address is of kind {:?} and has an address of {:?}" 
    , home.kind , home.address);

    let loopback = IpAddress::new_ip_address(IpAddKind::V6 , String::from("::1"));
    println!("This IP Address is of kind {:?} and has an address of {:?}" 
    , loopback.kind , loopback.address)
}

*/


// However, you can take care of the kind and actual IP address (the String) using enum
// by itself
/*

#[derive(Debug)]
enum IpAddress {
    V4(String),
    V6(String),
}


fn main(){
    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    // println!("This IP Address is of kind {:?} and has an address of {:?}" 
    // , home.kind , home.address);

    println!("This IP Address is of kind {:?} and has an address of {:?}" 
    , loopback:: , loopback.address)

}

*/


// There’s another advantage to using an enum rather than a struct: Each variant can
// have different types and amounts of associated data. Version four IP addresses will
// always have four numeric components that will have values between 0 and 255. If we
// wanted to store V4 addresses as four u8 values but still express V6 addresses as
// one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:

/*
enum IpAddress {
    V4(u8 , u8 , u8 , u8),
    V6(String),
}

let home = IpAddress::V4(127,0,0,1);
let loopback = IpAddress::V6(String::from("::1"));

*/



// ------------------------------------ The Option Enum -------------------------------------------
// Option is a particular enum defined by the standard library.
// The Option enum encodes the possibility of a value to be something or nothing
// For example, if we want to check the first elemenet of a list, it could exist if the 
// list is non-null otherwise it does not

// Rust does not have a Null concept instead it offers an encapsulated null in an enum

// Rust does not have nulls, but it does have an enum that can encode the concept of a
// value being present or absent. This enum is Option<T>, and it is defined by the
// standard library as follows:
/*


enum Option<T> {
    None,
    Some(T),
}


*/

// The Option<T> enum is so useful that it’s even included in the prelude; you don’t
// need to bring it into scope explicitly. Its variants are also included in the
// prelude: You can use Some and None directly without the Option:: prefix.

// The Option<T> enum is still just a regular enum, and Some(T) and None are still
// variants of type Option<T>.

// T is a generic type parameter that we'll cover later
/*
let some_number = Some(5);
let some_char = Some('t');
let absent_number: Option<i32> = None;
*/

// The type of some_number is Option<i32>. The type of some_char is Option<char>,
// which is a different type. Rust can infer these types because we’ve specified
// a value inside the Some variant. For absent_number, Rust requires us to
// annotate the overall Option type: The compiler can’t infer the type that the
// corresponding Some variant will hold by looking only at a None value. Here,
// we tell Rust that we mean for absent_number to be of type Option<i32>.

// So we said that Rust does not have a null value and instead it has Option<T>::None 
// which we write as None, but whys is having Option<T> any better than null?

// In short, because Option<T> and T (where T can be any type) are different types,
// the compiler won’t let us use an Option<T> value as if it were definitely a valid
// value. For example, this code won’t compile, because it’s trying to add an i8 to
// an Option<i8>:

/*

    Running the following code r5esults in an error

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

*/

// The error message means that Rust doesn’t understand how to add an i8 and an Option<i8>




// --------------------- The match Control Flow Construct -------------------------------
// Rust has a powerful control flow construct called match that allows you to compare a
// value against a series of patterns and then execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards, and many other things;

// Let's look at an example. Consider an enum that has every coin and a function
// that maps the coins to their dollar values


/*
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,            // The first arm
        Coin::Nickel => 5,           // The second arm
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
*/


// Look at the match arms. An arm consists of two parts: a pattern and some code
// The first arm here has a pattern that is the value Coin::Penny and then the =>
// operator that separates the pattern and the code to run. The code in this case
// is just the value 1. Each arm is separated from the next with a comma.

// When the match expression executes, it compares the resultant value against the
// pattern of each arm, in order. If a pattern matches the value, the code associated
// with that pattern is executed. If that pattern doesn’t match the value, execution
// continues to the next arm


// f you want to run multiple lines of code in a match arm, you must use curly brackets,
// and the comma following the arm is then optional

// For example

/*

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky Penny!");
            1,
        }                              // The code prints "Lucky Penny" and returns 1
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

*/




// ------------------------------ Patterns That Bind to Values -------------------------------
// From 1999 through 2008, the United States minted quarters with different designs for
// each of the 50 states on one side. No other coins got state designs, so only quarters
// have this extra value. We can add this information to our enum by changing the Quarter
// variant to include a UsState value stored inside it

/*
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}


fn main() {
    let state = UsState::Alaska;

    let coin1 = Coin::Quarter(state);
    let coin2 = Coin::Quarter(UsState::Alabama);

    let value1 = value_in_cents(coin1);
    let value2 = value_in_cents(coin2);

    println!("Coin 1 has value: {:?}", value1);
    println!("Coin 2 has value: {:?}", value2);
}
*/



// ----------------------------- The Option<T> match Pattern -----------------------------
// We can also handle Option<T> using match, as we did with the Coin enum!

// Let’s say we want to write a function that takes an Option<i32> and, if there’s a
// value inside, adds 1 to that value. If there isn’t a value inside, the function
// should return the None value and not attempt to perform any operations.

/*
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


}
*/


// Let’s examine the first execution of plus_one in more detail. When we call
// plus_one(five), the variable x in the body of plus_one will have the value
// Some(5). We then compare that against each match arm. The Some(5) value doesn’t
// match the pattern None, so we continue to the next arm.
// Does Some(5) match Some(i)? It does! We have the same variant. The i binds to the
// value contained in Some, so i takes the value 5. The code in the match arm is then
// executed, so we add 1 to the value of i and create a new Some value with our total
// 6 inside.








// ---------------------------- Matches Are Exhaustive -------------------------------------
// The arms' pattern of our match must exhaust all possibilities. In the plus_one code above,
// if we remove the None pattern, we'll get an error (only since Rust knows this is not an 
// exhaustive pattern).

/*
    // Code below returns an error

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

*/



// ----------------- Catch-All Patterns and the _ Placeholder ---------------------
// Imagine we’re implementing a game where, if you roll a 3 on a dice roll, your
// player doesn’t move but instead gets a fancy new hat. If you roll a 7, your
// player loses a fancy hat. For all other values, your player moves that number
// of spaces on the game board. Here’s a match that implements that logic, with
// the result of the dice roll hardcoded rather than a random value, and all other
// logic represented by functions without bodies because actually implementing them
// is out of scope for this example:

/*

let dice_roll = 9;
match dice_roll {                      // patter => {%code for that pattern%}
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),       // other is the cathc-all pattern
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

*/


// Note that we have to put the catch-all arm last (Why?)

// Rust also has a pattern we can use when we want a catch-all but don’t want to use
// the value in the catch-all pattern: _ is a special pattern that matches any value
// and does not bind to that value. This tells Rust we aren’t going to use the value,
// so Rust won’t warn us about an unused variable.

// Let’s change the rules of the game: Now, if you roll anything other than a 3 or a 7,
// you must roll again. We no longer need to use the catch-all value, so we can change
// our code to use _ instead of the variable named other:

/*
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ =>  reroll(),               // You will not get a warning if you reroll do not use _ 
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
*/

// --------------------- Concise Control Flow with if let and let...else ------------------------
// The if let syntax lets you combine if and let into a less verbose way to handle values
// that match one pattern while ignoring the rest.


// *** In Rust, 3u8 is an unsigned 8-bit integer literal with a value of 3 ***
/*
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}
*/



// In the code snippet above, if the value is Some, we print out the value in the
// Some variant by binding the value to the variable max in the pattern. We don’t
// want to do anything with the None value. To satisfy the match expression, we have
// to add _ => () after processing just one variant, which is annoying boilerplate
// code to add.

// nstead, we could write this in a shorter way using if let.

/*
let config_max = Some(3u8)
if let Some(3u8) = config_max {
    println!("The maximum is configured to be {max}");
}
*/


// The syntax if let takes a pattern and an expression separated by an equal sign. It
// works the same way as a match, where the expression is given to the match and the
// pattern is its first arm. In this case, the pattern is Some(max), and the max binds
// to the value inside the Some. We can then use max in the body of the if let block
// in the same way we used max in the corresponding match arm. The code in the if let
// block only runs if the value matches the pattern.


// Using if let means less typing, less indentation, and less boilerplate code. However,
// you lose the exhaustive checking match enforces that ensures that you aren’t forgetting
// to handle any cases.


// We can include an else with an if let. The block of code that goes with the else is
// the same as the block of code that would go with the _ case in the match expression
// that is equivalent to the if let and else.


// The following two snippets are equivalent

/*
// 1. Using match
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}


// 2. Using if let and else
let mut count = 0;

if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
}
else {
    count += 1;
}
*/






















#[derive(Debug)]
enum ThermostatMode {
    Cooling,
    Heating,
    Eco,
    Off,
}

#[derive(Debug)]
enum SensorReading {
    Temperature(f64),
    Humidity(u8),
    BatteryLow,
    Error(String),
}

fn process_status(mode: &ThermostatMode, reading: SensorReading) -> String {
    // TODO: Write a match expression on `reading` (or a tuple `(mode, reading)`)
    // Requirements:
    // 1. If reading is BatteryLow, return "Warning: Battery is low!".
    // 2. If reading is Error(msg), return the error message formatted like "System Error: {msg}".
    // 3. If reading is Temperature(temp):
    //    - If mode is Heating and temp < 20.0, return "Heating active: boosting heat."
    //    - If mode is Cooling and temp > 25.0, return "Cooling active: lowering temp."
    //    - For any other temperature state, return "Temperature nominal: {temp}C".
    // 4. Handle all remaining cases (like Humidity) with a wildcard `_` returning "Reading processed: {:?}".

    match reading {
        // your code here
        SensorReading::BatteryLow => String::from("Warning: Battery is low!"),
        SensorReading::Error(msg) => {
            let res = String::from("System Error:");
            format!("{} {}" , res , msg)
        },
        SensorReading::Temperature(temp) => {
            match mode {
                ThermostatMode::Heating if temp < 20.0 => String::from("Heating active: boosting heat."),
                ThermostatMode::Cooling if temp > 25.0 => String::from("Cooling active: lowering temp."),
                _ => format!("Temperature nominal: {}C", temp),
            }
        },
        _ => format!("Reading processed: {:?}", reading),
    }
}

fn main() {
    let mode = ThermostatMode::Heating;
    let reading = SensorReading::Temperature(18.5);

    println!("{}", process_status(&mode, reading));
}