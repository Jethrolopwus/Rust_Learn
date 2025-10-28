

fn main() {
//   loop  and label //
'outer: loop {
            loop{

                println!("this is an infinit loop: ");
                break 'outer;
            }
    }

    let a: i32 = loop{
        break 5;
    };
    println!("The value of a is: {:?} ", a);

    // For Loop //
    let vec: Vec<i32> = vec![30, 40,50,60,80];
        for i in vec {
            println!("{i}");
        }

        // While loops

    let mut num: i32 = 0;
    while num < 10 {
        num = num + 1;
        }
        println!("the value of num is: \n {:?}  ", num);

}
