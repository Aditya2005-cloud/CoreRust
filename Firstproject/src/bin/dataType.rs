// ============================================================
//                  RUST DATA TYPES
// ============================================================
//
// Rust is a statically typed language.
// Every value in Rust has a type.
//
// Rust data types can be broadly understood as:
//
// 1. Primitive / basic types
// 2. Compound types
// 3. String and slice types
// 4. Standard-library types
// 5. User-defined types
//
// ============================================================


// ============================================================
// 1. PRIMITIVE DATA TYPES
// ============================================================
//
// The main primitive types are:
//
// Integer types
// Floating-point types
// Boolean type
// Character type
//
// ============================================================


// ============================================================
// 1.1 INTEGER TYPES
// ============================================================
//
// Rust has two kinds of integer types:
//
// SIGNED INTEGER
// -------------------------
// i8
// i16
// i32
// i64
// i128
// isize
//
// Signed integers can store negative and positive values.
//
// The `i` means "signed integer".
//
//
// UNSIGNED INTEGER
// -------------------------
// u8
// u16
// u32
// u64
// u128
// usize
//
// Unsigned integers can store 0 and positive values.
// They cannot store negative values.
//
// The `u` means "unsigned integer".
//


// ============================================================
// SIGNED INTEGER RANGES
// ============================================================
//
// Type     Size       Minimum                                      Maximum
//
// i8       8-bit      -128                                         127
//
// i16      16-bit     -32,768                                      32,767
//
// i32      32-bit     -2,147,483,648                              2,147,483,647
//
// i64      64-bit     -9,223,372,036,854,775,808                  9,223,372,036,854,775,807
//
// i128     128-bit    -170141183460469231731687303715884105728    170141183460469231731687303715884105727
//
// isize    32/64-bit  Architecture dependent                       Architecture dependent
//
//
// `isize` depends on the CPU architecture.
//
// On a 64-bit system:
//
// -9,223,372,036,854,775,808
// to
//  9,223,372,036,854,775,807
//
// On a 32-bit system:
//
// -2,147,483,648
// to
//  2,147,483,647
//
//


// ============================================================
// UNSIGNED INTEGER RANGES
// ============================================================
//
// Type     Size       Minimum        Maximum
//
// u8       8-bit      0              255
//
// u16      16-bit     0              65,535
//
// u32      32-bit     0              4,294,967,295
//
// u64      64-bit     0              18,446,744,073,709,551,615
//
// u128     128-bit    0              340282366920938463463374607431768211455
//
// usize    32/64-bit  0              Architecture dependent
//
//
// `usize` also depends on the CPU architecture.
//
// On a 64-bit system:
//
// 0
// to
// 18,446,744,073,709,551,615
//
// On a 32-bit system:
//
// 0
// to
// 4,294,967,295
//
//


// ============================================================
// INTEGER EXAMPLE
// ============================================================

fn numbers_i() {

    // Signed integer
    let a: i32 = 45;

    // Signed integer can contain negative values
    let b: i32 = -45;

    // Unsigned integer
    let c: u32 = 45;

    // isize depends on the CPU architecture
    let d: isize = 45;

    // Rust normally infers this as i32
    // when there is no other type information.
    let age = 21;

    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");
    println!("d = {d}");
    println!("age = {age}");
}


// ============================================================
// 2. FLOATING-POINT TYPES
// ============================================================
//
// Rust has two floating-point types:
//
// f32 -> 32-bit floating point
// f64 -> 64-bit floating point
//
// f64 is Rust's default floating-point type.
//
// ============================================================

fn numbers_f() {

    let a: f32 = 45.0;

    let b: f64 = 45.0;

    // Rust infers this as f64
    let temperature = 98.6;

    println!("a = {a}");
    println!("b = {b}");
    println!("temperature = {temperature}");
}


// ============================================================
// 3. BOOLEAN TYPE
// ============================================================
//
// Rust has one Boolean type:
//
// bool
//
// Boolean has only two possible values:
//
// true
// false
//
// ============================================================

fn number_b() {

    let raining: bool = true;

    let sunny: bool = false;

    println!("Is it raining? {raining}");
    println!("Is it sunny? {sunny}");
}


// ============================================================
// 4. CHARACTER TYPE
// ============================================================
//
// Rust has the `char` type.
//
// A Rust char represents a Unicode scalar value,
// not just an ASCII character.
//
// A char is 4 bytes (32 bits) in Rust.
//
// IMPORTANT:
//
// char uses SINGLE quotes:
//
// 'a'
//
// Strings use DOUBLE quotes:
//
// "hello"
//
// ============================================================

fn number_c() {

    let a: char = 'a';

    let emoji: char = '😀';

    let hindi: char = 'न';

    println!("a = {a}");
    println!("emoji = {emoji}");
    println!("Hindi character = {hindi}");
}


// ============================================================
// 5. COMPOUND TYPES
// ============================================================
//
// Compound types group multiple values together.
//
// Rust has two primitive compound types:
//
// 1. Tuple
// 2. Array
//
// ============================================================


// ============================================================
// 5.1 TUPLE
// ============================================================
//
// A tuple groups multiple values of different types
// together into one value.
//
// Example:
//
// (char, i32, bool)
//
// ============================================================

fn comp_tuple() {

    let person: (char, i32, bool) = ('A', 45, true);

    // `:?` is Debug formatting.
    // Tuple does not implement Display,
    // so we use Debug formatting here.
    println!("person = {person:?}");

    // Access tuple elements using index.
    //
    // Tuple indexing starts at 0.

    println!("character = {}", person.0);
    println!("number = {}", person.1);
    println!("boolean = {}", person.2);
}


