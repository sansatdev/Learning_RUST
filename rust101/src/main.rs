// This is a comment 
/* This is a multiline comment
    ends here*/

//Constants - const 
const BIRTHDAY: i32 = 1;
const NUMBER: i32 = 21;
fn main() {
    println!("This is first line!");
    println!("Hello, world!"); //This prints hello world!

    // Scaler Types : int , float , boolean , char 
    // unsigned - never negative - u8 , u16 , u32 , u64 , u128 , usize
    // signed - can be postive and negative - i8 , i16 , i32 , i64 , i128 , isize
    println!("Working with Scaler Type ");
    println!("Max size of a u32 : {}",u32::MAX);
    println!("Max size of a u64 : {}",u64::MAX);
    println!("Max size of a i32 : {}",i32::MAX);
    println!("Max size of a i64 : {}",i64::MAX);

    //Floats - f32 , f64
    println!("Max size of a f32 : {}",f32::MAX);
    println!("Max size of a f64 : {}",f64::MAX);

    //Boolean - True / False
    //Character - char - 4 bytes


    // String Variables
    let hello: &str = "Hello , World!"; // Immutable variable
    println!("Length of the String is :: {}" , hello.len());

    let mut hey : &str = "Hey there , how are you !";
    println!("{}" , hey);
    hey = "Are you there ? ";
    println!("{}",hey);
    
    //Interger Variables 
    let x: i32 = 22;
    let y: i32 = 7;
    println!("The sum of {} and {} is :: {} " , x , y , x+y);
    
    println!("{}" , NUMBER);

    //Scope and Shadowing
    let a: i32 = 5;
    {
    let b: i32 = 6;
    println!("Math : Sun of {} and {} is :: {} ",a,b,a+b);
    }
    /*println!("Math : Sun of {} and {} is :: {} ",a,b,a+b);
     * As b is out of the scope it's in another scope
     */
    
    // Shadowing
    {
        let a: i32 = 10;
        println!("This is the overwritten value of a in a local function to main function : {} " , a);
    }
    println!("The value a of to the main function is :: {} " , a);

    // Trying with another example to understand shadowing
    println!("Another example to understand the shadowing");
    let m: i32 = 20;
    println!("The value of m at this point is : {} " , m);
    let m: &str = "Hello";
    println!("The value of m at this point is : {} " , m);


    //Suffix and Underscore
    //Suffix - specify the type of a numeric literal
    let ab: u32 = 23_u32;
    println!("{}" , ab);
    let bc: i64 = 1_00_000__0000;
    println!("{}",bc);


    //solving a question

    let abc: i32 = 22;
    let xyz: i32 = 7;
    let res: f64 = abc as f64 / xyz as f64;
    println!("{}" , res);

    //******************** Challenge 1 *********************
    let my_name: &str = "Satish Yadav";
    let my_birthday: &str = "26th August";
    let age: i32 = 21;
    let new_age: i32 = age+BIRTHDAY;
    println!("My Name is {} and I am {} years old , and i will turn {} on {}",my_name,age,new_age,my_birthday);
    
    //Working with tuples
    let t: (&str , i32 , f64) = ("Satish" ,32 , 3.14);
    println!("{:?}",t);
    
    //Woring with arrays 
    let array: [i32;5] = [23,54,56,89,32];
    println!("The first number is :: {}",array[0]);
    println!("The second number is :: {}",array[1]);
    println!("The third number is :: {}",array[2]);


    //Slicing
    let arr: [i32;7] = [2,3,4,5,6,7,8];
    let slice: &[i32] = &arr[1..3];
    println!("{:?}",slice);
}
