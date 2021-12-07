fn main() {
    // converting a string to a numeric type [have to add a type annotation]
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("the guess is {}", guess);


    // FLOATING POINT TYPES
    let x = 2.0;            //f64
    let y: f32 = 23.0;      //f32



    // NUMERIC OPERATIONS
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 3 * 2;

    // division
    let quotient = 45.2/23.1;
    let floored = 2 / 3;    // Results to 0;

    // remainder
    let remainder = 43 % 5;



    // BOOLEAN TYPE
    let f: bool = false;


    // CHARACTER TYPE
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';



                                    // COMPOUND TYPES
    println!("\n \t COMPOUND TYPES \n TUPLES", );
    // TUPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("[ destructuring ] The value of y is {}", y);
    println!("[period] tupple values are ({}, {}, {}) \n \nARRAYS", tup.0, tup.1, tup.2);


    // ARRAY
    let a: [i32; 5] = [1,2,3,4,5];

    println!("accessing first element {} and fourth element {}", a[0], a[3]);
}