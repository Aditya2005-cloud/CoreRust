//Good breakdown overall:- Basic Data Types

//Primitive data type
// Integer ,Unsigned integer:-
//Rust has signed (+ and -) and unsigned (+ only) integers.
//i8   i16   i32   i64   i128   isize is a signed integer
//u8   u16   u32   u64   u128   usize is an unsigned integer

//Float point
//Boolean
//Character or char
//Scalar (primitive) types — a single value


//Compound types — group multiple values, fixed structure
//Tuple
//Array

// Common standard-library types — built using the above
//String
//&str
//Option<T>
//Result<T, E>


// User-defined types — you define the shape
//struct
//enum
//union — mainly for low-level/FFI use



// The big picture
// Think of Rust's type system like this:

// Rust Types
// │
// ├── Primitive / built-in
// │   ├── Integers
// │   │   ├── i8 i16 i32 i64 i128 isize
// │   │   └── u8 u16 u32 u64 u128 usize
// │   ├── Floating point
// │   │   ├── f32
// │   │   └── f64
// │   ├── bool
// │   └── char
// │
// ├── Compound
// │   ├── Tuple
// │   └── Array
// │
// ├── Common standard-library types
// │   ├── String
// │   ├── &str
// │   ├── Vec<T>
// │   ├── Option<T>
// │   └── Result<T, E>
// │
// └── User-defined
//     ├── struct
//     ├── enum
//     └── union
