fn main(){
    let a=5;
    println!("a is {a}");// in print {} is a place holder for a variable or a value
    let line = "-".repeat(20);
    println!("{line}");
    println!("{}","-".repeat(20));//or
    // println!("{"-".repeat(20)}");this wont work
    println!("a is {}",a);
}
//Moved into a src/bin/ folder so Cargo treats each as its own separate program
// and to run it cargo run --bin (your file name)
// whatever is in the bin is a total seperate file from the main file and u have to run it separately
