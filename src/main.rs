mod print;
mod var;
mod types;

fn main() {
    print::run();

    //Formatting
    println!("Number: {} {}", 1,2);
    // println!("Hello, world!");

    //Positional Argument
    println!("{0} what do you {1} {0}", "Brad", "what" );

    //Named Arguments

    println!("{name} of the boy", name = "saksham");

    //Placeholder traits
    println!("Binary {:b}, Octal {:o}, hexadecimal: {:x}", 10, 10, 10);

    //Placeholder for debug trait

    println!("{:?}", (1, "dasd", true));

    //math
    println!("10 + 10 = {}", 10+10);

    var::run();
}
