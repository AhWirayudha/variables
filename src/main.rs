fn main() {
    //mutable variables
    let mut x = 5;
    println!("Hello, world! {x}");
    x = 6;
    println!("Hello, world! {x}");

    //shadowing variables
    let y = 5;
    //scope of y is limited to this block
    {
        let y = 6;
        println!("Hello, world! {y}");
    }
    println!("Hello, world! {y}");

    //why shadowing not mutable?
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Hello, world! {spaces}");

    //constants
    const MAX_POINTS: u32 = 100_000;
    println!("Hello, world! {MAX_POINTS}");
}
