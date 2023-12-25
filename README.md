1. How to compile the RUST file?
    > > rustc main.rc
    > > This will create an executable file for a particular OS, if you compiled it on Linux, this executable file can be run on Linux OS.
2. How to run the executable file in Linux?
    > > ./main
3. The above commands are useful when you did not setup your project using cargo.
4. How to setup the project using Cargo?
   type:
    > > cargo projectName in command prompt
    > > In this project it is cargo tutorial1
    > > This creates a tutorial1 folder with an src folder, cargo.toml, .gitignore
    > > src folder will have a main file in it.
5. How to compile a project made using cargo?
   Navigate into the tutorial1 folder and type
    > > cargo build
    > > This will create an extra folder called target and a file Cargo.lock
6. How to run this project?
   Navigate into target/debug
   Then you will find an executable file of the project ---in this example, tutorial1
   type :
    > > ./tutorial1
    > > It will print Hello world written in main func on the command line.
7. In the above code you need to build it manually every-time with a code change, like you will have to do cargo build --- it will create a new build file, navigate to the build file --- ./target/debug then run the build file ---- ./tutorial1 (in this example project is named tutorial1)
   So to avoid this long process you can use the following command:
    > > > cargo run (use this by navigating to tutorial1 folder)
    > > > This will compile and automatically run the code.
8. How can you check if your code has any syntactical error?
    > > cargo check
    > > This will check if there are any errors in your code
9. How can you format your code?
   Navigate to the file, in this example navigate to main file----/rust/tutorial1/src. Then type:
    > > rustfmt filename (rustfmt main.rs) ---------this will format the file

<!-- -----VARIABLES------ -->

10. By default in RUST all variables that we define are immutable (cannot change them).
11. To create mutable variables you need to add mut syntax;
    > > let mut y = 19;
12. A constant's value or type cannot be changes throughout the program once it's defined.

<!-- ------PRIMITIVE DATA TYPES--------------- -->

13. RUST has 2 categories of primitive data type:

    > > Scalar data type
    > > It's a single value, int boolean etc
    > > Compact data type
    > > ex: Array, tuple

14. For signed integers:

    > > i8 --- you can represent numbers from -2^7 to 2^7 - 1 (-128 to 127)
    > > i16
    > > i32
    > > i64
    > > i128

15. For unsigned integers:
    > > u8 --- you can represent numbers from 0 to 2^8 - 1 (0 to 255)
    > > u16
    > > u32
    > > u64
    > > u128
16. Floating point type:

    > > f32
    > > f64

17. COMPOUND TYPES

    > > Fixed length sequence of element that's immutable.

18. PRELUDE: is a list of things that Rust automatically imports into every Rust program. It's kept as small as possible and is focused on things particularly traits which are used in almost every single Rust program.

19. CRATE - a package or a library

20. MODULES - are within CRATES

21. Anything in RUST that return or give you a value is an expression.

    > > calling a fun
    > > printing
    > > Conditions

22. Statements doesn't return anything, example:

    > > fn divide_number(a: i32, b: i32){}, divide_number(a: i32, b: i32) is a statement
    > > let x = 3; this is a statement

23. Expression - return a value

-   func call, conditions
    expression example:
    test_expression = {
    let x = 3;
    x + 4
    };
    Here let test_expression = 7 is a statement but {
    let x = 3;
    x + 4
    }; is an expression.
    x + 4 does not have ; at the end.
    Implies the expression return a value which is assigned to variable test_expression.

24. MEMORY MANAGEMENT
    > > Stack is the fastest, because all the information associated with the current scope that we are in is at the top of the stack. Can only hold information that has a known and fixed size.
    > > Heap is useful dynamic sized objects. Search through the heap to allocate space for the dynamic sized object, this consumes time - Memory allocator. The variable name is stored in the stack and we have a pointer that points to the location where the string is available on heap.
    > > ex: let string_example = String::from("Hi there");
    > > string_example - stored on stack with a value that points to the address in which "Hi there" is stored on heap.
