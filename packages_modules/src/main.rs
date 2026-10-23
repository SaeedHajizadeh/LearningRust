// --------------------------- Packages, Crates, Modules ---------------------------------
// The programs we’ve written so far have been in one module in one file.

// As a project grows, you should organize code by splitting it into multiple modules and
// then multiple files.


//  package can contain multiple binary crates and optionally one library crate. As a
// package grows, you can extract parts into separate crates that become external dependencies.

// We’ll also discuss encapsulating implementation details, which lets you reuse code at
// a higher level: Once you’ve implemented an operation, other code can call your code
// via its public interface without having to know how the implementation works.

// The way you write code defines which parts are public for other code to use and which
// parts are private implementation details that you reserve the right to change. This is
// another way to limit the amount of detail you have to keep in your head.

// A related concept is scope: The nested context in which code is written has a set
// of names that are defined as “in scope.” When reading, writing, and compiling code,
// programmers and compilers need to know whether a particular name at a particular spot
// refers to a variable, function, struct, enum, module, constant, or other item and what
// that item means. You can create scopes and change which names are in or out of scope.
// You can’t have two items with the same name in the same scope; tools are available to
// resolve name conflicts.

/*
********************************************************************************************
    Rust has a number of features that allow you to manage your code’s organization,
    including which details are exposed, which details are private, and what names are
    in each scope in your programs. These features, sometimes collectively referred to
    as the *module system*
********************************************************************************************
*/

// Module System includes:
/*
    1. Packages: A Cargo feature that lets you build, test, and share crates
    2. Crates: A tree of modules that produces a library or executable
    3. Modules and use: Let you control the organization, scope, and privacy of paths
    4. Paths: A way of naming an item, such as a struct, function, or module
*/



// ------------------------------ Packages and Crates ----------------------------------
// A crate is the smallest amount of code that the Rust compiler considers at a time.
// Even if you run rustc rather than cargo and pass a single source code file the
// compiler considers that file to be a crate. Crates can contain modules

// A crate can come in one of two forms: a binary crate or a library crate.

/*
    1. Binary Crate:
        programs you can compile to an executable that you can run, such as a command
        line program or a server. Each must have a function called main that defines
        what happens when the executable runs. All the crates we’ve created so far
        have been binary crates.
    2. Library Crates
       programs that don’t have a main function, and they don’t compile to an executable.
       Instead, they define functionality intended to be shared with multiple projects.
       For example, the rand crate we used ealier provides functionality that generates
       random numbers. Most of the time when Rustaceans say “crate,” they mean library
       crate, and they use “crate” interchangeably with the general programming concept
       of a “library.”
*/

// A package is a bundle of one or more crates that provides a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates.
// Cargo is actually a package that contains the binary crate for the command line
// tool you’ve been using to build your code.

// A package can contain as many binary crates as you like, but at most only one
// library crate. A package must contain at least one crate, whether that’s a
// library or binary crate.

// You can create a package using cargo new <project_name>
// This command creates a directory named <project_name> and creates an src folder 
// inside it along with a Cargo.toml file for package dependencies

// Cargo follows a convention that src/main.rs is the crate root of a binary crate
// with the same name as the package.

// Here, we have a package that only contains src/main.rs, meaning it only contains a
// binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it
// has two crates: a binary and a library, both with the same name as the package. A
// package can have multiple binary crates by placing files in the src/bin directory:
// Each file will be a separate binary crate.

// Notice that the package can only have a single library crate

// -------------------- Control Scope and Privacy with Modules ----------------------------
// Read from Section 7.2 of Rust documentation in 
// https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

// In this section, we’ll talk about modules and other parts of the module system,
// namely paths, which allow you to name items; the use keyword that brings a path
// into scope; and the pub keyword to make items public.

// ------------------------------- Modules Cheat Sheet ------------------------------------
// how modules, paths, the use keyword, and the pub keyword work in the compiler?
// how most developers organize their code?

// We'll see examples later on, but let us start from
// *Start from the crate root*: 
//      When compiling a crate, the compiler first looks in the
//      crate root file (usually src/lib.rs for a library crate and src/main.rs for a binary
//      crate) for code to compile.

// *Declaring modules*:
//       In the crate root file, you can declare new modules; say you declare a “garden”
//       module with "mod garden;"

// The compiler will look for modules in these places:
/*
    1. Inline, within curly brackets that replace the semicolon following mod garden
    2. In the file src/garden.rs
    3. In the file src/garden/mod.rs
*/

// *Declaring submodules*:
// In any file other than the crate root, you can declare submodules. For example,
// you might declare mod vegetables; in src/garden.rs. The compiler will look for
// the submodule’s code within the directory named for the parent module in these places:

// The compiler will look for submodules in these places:
/*
    1. Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    2. In the file src/garden/vegetables.rs
    3. In the file src/garden/vegetables/mod.rs
*/


// *Paths to code in modules*:
//       Once a module is part of your crate, you can refer to code in that module from
//       anywhere else in that same crate, as long as the privacy rules allow, using the
//       path to the code. For example, an Asparagus type in the garden vegetables module
//       would be found at crate::garden::vegetables::Asparagus.

// *Private vs Public*:
//      Code within a module is private from its parent modules by default. To make a
//      module public, declare it with pub mod instead of mod. To make items within a
//      public module public as well, use pub before their declarations.


// *The use keyword*:
//      Within a scope, the use keyword creates shortcuts to items to reduce repetition
//      of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus,
//      you can create a shortcut with use crate::garden::vegetables::Asparagus;, and from
//      then on you only need to write Asparagus to make use of that type in the scope.

// Note
// a binary crate is a compilable target inside a project, not the project itself.



// Suppose we create a binary crate like this named backyard. Below is the directory
/*
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
*/


// The crate root file in this case is src/main.rs, and it contains:

/*
    use crate::garden::vegetables::Asparagus;

    pub mod garden;

    fn main() {
        let plant = Asparagus {};
        println!("I'm growing {plant:?}!");
}

*/

// The pub mod garden; line tells the compiler to include the code it finds in
// src/garden.rs, which is:

/*
    // Filename: src/garden.rs

    pub mod vegetables;
*/

// pub mod vegetables; means the code in src/garden/vegetables.rs is included too.
// vegatbles.rs is:

/*
    #[derive(Debug)]
    pub struct Asparagus {}
*/

// ----------------------------- Grouping Related Code in Modules ------------------------------
// Modules let us organize code within a crate for readability and easy reuse. Modules
// also allow us to control the privacy of items because code within a module is private
// by default. Private items are internal implementation details not available for outside
// use. We can choose to make modules and the items within them public, which exposes them
// to allow external code to use and depend on them.

// Note that modules in Rust are not like functions
// Key Differences between modules and functions
/*
        Functions (fn) contain executable statements and logic that perform a specific
        task when called.

        Modules (mod) act as a structural directory or folder. They hold a collection
        of other items, which can include functions, structs, enums, traits, constants,
        and even other child modules.
*/

// • private by default to that module unless you mark them with the pub keyword.
// • Modules themselves can also be marked pub to expose their contents to outer
//   modules or other crates.



// This is an example of parent and child modules

/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
*/

// we mentioned that src/main.rs and src/lib.rs are called crate roots.
// Here is the module tree for the code above
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// Modules hosting and serving are siblings

// ------------------ Paths for Referring to an Item in the Module Tree ------------------------
// To show Rust where to find an item in a module tree, we use a path in the same way we
// use a path when navigating a filesystem. To call a function, we need to know its path.

// A path can take two forms:

/*
    1. An absolute path is the full path starting from a crate root; for code from an external
        crate, the absolute path begins with the crate name, and for code from the current crate,
        it starts with the literal crate.
    2. A relative path starts from the current module and uses self, super, or an identifier
        in the current module.
*/

// Both absolute and relative paths are followed by one or more identifiers separated by
// double colons (::).

// Let's say in the code above we want to call the add_to_waitlist function from a new
// function, eat_at_restaurant, defined in the crate root.

/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
*/


// The eat_at_restaurant function is part of our library crate’s public API, so we
// mark it with the pub keyword.




/*
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {}
*/

// The code above runs into privacy error. The module hosting and function
// add_to_waitlist are private. Remember that the submodules are private to the
// parent module's siblings


// ********* Privacy Rules of Rust Modules ***********

// In Rust, all items (functions, methods, structs, enums, modules, and constants)
// are private to parent modules by default. If you want to make an item like a
// function or struct private, you put it in a module.

// Items in a parent module can’t use the private items inside child modules, but
// items in child modules can use the items in their ancestor modules. This is because
// child modules wrap and hide their implementation details, but the child modules
// can see the context in which they’re defined.


// Rust does give you the option to expose inner parts of child modules’ code to outer
// ancestor modules by using the pub keyword to make an item public.




/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {}
*/

// Notice the pub before hosting makes hosting public NOT its *contents*. This pub  makes
// hosting accesible to anything that has already access to front_of_house. To access 
// add_to_waitlist we need to precede its definition with another pub keyword




