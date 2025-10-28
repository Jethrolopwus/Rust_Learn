



fn main() {
// Arithmetic operators // they perform math ops on numeric types like i32 and f64.

// +, -,*, /, %

println!("The remainder after dividing 17 by 5 is:{} ", 17 % 5);
    // comparison operators // compare two values and return a boolean value 

    let x: i32 = 10;
    let y: i32 = 20;

    println!("x == y: {}, x != y:{}. x > y: {}, x < y: {}, x >= y: {}, x <= y:{} ",
    x == y,
    x != y,
    x < y,
    x > y,
    x <= y,
    x >= y,
);

// Logical operators //
// AND: &&, OR: ||, NOT: !

let num1: i32 = 20;
let num2:i32 = 30;
if (num1 > 10) && (num2 < 35) {
    println!("This rule is satisfied");
}else {
    println!("Rule not satisfied");
}

// assignment Operators // use to update a particular value in a variable
//  +=, -=, *=, /=, %=

let mut x:i32 = 8;
x += 2;
println!("The addition of x is : {:?}", x);

x -= 2;

println!("The subtraction of x is : {:?}", x);

x *= 2;

println!("The multiplication of x is : {:?}", x);

x /= 2;

println!("The diviion of x is : {:?}", x);
x %= 2;

println!("The modules of x is : {:?}", x);


// Bitwise Operator // these performs opeartions at the binary level manipulating the individual bits of the integer.

    let x:u8 = 4;
    println!("the AND of x is: {}", x & x);

    let y:u8 = 4 << 2;
    println!(" the left shift of y to 1 is: {y}");


}
