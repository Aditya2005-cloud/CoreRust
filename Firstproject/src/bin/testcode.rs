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

fn main() {
    let name = String::from("Aditya");
    print_name(name);
    
    println!("{}", name); // ❌ this will NOT compile — why?
}

fn print_name(n: String) {
    println!("{}", n);
}