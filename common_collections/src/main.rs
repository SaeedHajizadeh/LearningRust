// ---------------------- Common Collections --------------------------------
// Rust’s standard library includes a number of very useful data structures called collections.
// Collections represent multiple values

// Unlike the built-in array and tuple types, the data that these collections point to is
// stored on the heap, which means the amount of data does not need to be known at compile
// time and can grow or shrink as the program runs. Each kind of collection has different
// capabilities and costs, and choosing an appropriate one for your current situation is a
// skill you’ll develop over time.

// We will consider
/*
    1. A vector allows you to store a variable number of values next to each other.
    2. A string is a collection of characters. We are talking about this in detail.
    3. A hash map allows you to associate a value with a specific key.
*/

// Remember the difference between array and tuple? Both have fixed size
// vectors however have floating sizes

// Vectors can only store values of the same type.

// Creating a new vector i.e. a type Vec<T>
/*
    let v: Vec<i32> = Vec::new();
*/

// Note that Vec<T> can hold any type which we'll talk about later
// In the code above, we did not populate the vector with any initial value so Rust requires
// us to pass in the intended type (i32 as in above)
// If we initialize the vector, Rust can infer the type

/*
    let v = vec![1 , 2 , 3];  // Rust infers the type of v is Vec<i32>
*/


// The rest is simple. Please refer to below

/*
// 1. Create
let v: Vec<i32> = Vec::new();   // empty → needs type annotation
let v = vec![1, 2, 3];          // macro, type inferred (i32)

// 2. Push (needs mut)
let mut v = Vec::new();
v.push(5);
v.push(6);                      // type inferred from pushes
v.pop();                        // returns Option<T>: Some(6)

// 3. Read: two ways
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];              // panics if out of bounds

// When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
let third: Option<&i32> = v.get(2);   // returns None if out of bounds

match v.get(100) {
    Some(x) => println!("{x}"),
    None => println!("no such element"),   // no crash
}
// &v[100] → panic

// 4. Borrow checker gotcha
let mut v = vec![1, 2, 3];
let first = &v[0];        // immutable borrow
v.push(4);                // ❌ mutable borrow while `first` is alive
println!("{first}");      // because push may reallocate, leaving `first` dangling

// 5. Iterate
for i in &v { println!("{i}"); }        // read-only
for i in &mut v { *i += 50; }           // modify (deref with *)
// can't push/remove inside these loops (same borrow rule)

// 6. Mixed types → wrap in an enum
enum Cell { Int(i32), Float(f64), Text(String) }
let row = vec![
    Cell::Int(3),
    Cell::Text(String::from("blue")),
    Cell::Float(10.12),
];
// unknown set of types at compile time → trait objects (Ch. 18)

// 7. Drop
{
    let v = vec![1, 2, 3];
} // v and all its elements are freed here
*/



// ------------------------- Storing UTF-8 Encoded Text with Strings ---------------------------
// What are strings?
// Rust has only one string type in the core language which is the string slice str that is
// usually seen in its borrowed form &str

// The String type, which is provided by Rust’s standard library rather than coded into the
// core language, is a growable, mutable, owned, UTF-8 encoded string type.

// String is actually implemented as a wrapper around a vector of bytes with some extra
// guarantees, restrictions, and capabilities. Hence it is very similar to Vec<T>
/*
let mut s = String::new();
*/
// This line creates a new, empty string called s, into which we can then load data.


// We can also use .to_string() method to create a String type
/*
let data = "Amber color";
let s = data.to_string();
*/
// This also works

/*
let s = "Amber color".to_string();
*/

// We can also do
/*
let s = String::from("Amber color");
*/


// Remember that strings are UTF-8 encoded so the following also work
/*
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
*/

// ------------------------- Updating a String --------------------------------

// Appending a string

/*
let mut s = String::from("hey there");
s.push_str("!!!!!"); 
*/


// push_str does not take ownership
/*
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

*/


// In the code above, we can print the value of s2 after using it
// The push method takes a single character as a parameter and adds it to the String.
/*
    let mut s = String::from("lo");
    s.push('l');
*/


// ------------------------------ Concatenating with + or format! ----------------------------
// The rest is summarized as below -> For more practical info, refer to rustlings exercises

/*
// 1. Concatenate with +
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;     // s1 is MOVED (gone), s2 still valid
// + is fn add(self, s: &str) -> String
//   self → takes ownership of s1, appends s2's bytes, returns it (no full copy)
//   &s2 is &String, auto-coerced to &str (deref coercion)
// String + String ❌, String + &str ✅

// 2. Many pieces: + gets ugly, use format!
let s = s1 + "-" + &s2 + "-" + &s3;     // works, unreadable
let s = format!("{s1}-{s2}-{s3}");      // clean, borrows everything, moves nothing

// 3. No indexing
let s = String::from("hi");
let h = s[0];          // ❌ compile error

// Why: String is a Vec<u8> of UTF-8 bytes
"Hola".len()           // 4 bytes
"Здравствуйте".len()   // 24 bytes, not 12 (2 bytes per char)
// s[0] would be byte 208, not 'З'. Also it couldn't be O(1) per char.

// 4. Three views of the same string
// "नमस्ते"
//   bytes:     [224, 164, 168, ...]           18 u8s
//   chars:     ['न','म','स','्','त','े']       6 Unicode scalar values
//   graphemes: ["न","म","स्","ते"]             4 "letters" (needs a crate)

// 5. Slicing by byte range: allowed but risky
let hello = "Здравствуйте";
let s = &hello[0..4];  // "Зд"  (4 bytes = 2 chars)
let s = &hello[0..1];  // 💥 runtime panic: not a char boundary

// 6. Iterate explicitly
for c in "Зд".chars() { println!("{c}"); }   // З, д
for b in "Зд".bytes() { println!("{b}"); }   // 208, 151, 208, 180

// 7. Useful methods
s.contains("ab");
s.replace("old", "new");
*/