// ----------------- Best Practices for Packages with a Binary and a Library ----------------
// We mentioned that a package can contain both a src/main.rs binary crate root as well as
// a src/lib.rs library crate root, and both crates will have the package name by default.
// Typically, packages with this pattern of containing both a library and a binary crate
// will have just enough code in the binary crate to start an executable that calls code
// defined in the library crate. This lets other projects benefit from the most
// functionality that the package provides because the library crate’s code can be shared.

// The module tree should be defined in src/lib.rs. Then, any public items can be used in
// the binary crate by starting paths with the name of the package. The binary crate
// becomes a user of the library crate just like a completely external crate would use the
// library crate: It can only use the public API. This helps you design a good API; not
// only are you the author, but you’re also a client!



// --------------------- Starting Relative Paths with super ---------------------------------
// We can construct relative paths that begin in the parent module, rather than the current
// module or the crate root, by using super at the start of the path. This is like starting
// a filesystem path with the .. syntax that means to go to the parent directory. Using
// super allows us to reference an item that we know is in the parent module, which can
// make rearranging the module tree easier when the module is closely related to the parent
// but the parent might be moved elsewhere in the module tree someday.

// Example
/*
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
*/



/*
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    mod back_back {
        fn inside_inside() {
            super::fix_incorrect_order();
            super::super::back_of_house::fix_incorrect_order();
        }
    }
}

fn main() {

}
*/


// ----------------------- Making Structs and Enums Public -----------------------------
// We can also use pub to designate structs and enums as public, but there are a few extra
// details to the usage of pub with structs and enums. If we use pub before a struct
// definition, we make the struct public, but the struct’s fields will still be private.
// We can make each field public or not on a case-by-case basis.

// Important Example
// Below we’ve defined a public back_of_house::Breakfast struct with a public toast
// field but a private seasonal_fruit field. This models the case in a restaurant
// where the customer can pick the type of bread that comes with a meal, but the chef
// decides which fruit accompanies the meal based on what’s in season and in stock.
// The available fruit changes quickly, so customers can’t choose the fruit or even see
// which fruit they’ll get.



/*
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    meal.toast = "Wheat".to_string();   // or "Wheat".to_owned()
    println!("I'd like {} toast please!", meal.toast);

    
    // The next lines won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
    // println!("Please note the {} is the only option for fruit!" , meal.seasonal_fruit());
}


fn main() {

}
*/

// Note that because back_of_house::Breakfast has a private field, the struct needs to
// provide a public associated function that constructs an instance of Breakfast (we’ve
// named it summer here). If Breakfast didn’t have such a function, we couldn’t create
// an instance of Breakfast in eat_at_restaurant, because we couldn’t set the value of
// the private seasonal_fruit field in eat_at_restaurant.



// In contrast, if we make an enum public, all of its variants are then public.
/*

mod back_of_house {
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn main() {

}

*/



// ------------------- Bringing Paths into Scope with the use Keyword --------------------
// Best to explain by an example. Suppose you want to frequently use
// crate::front_of_house::hosting. You can use the use keyword and put this path into scope
// so you use a shorter version. 
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Following the line below, you can replace any crate::front_of_house::hosting with hosting
use crate::front_of_house::hosting;

pub fn eat_at_restaurant()  {
    hosting::add_to_waitlist();
}
*/



// Note that use only creates the shortcut for the particular scope in which the use occurs.

// The following produces an error

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Following the line below, you can replace any crate::front_of_house::hosting with hosting
use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant()  {
       hosting::add_to_waitlist();
    }
}

fn main {}
*/


// Solution: Use the use keyword in the scope in which it is going to be used

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant()  {
       hosting::add_to_waitlist();
    }
}

fn main() {}
*/



// An example of use with the standard library
/*
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
*/



// One thing to note that in the above we did not annouce to use the path all the way
// i.e. we did not say 
// use crate::front_of_house::hosting::add_to_waitlist;
// The reason for that is that it is better practice not to do that. Consider the
// following example

/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

// If instead, we specified use std::fmt::Result and use std::io::Result, we’d have
// two Result types in the same scope, and Rust wouldn’t know which one we meant when
// we used Result.


// ------------- Providing New Names with the as Keyword -------------------
// Another solution to the problem of bringing two types of the same name into the same
// scope with use

/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/









// -------------------------- Re-exporting Names with pub use ------------------------
// Suppose in the code we discussed before, we replace use with pub use

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}



// Before this change, external code would have to call the add_to_waitlist function
// by using the path restaurant::front_of_house::hosting::add_to_waitlist(), which also
// would have required the front_of_house module to be marked as pub. Now that this
// pub use has re-exported the hosting module from the root module, external code can
// use the path restaurant::hosting::add_to_waitlist() instead.


