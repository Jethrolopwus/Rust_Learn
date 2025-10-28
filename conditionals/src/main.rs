


fn main() {

    // If else //
    let number: i32 = 50;
    if number < 60 {
        println!{"The number is lesser than 60!"};
    } else {
        println!{"The number is greater than or equal to 60"}
    
    }
    // if else If ladder // checks multiple conditions in sequences

    let scores: i32  = 100;


    let  grade: char =  if scores >= 90 {
        'A'
    } else if scores >= 80 {
         'B'
    } else if scores >= 70 {
         'C'
    }else {
     'F'
    };

    println!("the values of grade is: {:?} ", grade);

    // match expression //
      let player_mark: i32 = 95;
    let grades = match player_mark {
        // these are the arms //
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',       
    };
    println!("The grades are as follows: {}", grades); 
}
 
