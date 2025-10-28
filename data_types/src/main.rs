fn main() {
    // primitive or scalar data types //

    let unsigned_integer: u32 = 30;

    let signed_integer: i16 = 20;

    let float_num: f64 = 5.5;

    let boolean: bool = false;

    let character: char = 'A';

    // platforms specifics integers which are pointer unsigned and signed integers. //
    let arch_1: usize = 10;
    let arch_2: isize = 5;

    println!("These are the values of primitive data types; signed and unsigned integer, float_numbers, boolean and characters and paltforms specifics: \n{}, {}, {}, {}, {}, {}, {}", unsigned_integer, signed_integer, float_num, boolean, character, arch_1, arch_2);
    //  type alias //
    type Age = u8;
    let john_age: Age = 45;
    println!(" John age is: {}", john_age);

    // type conversion //

    let num_1: i32 = 10;
    let num_2: f64 = num_1 as f64;
    println!("The value of num_2 is: {}", num_2);



    // compound data types // these are data types that can hold multiple values

    // 1. &str(slice):- this is immutable and fix size and String:- can be modified and not fixed in size.
    let fixed_str: &str = "This is a fixed length string!";

     let mut growable_string: String = String::from("This is a flexible string.");
        growable_string.push_str(" It can grow in size.");
        println!("This is the values of strings: \n {}, \n {}", fixed_str, growable_string);


        // ARRAYS // holds multiple values but of same types and fixed size.

        let mut array_1: [i32; 4] = [2,4,6,8];
        let num_3: i32 = array_1[2];
        println!(" the entire array values are : {:?}", array_1);

        let array_2: [i32; 10] = [1; 10];
        println!("this is the value of array_2: {:?}", array_2);


        //  VECTORS //
        let vec_1: Vec<i32> = vec![1,2,3,4,5,6];
        let num_2: i32 =vec_1[1];

        println!("The vector value is: {:?}", vec_1);

        // TUPLES // this hold values of different types.

        let personal_info:(&'static str, i32, &'static str, i32, bool) = ("Salary", 4000, "Age", 30,  true);
        println!("the tupple value: {:?}", personal_info);
        let my_slary: i32 = personal_info.1;
        println!("my salary is {:?}", my_slary);
        // destructuring tuples //

        let (salary, salary_amount, age, age_value, is_active ) = personal_info;
        println!("the destructured value of tuples are: \n{:?}",personal_info );













}
