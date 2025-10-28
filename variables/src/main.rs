

fn main() {
    // variable declaration with type anotation //

    let number: i32 = 42;
    let float_variable: f64 = 3.7;
    let boolean_variable: bool = true;

    println!("The number is: {}", number);

    println!("The float variable is: {}", float_variable);

    println!("The boolean variable is: {}", boolean_variable);

    // variable mutability //

    let mut x: i32 = 50;
    println!("The Initial value of x is: {}", x);

    x = 20;
    println!("The updated x is: {}", x);

//    scope // 
        {
            let num: i32 = 20;
            let new = num;
            println!("The value of Num is: {} ", new);

        }

        // let new =num; // this is not allowed 


        // shadowing // you can declar a new variable with the same name as the previous one, the first variABLE IS Shadowed,
        // which means the second variable is what the compiler sees when you use the variable name.

     let x = 6;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in outer scope is: {x}"); 
        

        // CONSTANT // always in capital letters and cannot be mutatted once created
        const MAX_VALUE: u32 = 200;
        println!("The constant value is: {}", MAX_VALUE);
   
}
