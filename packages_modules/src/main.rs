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

