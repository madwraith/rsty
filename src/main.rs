//comment
///documentation comment and support markdown notation.
/*multi line
comment*/
fn main() {
    println!("hello world");

    let a = 10;
    println!("a is {}", a);

    /*can't do:
    let a2;
    someFunction(a2); error: borrow of possibly-uninitialized variable: `a2`. must me initialized before use
    a2 = 22;*/

    /*can do:
    let a3;
    a3 = 332;
    someFunction(a3);*/

    //b = 20; will error, rust variables are immutable by default. use mut
    let mut b = 11 + 4;
    b = b + b;
    println!("b is {}", b); //{} for where b will go

    //rust defaults i32, i for a signed integer
    //b = -22;

    //255 max with unsigned 8: u8
    let c: u8 = 255; 
    println!("c is {}", c);

    //isize / usize: Depends on platform. It’s typically used for memory indexing or sizes.
    let d: isize = -15;
    println!("d is {}", d);

    /*
    i8, i16, i32, i64, i128 for signed
    u8, u16, u32, u64, u128 for unsigned

    i8: -128 to 127
    i16: -32,768 to 32,767
    i32: -2,147,483,648 to 2,147,483,647
    i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    i128: -170,141,183,460,469,231,731,687,303,715,884,105,728 to 170,141,183,460,469,231,731,687,303,715,884,105,727

    u8: 0 to 255
    u16: 0 to 65,535
    u32: 0 to 4,294,967,295
    u64: 0 to 18,446,744,073,709,551,615
    u128: 0 to 340,282,366,920,938,463,463,374,607,431,768,211,455

    Depends on platform:
    usize:
        32-bit: 0 to 4,294,967,295
        64-bit: 0 to 18,446,744,073,709,551,615
    isize:
        32-bit: -2,147,483,648 to 2,147,483,647
        64-bit: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
     */

}
