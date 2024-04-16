use std::io;

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

    //annotations
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Hello, world! {guess}");

    //SCALAR TYPES
    //integer overflow
    let x: u8 = 255;
    println!("Hello, world! {x}");
    let x: i8 = -128;
    println!("Hello, world! {x}");
    //Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, 
    //where n is the number of bits that variant uses. 
    //So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 
    //Unsigned variants can store numbers from 0 to 2n - 1, 
    //so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    //byte example u8 only
    let x: u8 = b'A';
    println!("Hello, world! {x}");

    //floating point all signed
    let z = 2.0; //f64 the default => double precision
    println!("Hello, world! {z}");
    let za: f32 = 3.0; //f32 => single precision
    println!("Hello, world! {za}");

    //numeric operation
    //addition
    let sum = 5 + 10;
    println!("Hello, world! {sum}");
    //subtraction
    let difference = 95.5 - 4.3;
    println!("Hello, world! {difference}");
    //multiplication
    let product = 4 * 30;
    println!("Hello, world! {product}");
    //division
    let quotient = 56.7 / 32.2;
    println!("Hello, world! {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("Hello, world! {truncated}");
    //remainder
    let remainder = 43 % 5;
    println!("Hello, world! {remainder}");

    //boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Hello, world! {t}");
    println!("Hello, world! {f}");

    //character type
    let c = 'z';
    let z_char: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("Hello, world! {c}");
    println!("Hello, world! {z_char}");
    println!("Hello, world! {heart_eyed_cat}");

    //COMPOUND TYPES
    //tuples => can contain different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Hello, world! {:?}", tup);
    //print one of the values
    println!("Hello, world! {}", tup.0); //(.) called period and the first index in tuple is 0
    //another way of tuple
    let tup_1 = (500, 6.4, 1);
    let (x, y, z) = tup_1;
    println!("Hello, world! {x} {y} {z}");

    //array => fixed length *in rust and same types
    let a = [1, 2, 3, 4, 5];
    println!("Hello, world! {:?}", a);
    //print one of the values
    println!("Hello, world! {}", a[0]);
    //example fixed length is month
    let month = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("Hello, world! {:?}", month);
    //using type
    let a: [u32; 5] = [1, 2, 3, 4, 5]; //type is u32 and length is 5
    println!("Hello, world! {:?}", a);
    //assign value
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("Hello, world! {:?}", a);
    //access array element
    println!("Hello, world! {}", a[0]); //index start from 0

    //invalid element access sample game
    let a = [1, 2, 3, 4, 5];
    println!("please enter array index");
    let mut index = String::new();
    //read line
    io::stdin().read_line(&mut index).expect("Failed to read line");
    //check type
    let index: usize = index.trim().parse().expect("Index entered was not a number"); //by using usize type, length is limit to architecture
    //print value
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