// --------------------------------- Hash maps ---------------------------------------------


// ------------------------- Storing Keys with Associated Values in Hash Maps -----------------------
// Hash map is the Type HashMap<K , V> where the key has Type K and the value has Type V

// **************************************************************************************************
// Hash maps maps keys to values using a *hash function* which determines the places these keys and
// values are stored in the memory.
// **************************************************************************************************


// We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in
// the functions defined on HashMap<K, V> by the standard library. As always, check the standard
// library documentation for more information.


// --------------------------- Creating a new Hashmap -------------------------------------------
/*
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // empty hash map created

    scores.insert(String::from("blue") , 10);   // (key , value) pairs added to the map
    scores.insert("Yellos".to_string() , 11);  // (key , value) pairs added to the map

    
}
*/

// *****************************************************************************************************
// Just like vectors, hash maps store their data on the heap
// *****************************************************************************************************

// Like vectors, hash maps are homogeneous: 
// All of the keys must have the same type, and all of the values must have the same type.

// ------------------------- Accessing values in a hash map -------------------------------------
// In the code above let's say we want to access to team Blue's score
/*
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue") , 10);
    scores.insert(String::from("Yellow") , 11);

    let team = String::from("Blue");

    let score = scores.get(&team).copied().unwrap_or(0);
}
*/

// The .get() method returns an Option<&V>; Remember Option<&V> is an *enum* where it is
// epresenting an optional shared reference to a value of type V. It is a binary option meaning
// it is either Some(V) or None; if there’s no value for that key in the hash map, get will
// return None. This program handles the Option by calling copied to get an Option<i32> rather
// than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an entry
// for the key.

// Also note that we pass the team as a reference to the .get() method in order to preserve the team
// Remember that not passing a reference, the .get() emthod receives the owenership so the variable
// team is dropped after the completion of .get() method.
// Having a score following running .get() method without the team associated with that score is not
// particularly useful




// Iterate over key, value pairs

/*
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue") , 10);
    scores.insert(String::from("Yellow") , 11);

    let team = String::from("Blue");

    let score = scores.get(&team).copied().unwrap_or(0);

    for (key , value) in &scores {
        println!("{key} : {value}");
    }
}
*/


// --------------------------- Managing Ownership in Hash Maps -------------------------------
// Remember that we said passing a value into a function moves the ownership and the value is
// dropped after the function call. So if we add keys/values to a Hash and we want to KEEP them
// afterwards, what should we do? For types that implement the Copy trait, like i32, the values
// are copied into the hash map. For owned values like String, the values will be moved and the
// hash map will be the owner of those values

/*
use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

*/


// If we insert references to values into the hash map, the values won’t be moved into the hash map.


// ----------------------------- Updating a Hash Map -----------------------------------------
// You could replace the old value with the new value, completely disregarding the old value.
// You could keep the old value and ignore the new value, only adding the new value if the
// key doesn’t already have a value. Or you could combine the old value and the new value.
// Let’s look at how to do each of these!

// ----------------------------- Overwriting a Value --------------------------------------
// Simply insert the new value to the key 

/*

use std::collections::HashMap;

fn main(){

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
}

*/

// {"Blue" : 50}


// -------------------- Adding a Key and Value Only If a Key Isn’t Present -----------------------
// Sometimes we need to only add a value if the key does not exist and if the key exists, no action
// is taken. There is a special API for this called *entry* that takes the key you want to check as
// a parameter. 

/*

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Blue")) , 50))
    println!("{scores:?}");
}

*/
// The code above takes the parameter of the entry and seaches for that amongs the keys of the HashMap
// If the key does not exist, it associates the value in the or_insert(<value>) with that key.
// Otherwise, status quo is preserved. 
// The result is {"Yellow": 50, "Blue": 10}


// -------------------------- Updating a Value Based on the Old Value- ------------------------------
// Let's say we want to implement a word frequency and add the count each time we see a word (key) and
// if a word has not been seen we insert 0

use std::collections::HashMap;

fn main() {
    let text = "Learning Rust is really really really a fun process!";

    let mut frequency_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = frequency_map.entry(word).or_insert(0);
        *count += 1;
    }

    // for (key , value) in &frequency_map {
    //     println!("{key} : {value}");
    // }

    println!("{frequency_map:?}")
}


// Note: iterating over a hash map happens in an arbitrary order.
// The split_whitespace method returns an iterator over subslices, separated by whitespace,
// of the value in text.
// The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
// Here, we store that mutable reference in the count variable, so in order to assign to that value,
// we must first dereference count using the asterisk (*). The mutable reference goes out of scope at
// the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.


// By default, HashMap uses a hashing function called SipHash that can provide resistance to
// denial-of-service (DoS) attacks involving hash tables1. This is not the fastest hashing
// algorithm available, but the trade-off for better security that comes with the drop in
// performance is worth it. If you profile your code and find that the default hash function is
// too slow for your purposes, you can switch to another function by specifying a different
// hasher. A hasher is a type that implements the BuildHasher trait. We'll talk about traits
// later. 

// You don’t necessarily have to implement your own hasher from scratch; crates.io has
// libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.


