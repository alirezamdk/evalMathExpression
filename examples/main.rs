use evalmath::{calculate::{self, infix_to_postfix}, parse};
use parse::parser::*;

fn main() 
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let res = calculate::calc_postfix(&infix_to_postfix(&parse!(&input)));
    println!("Input:  {}\nResult {}", input, res);
}