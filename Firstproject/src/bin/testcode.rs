// fn main(){
//     let a=5;
//     println!("a is {a}");// in print {} is a place holder for a variable or a value
//     let line = "-".repeat(20);
//     println!("{line}");
//     println!("{}","-".repeat(20));//or
//     // println!("{"-".repeat(20)}");this wont work
//     println!("a is {}",a);
// }


//Moved into a src/bin/ folder so Cargo treats each as its own separate program
// and to run it cargo run --bin (your file name)
// whatever is in the bin is a total seperate file from the main file and u have to run it separately


// fn main() {
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
//     println!("{}", numbers[10]); // there is no index 10 — only 0 to 4 exist
// }


// fn main() {
//     let mut name: String = String::from("Aditya");
//     println!("{}", name);
// }


// Modifying a String
// fn main() {
//     let mut name: String = String::from("Aditya");
    
//     name.push_str(" Saha");
//     println!("{}", name);
// }

// fn main() {
//     let mut name: &str = "Aditya";
    
//     name.push_str(" Saha"); // ❌ this will NOT compile
//     println!("{}", name);
// }

// fn main() {
//     let a = String::from("Aditya");
//     let b = a; // ownership MOVES from a to b
    
//     println!("{}", a); // ❌ this will NOT compile
// }

// fn main(){
//     let a= String::from("Aditya");
//     let b=a.clone();
//     println!("{}",a);
//     println!("{}",b);
// }


// so Rust just copies them automatically instead of moving them. No .clone() needed; it happens silently because copying them is cheap.
// fn main() {
//     let x = 5;
//     let y = x;
//     println!("{}", x); // this DID work, no error — why?
// }

// fn main() {
//     let name = String::from("Aditya");
//     print_name(name);
    
//     println!("{}", name); // ❌ this will NOT compile — why?
// }

// fn print_name(n: String) {
//     println!("{}", n);
// }
// fixed
// fn main() {
//     let name = String::from("Aditya");
//     print_name(&name); // borrow, don't move
    
//     println!("{}", name); // ✅ this works now
// }

// fn print_name(n: &String) {
//     println!("{}", n);
// }


// fn main() {
//     let student: (i32, char) = (85, 'A');
//     println!("{:?}", student);
// }

// fn main(){
//     let grid: [[i32; 2]; 2] = [[1, 2], [3, 4]];
//     println!("{:?}", grid);
//     println!("{}", grid[0][0]); // 1  — outer index 0, inner index 0
//     println!("{}", grid[0][1]); // 2  — outer index 0, inner index 1
//     println!("{}", grid[1][0]); // 3  — outer index 1, inner index 0
//     println!("{}", grid[1][1]);
// }

// fn main() {
//     let a: &str = "Aditya";              // String slice
//     let b: String = String::from("Aditya"); // String
    
//     println!("{}", a);
//     println!("{}", b);
// }

// fn main() {
//     let a = String::from("Aditya");
//     let b = a.clone(); // makes a full separate copy
    
//     println!("{}", a); // ✅ works — a still owns its own data
//     println!("{}", b); // ✅ works — b owns a separate copy
// }

// fn main() {
//     let a = String::from("Aditya");
//     let b = a; // ownership MOVES from a to b
    
//     println!("{}", a); // ❌ ERROR — a no longer owns anything
// }

// fn main() {
//     let name = String::from("Aditya");
//     print_name(&name); // borrow — just lend access
    
//     println!("{}", name); // ✅ works — name never lost ownership
// }

// fn print_name(n: &String) {
//     println!("{}", n);
// }

// fn main(){
//     let original = String::from("Aditya");
//     let copy = original.clone();
//     println!("{}", original);
//     println!("{}", copy);
//     greet(&original);//without & you'd be moving ownership of original into the function
//     greet(&copy);
// }

// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }


// fn main() {
//     let original = String::from("Aditya");
//     take_it(original); // this MOVES ownership away from original
    
//     println!("{}", original); // what happens here?
// }

// fn take_it(name: String) { // takes ownership (String, not &str)
//     println!("Got: {}", name);
// }

// fn main() {
//     let mut numbers: Vec<i32> = vec![1, 2, 3];
//     println!("{:?}", numbers);
//     numbers.push(4);// to add item in vec as its a growable collection we cna use .push()
//     println!("{:?}", numbers);
//     let removed = numbers.pop();//removes the last item from the Vec and hands it back to you
//     println!("{:?}", numbers);
//     println!("{:?}", removed);
// }
//.pop() doesn't return a plain i32. It returns something called an Option<i32> — which can be one of exactly two things:
// Some(4) — "yes, there was a value, and it's 4"
// None — "no, there was nothing to give you" (this is what you'd get if you called .pop() on an empty Vec)

// fn main() {
//     let mut empty_list: Vec<i32> = vec![];
//     let removed = empty_list.pop();
//     println!("{:?}", removed);// None will be printed
// }

// fn main() {
//     let some_number: Option<i32> = Some(5);
//     let no_number: Option<i32> = None;
    
//     println!("{:?}", some_number);
//     println!("{:?}", no_number);
// }



// fn main() {
//     let some_number: Option<i32> = Some(5);
    
//     match some_number {//match. Think of it as Rust's way of saying: "look at what this actually is, and run different code depending on which case it turns out to be.
//         Some(value) => println!("Got a value: {}", value),
//         None => println!("There was nothing"),
//     }
// }

// fn main() {
//     let some_number: Option<i32> = Some(5);
//     let no_number: Option<i32> = None;
    
//     println!("{}", some_number.is_some());
//     println!("{}", no_number.is_none());
// }

// fn main() {

//     let success: Result<i32, String> = Ok(100);

//     let failure: Result<i32, String> =
//         Err(String::from("Something went wrong"));

//     println!("success = {success:?}");
//     println!("failure = {failure:?}");
// }


// fn main() {
//     let good: Result<i32, std::num::ParseIntError> = "42".parse();
//     let bad: Result<i32, std::num::ParseIntError> = "abc".parse();
    
//     println!("{:?}", good);
//     println!("{:?}", bad);
// }

fn main() {
    let mut a: String = String::from("Adiya");
    
    a.push_str(" Saha"); // adds a whole &str
    a.push('!');          // adds just ONE character (single quotes)
    
    println!("{}", a);
}