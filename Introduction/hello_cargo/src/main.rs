fn main(){
    println!("Hello, world!");
}

// main is the first function that runs in rust; just like any other programming language
// println! calls a Rust *macro*. If it had called a *function* instead, it would be
// entered as println (without the !). 

// For now, you just need to know that using a ! means that you’re calling a macro instead
// of a normal function and that macros don’t always follow the same rules as functions.


// You can create a new project in a given directory uising the following command
// from the terminal. First cd to the directory you want to create the project and then run

/*
    cargo new <project_name>
*/

// It creates a directory named <project_name> and, in it, you see a src folder along with
// a Cargo.toml which includes your dependencies. Hiddent in there, you also see a .gitignore
// which include the files not uploaded to the git, such as executables.
// Inside the src folder, you see main.rs. These are the default files created. 
// The Git repo file .gitignore will not be generate if you run a cargo new in a directory
// that already has a cargo file


// Cargo expects your source files to live inside the src directory. The top-level project
// directory is just for README files, license information, configuration files, and anything
// else not related to your code. Using Cargo helps you organize your projects

// One way to create a project when you have .rs files but not a project is to transfer the
// projects into a src folder and run cargo init to create a proper Cargo.toml file

// ----------------- Building and running a Cargo project ------------------------
// From your <projec_name> directory, build your project by entering the following command:
/*
$ cargo build
*/
// This command creates an executable file in target/debug/hello_cargo rather than in
// your current directory. Because the default build is a debug build, Cargo puts the
// binary in a directory named debug. You can run the executable with this command:
/*
$ ./target/debug/<project_name>
*/

// Notice that you are running the above command from terminal in the current directory
// of your project. Running cargo build creates the ./target/debug/<project_name> for you


// Instead of running cargo build to compile and then remember the directory of the exec file
// you can directly run "cargo run" on the current directory of the project (NOT the
// src directory) to compile AND run the executable.

/*
$ cargo check  
*/

// The command above checks your code to make sure it compiles but doesn’t produce an executable
// Why do we need that? We can run the cargo check much faster than cargo build as cargo check 
// because it skips the steps of producing an executable.


/*
----------------------------------------- Recap ----------------------------------------
1. We can create a project using cargo new.
2. We can build a project using cargo build.
3. We can build and run a project in one step using cargo run.
4. We can build a project without producing a binary to check for errors using cargo check.
5. Instead of saving the result of the build in the same directory as our code, Cargo stores
   it in the target/debug directory.
*/


// -------------------- building for release ---------------------------------------
// When your project is finally ready for release, you can use cargo build --release to
// compile it with optimizations. This command will create an executable in target/release
// instead of target/debug. The optimizations make your Rust code run faster, but turning
// them on lengthens the time it takes for your program to compile.




