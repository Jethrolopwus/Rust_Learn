

fn my_func (x: &str)  {
    println!("{x}");

}


fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}

fn math_calculus(num1: i32, num2: i32) -> (i32, i32, i32) {
   (num1 + num2, num1 - num2, num1 * num2)
   
}

    // mutability of function parameters //

    fn mutable_param(mut x: i32) {
        x += 10;
        println!("the value of mutable parameter is: {}", x);
    }



fn main() {
  my_func("this is a function");

  let string: &'static str = "this is a function of string slice type stored  in a variable and called";

  my_func(string);

    let answer = multiplication(5, 6);
    println!("The multiplication answer is: {}", answer);

    // calling the math_calculus function  by declaring a variable//
    let result: (i32, i32, i32)  = math_calculus( 20,  5);
    println!("The math calculus results are:{:?} ", result);

    // calling the mutable_param function //
    let mut num: i32 = 15;
    mutable_param(num);
    println!("the value of num after calling mutable_param function is: \n {}", num);



}
