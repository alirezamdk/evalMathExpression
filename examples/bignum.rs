extern crate evalmath;
use evalmath::bignum::{self, BigNum};

fn main()
{
    let a = BigNum::new("1.2");
    let b = BigNum::new("22.11");

    println!("{}", a+b);

}