// ============================================================
// TUPLE INDEX
// ============================================================
//
// person.0 -> 'A'
// person.1 -> 45
// person.2 -> true
//
// ============================================================


// ============================================================
// 5.2 ARRAY
// ============================================================
//
// An array stores multiple values of the SAME type.
//
// An array has a FIXED length.
//
// Example:
//
// [i32; 5]
//
// means:
//
// i32 -> element type
// 5   -> number of elements
//
// ============================================================

fn comp_array() {

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // `:?` is Debug formatting.
    // Array does not implement Display.
    println!("numbers = {numbers:?}");

    // Access array elements using an index.

    println!("first = {}", numbers[0]);// 1
    println!("second = {}", numbers[1]);
    println!("last = {}", numbers[4]);
}


// ============================================================
// 6. STRING AND STRING SLICE
// ============================================================
//
// Rust commonly uses:
//
// String
// &str
//
//
//
// String
// -------------------------
//
// String is an owned, growable UTF-8 string.
//
// Example:
//
// let name: String = String::from("Aditya");
//
//
// &str
// -------------------------
// The & in front means it's a reference — a pointer to text stored somewhere else in memory, rather than owning the text itself.
// &str is a string slice.
//
// Example:
//
// let name: &str = "Aditya";
//
//
//
// We will study String, &str, ownership,
// borrowing and slices separately.
//
// ============================================================
fn string_example() {

    let name: String = String::from("Aditya");
    let name_slice: &str = "Aditya";
    println!("name = {name}");
    println!("name_slice = {name_slice}");

    let num = 42;
    let num_str: String = num.to_string(); // "42"
}


// ============================================================
// 7. COMMON STANDARD-LIBRARY TYPES
// ============================================================
//
// Some important standard-library types are:
//
// String
// Vec<T>
// Option<T>
// Result<T, E>
//
// These are NOT primitive types.
//
// ============================================================


// ============================================================
// Vec<T>
// ============================================================
//
// Vec<T> is a growable collection.
//
// Example:
//
// let numbers: Vec<i32> = vec![1, 2, 3, 4];
//
// ============================================================

fn vector_example() {

    let numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("vector = {numbers:?}");
}


// ============================================================
// Option<T>
// ============================================================
//
// Option<T> represents a value that may or may not exist.
//
// It has two main variants:
//
// Some(value)
// None
//
// Example:
// ============================================================

fn option_example() {

    let age: Option<i32> = Some(21);

    let unknown: Option<i32> = None;

    println!("age = {age:?}");
    println!("unknown = {unknown:?}");
}


// ============================================================
// Result<T, E>
// ============================================================
//
// Result<T, E> represents either:
//
// Ok(value)  -> success
// Err(error) -> failure
//
// Example:
// ============================================================

fn result_example() {

    let success: Result<i32, String> = Ok(100);

    let failure: Result<i32, String> =
        Err(String::from("Something went wrong"));

    println!("success = {success:?}");
    println!("failure = {failure:?}");
}


// ============================================================
// 8. USER-DEFINED TYPES
// ============================================================
//
// Rust allows us to create our own types.
//
// Main user-defined types:
//
// struct
// enum
// union
//
// ============================================================


// ============================================================
// STRUCT
// ============================================================
//
// A struct allows us to create a custom type
// containing related data.
//
// ============================================================

struct User {

    name: String,

    age: u32,
}


// ============================================================
// ENUM
// ============================================================
//
// An enum allows a value to be one of several variants.
//
// ============================================================

enum Status {

    Active,

    Inactive,
}


// ============================================================
// UNION
// ============================================================
//
// A union allows different fields to share the same
// memory location.
//
// Unions are mainly used for low-level programming
// and FFI (Foreign Function Interface).
//
// Reading union fields generally requires unsafe code.
//
// ============================================================

union Data {

    integer: u32,

    float: f32,
}


// ============================================================
// STRUCT / ENUM EXAMPLE
// ============================================================

fn user_defined_example() {

    let user = User {
        name: String::from("Aditya"),
        age: 21,
    };

    println!("User name = {}", user.name);
    println!("User age = {}", user.age);


    let status = Status::Active;

    // Debug output for enum requires Debug implementation,
    // so we simply use match here.

    match status {

        Status::Active => println!("Status = Active"),

        Status::Inactive => println!("Status = Inactive"),
    }
}


// ============================================================
// BIG PICTURE
// ============================================================
//
// Rust Types
//
// ├── Primitive Types
// │
// │   ├── Integer
// │   │   │
// │   │   ├── Signed
// │   │   │   ├── i8
// │   │   │   ├── i16
// │   │   │   ├── i32
// │   │   │   ├── i64
// │   │   │   ├── i128
// │   │   │   └── isize
// │   │   │
// │   │   └── Unsigned
// │   │       ├── u8
// │   │       ├── u16
// │   │       ├── u32
// │   │       ├── u64
// │   │       ├── u128
// │   │       └── usize
// │
// │   ├── Floating Point
// │   │   ├── f32
// │   │   └── f64
// │
// │   ├── bool
// │   └── char
// │
// ├── Compound Types
// │   ├── Tuple
// │   └── Array
// │
// ├── String / Slice Types
// │   ├── String
// │   ├── &str
// │   └── &[T]
// │
// ├── Standard Library Types
// │   ├── Vec<T>
// │   ├── Option<T>
// │   └── Result<T, E>
// │
// └── User-Defined Types
//     ├── struct
//     ├── enum
//     └── union
//
// ============================================================
// MAIN
// ============================================================

fn main() {
    numbers_i();
    numbers_f();
    number_b();
    number_c();
    comp_tuple();
    comp_array();
    string_example();

    vector_example();
    option_example();
    result_example();
    user_defined_example();
}