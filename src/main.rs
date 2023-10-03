/* Hello World for Rust

main function is always called first in Rust. Function is indicated with keyword "fn". E.g fn main().

Create a file called main.rs for Hello World
Compilation:

1. To compile rust, navigate to the src folder and run 
                    rustc main.rs

    This will create two files:
        1. main.exe - Binary executable
        2. main.pdb - Contains debugging information

*/



fn main() {
    /* 1. Rust style is to indent with four spaces and not tabs - Although this does not affect the build.
       2. println! is a Rust Macro. A function call in Rust would not have ! at the end of its name
       3. String in args "Hello World" prints the argument. Try changing it to your name instead!
       4. ; is used to expression is over and next one is ready to begin  */ 
    println!("Hello, world!"); 
}
