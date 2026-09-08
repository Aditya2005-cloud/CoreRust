//The ; at the end tells Rust "this instruction is complete." Rust requires a ; after most lines
// fn is a function
// main is the name of the function that is called when you run the program and is the entry point
fn main(){//Rust calls main() for you automatically
    println!("Hellow World");//! right after it means this is a macro, not an ordinary function. 
    //For now, just remember: anytime you see something!(...), it's a macro, and println! specifically is how you print text.
    //  this is a macro. Don't overthink the ! yet.
    call();
}
//Remove-Item *.exe, *.pdb -Force 
//del *.exe, *.pdb -Force
//this are the commands to remove the files from power shell
fn call(){
    println!("Hellow World calling from function call method");
}

// main(); you can't call main() 
// error: aborting due to 1 previous error