// ALL ABOUT DATA TYPES AND HOW THE INTEGER BITS WORK
#![allow(unused_variables)]
#[allow(non_upper_case_globals)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
    // by default all objects in rust are immutable(it means cannot be changed) but we want to make it mutable so we have to use one keyword know are MUT
    // the type of mutable objects cannot be changed where as the immutable objects the type can be changed if we redfine it. 
    let mut x  = 4; 
    let mut y = "Hello";
    println!("{}",y);
    // lets create some constants so here the thing being

    /*
        BY DEFAULT ITS i32



        the range goes like if it i8 - > then the number range is 2^8.
        the integer bits that we get 
        i8  - theese are signed integers - > the signed integers 
        i16
        i32
        i64
        i128
    
    
    
    */

    const abhisek:u32 = 10; 
    println!("{}",abhisek);
    


    const SECONDS_IN_MINUTE:i32 =  -10 ; 
    println!("{}", SECONDS_IN_MINUTE) ; 

}
