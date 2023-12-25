use std::io;
//io -i/p o/p module, std - standard library (is a crate)

fn main() {
    //fn -func
    //main func is special func in RUST that gets executed automatically when you run RUST.
    //main is the entry point to the code and anything inside of this func will run immediately

    //print line
    //Exclamation means we are running something called a macro

    println!("Hello, world, how are you doing!");

    // ---------------------------------------------------------------------------------------------------
    // ?VARIABLES CONSTANTS and SHADOWING
    // STATIC VARIABLE
    let x = 4; //Here compiler implicitly decides the type, here compiler decides it as type int. You cannot change the type of variable throughout the program.
    let x1: u8 = 4; //Explicitly decides the type -unsigned int8 type

    println!("{} {}", x, x1); //4 4 ---- "{} {}" this is how you format strings, embed variable in string.

    // IMMUTABLE
    // Try change the value x to 5
    // x = 5; //this will give an ERROR - you cannot assign twice to immutable variable

    // MUTABLE VARIABLE
    // So to create a mutable variable you do this:
    let mut y = 10;
    println!("Print y here: {}", y); //10

    y = 11;
    println!("Print y now: {}", y); //11

    // OVERWRITE A VARIABLE
    let m1 = 100;
    println!("Print m1 {}", m1); //100
    let m1 = 200;
    println!("Print m1 {}", m1); //200

    // NAME SHADOWING - here you use the same variable name but from a different scope
    // CREATE YOUR OWN SCOPE TO EXPLAIN THIS EXAMPLE

    {
        let m1 = 27;
        println!("Printing m1 inside scope {}", m1); //27
    }

    let m1 = m1 + 100;
    println!("Print m1 {}", m1); //it's 300 and not 127

    // m1 = 27 is in a different scope, implies it's 27 inside the curly braces
    // For m1 = m1 + 100 --- it adds m1 in it's scope with 100, i.e; 200 + 100 = 300

    // Usage of x from the exterior scope in the interior scope
    {
        let m1 = m1 * 2;
        println!("print m1 in this second scope {}", m1); //600
    }

    //CAN CHANGE TYPE DURING VARIABLE RE-DEFINE
    let m1 = "Now lets make this a string.";
    println!("This is a string m1 - {}", m1);

    // BUT THIS WILL GIVE AN ERROR IF YOU USE mut keyword
    let mut define_int = 47;
    println!("This is define_int - {}", define_int);

    define_int = 57;
    println!("This is define_int - {}", define_int);

    // define_int = "47"; //This gives an ERROR ---- expected int but found type string. Here we did not make a new variable but just changed the type
    // println!("This is define_int string- {}", define_int);

    let define_int = "47"; //This works, as we define it as new variable
    println!("This is a new variable define_int {} ", define_int);

    // CONSTANTS
    // convention of writing constants - caps with underscores and define the type
    const PI_VALUE: f32 = 3.14;
    println!("Print pi value {}", PI_VALUE);

    // PI_VALUE = 3.17; //This will give an ERROR
    // const PI_VALUE:f32 = 3.19; //This will give an ERROR

    // ---------------------------------------------------------------------------------------------------

    //? PRIMITIVE DATA TYPES
    // signed int 32 is a default value for an integer in RUST

    let new1: u8 = 24;
    println!("print new1 {}", new1);
    let new1: u32 = 100;
    println!("print new1 {}", new1);
    // new1:u64 = 10000; //gives an ERROR, add let keyword

    let mut new2: u8 = 20;
    println!("print new2 {}", new2);

    // new2: u32 = 104; // gives an ERROR, add let keyword

    // FLOAT
    // default is f32

    // BOOLEAN
    let true_or_false: bool = true;
    let false_or_true: bool = false;

    // CHAR
    let character: char = '9'; //single character --- requires single quote
    let character: char = ';'; //single character

    // COMPOUND TYPES
    // You have to define the type of the tuple
    let tt: (char, i32, bool) = ('s', 1, true);
    // println!("Print tt {}", tt); //this will not work
    println!("Print tt {}", tt.0); //"s"
    println!("Print tt {}", tt.1); //1
    println!("Print tt {}", tt.2); //true

    let t1: (char, i8, bool) = ('e', 2, false);
    // println!("Print t1 {}", t1); //this will not work
    println!("Print tt {}", t1.0); //"s"
    println!("Print tt {}", t1.1); //1
    println!("Print tt {}", t1.2); //true

    let t1 = tt;
    println!("Print t1 {}", t1.0);

    // tt.0 = 'r'; //will give an ERROR because it's not mutable

    // Create a mutable tuple
    let mut tuple1: (char, i32, bool) = ('y', 45, true);

    tuple1.0 = 'z';
    println!("Print tuple1 {}", tuple1.0);

    tuple1 = ('i', 10, true);
    println!("Print tuple1 {}", tuple1.0);
    println!("Print tuple1 {}", tuple1.1);
    println!("Print tuple1 {}", tuple1.2);

    // ARRAY
    // You can't add ele into the arr on go. For that you need to add new array variable with new size.
    let mut arr = [1, 2, 3, 4];
    arr[3] = 5;
    println!("{}", arr[3]);

    let mut arr2: [i32; 5] = [10, 20, 3, 500, 67];

    // let mut arr2 :[i32;5]; //ERROR
    // let mut arr2 :[i32;5] = []; //ERROR

    let w1: u8 = 4;
    // let w2: u32 = w1; //this will give an ERROR
    let w2 = w1; //type of w2 is int 8

    println!("Print w2 and w1 {} {}", w2, w1);

    // Special case
    // Strings
    let string_example = String::from("Hi There"); //String::from implies the string is mutable, you can add or remove characters, can change the size
    println!("Print w2 string_example {}", string_example);

    // ----------------------------------------------------------------------------------------------------
    // CRATE, PRELUDES, MODULES

    // Create a mutable string
    // We initialize the value using new()
    let mut input = String::new();

    // get an input
    // read a line of input from the user and store it in the mutable variable input
    // read_line is a method called on the standard input stream. It reads a line from the input and appends it to the provided String.
    // &mut input is a mutable reference to a String variable (input in this case).

    // io:stdin().read_line(input); //this is pass value - the input variables copy will be created and the input from user is stored in the copied variable.
    // io:stdin().read_line(&input); //this pass by reference, but this way of writing is immutable, hence you add &mut

    // Takes in a user input from command line
    // When we use read_line() our user input type should be String
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("{}", input);

    // ----------------------------------------------------------------------------------------------------
    // ARITHMETIC AND TYPE CASTING

    let v1: u8 = 200; // 0 to 255
    let v2: i8 = 10; //-128 to 127

    // let v3 = v1 + v2; //ERROR --- type different

    let f1: f32 = 12.0; //12 won't work, it should be 12.0
    let f2: f32 = 14.3;
    let f3 = f1 + f2;
    println!("Print f3 {}", f3);

    let u1: u8 = 255;
    let u2: u8 = 10;
    // let u3 = u1 + u2; //ERROR = attempt to compute `u8::MAX + 1_u8`, which would overflow --- don't have enough bits to represent the result of summation of u1 and u2.
    // let u3 = u2 - u1 //ERROR

    let u3 = u1 / u2; //integer output
                      // let u3:f32 = u1 / u2; //ERROR --- because this operation return uint type integer
    println!("Print u3 {}", u3);

    let mod1 = u1 % u2;
    println!("Print mod1 {}", mod1);

    // The default float type in RUST is f64
    // To assign it as f32
    let ex1 = 255.0f32;
    println!("Print ex1 {}", ex1);

    let ex2 = 255.0_f32; //another way
    println!("Print ex2 {}", ex2);

    // To assign it as i8, by default int is i32
    let ex3 = 120_i8;
    println!("Print ex3 {}", ex3);

    let ex4 = 121i8;
    println!("Print ex4 {}", ex4);

    let ex5 = 121_123 as i64; //this is int 1,21,123
    println!("Print ex5 {}", ex5);

    let ex6 = 10 as i32;
    println!("Print ex6 {}", ex6);

    // TYPE CASTING
    // EXPLICIT TYPE CONVERSION
    let req = ex5 / (ex6 as i64);
    println!("Print req {}", req);

    // This method where reducing the bit, can also cause overflow ---- so make sure you are in the range
    let req2 = (ex5 as i32) / ex6;
    println!("Print req2 {}", req2);

    let expt1 = (i32::MAX as i64);
    println!("Print expt1 {}", expt1); //2147483647 , adding 1 to expt1 creates an overflow

    let r1 = (i32::MAX as i64) + 1; //max value that cab be represented by 32 bit signed int. it is then cast to an i64 before adding 1 to it. This addition results in an overflow, as the value exceeds the maximum representable value for an i32. r1 is i64
    let r2 = r1 as i32 / ex6; //over flow value r1 is cast back to i32
    println!("Print r2 {}", r2); //Print r2 -214748364

    // When over flow occurs - we use 2's complement wrapping

    // CONVERT STRING TO INTEGER

    let mut string_new = String::new();
    io::stdin()
        .read_line(&mut string_new)
        .expect("expected to read line");

    let int_input: i64 = string_new.trim().parse().unwrap(); //trim removes the new line ch or esc char.

    println!("Print int_input {}", int_input); //Print r2 -214748364

    // ---------------------------------------------------------------------------------------------------
    // CONDITIONS AND CONTROL FLOW

    let con = 2 < 3;
    println!("Print con {}", con);

    let con1: bool = 2 <= 2;
    println!("Print con1 {}", con1);

    // let con2:bool = 100 < 100.1; //ERROR

    // Type cast and then compare
    let con2: bool = (100 as f32) < 100.1;
    println!("Print con2 {}", con2);

    // COMPOUND CONDITIONS

    // AND OPERATOR
    let con3 = true && con2;
    println!("Print con3 {}", con3); //true

    // OR OPERATOR
    let con4: bool = (24 as f32) > 100.0;

    let con5: bool = true || con4;
    println!("Print con5 {}", con5); //true. Either of  them true, result is true

    // NOT OPERATOR
    let negate_op = !con5;
    let negate_op2 = false || !con5;
    println!("Print negate_op {}", negate_op); //false
    println!("Print negate_op2 {}", negate_op2); //false

    // CONTROL FLOW
    let study: &str = "RUST";

    if study == "RUST" {
        println!("Learning {}", study);
    }

    if study != "RUST" {
        println!("Not Learning {}", study);
    } else if study == "C" {
        println!("Okay Learning {}", study);
    } else {
        println!("Learning something");
    }

    // ---------------------------------------------------------------------------------------------------
    // FUNCTION, EXPRESSIONS & STATEMENTS
    // Naming convention - func - Snake case

    // func call
    divide_number(100, 1001);

    // EXPRESSION
    let test_expression = {
        let x = 3;
        x + 4 //adding ; here will give an error
    };

    println!("Print test_expression {}", test_expression);

    // FUNCTION RETURN VALUE - IMPORTANT
    let result_add = add_number(30, 40);
    println!("Print result_add {}", result_add);

    let result_mult = multiply_numbers(30, 2);
    println!("Print result_mult {}", result_mult);
}

fn divide_number(a: i32, b: i32) {
    println!("Division result: {}", b / a);
}

fn add_number(a: i32, b: i32) -> i32 {
    a + b // you should not add semicolon while return
}

fn add_numbers(a: i32, b: i32) -> i32 {
    return a + b; // if you have return keyword then add ;
}

// You have to specify the type of return value if a value is being returned
fn multiply_numbers(a: i32, b: i32) -> i32 {
    let result = a * b;

    if result > 30 {
        return result - 1; //early return needs a ; else it will display error
    }
    result
}
