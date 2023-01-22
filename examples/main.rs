extern crate evalmath;
use evalmath::calculate;
use calculate::{*, Calculatealgorithm::*};

fn main() 
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let res = calculate!(&input, ShuntingYard);
    match res 
    {
        Ok(res) => println!("{} = {}",input, res),
        Err(e) => println!("Error: {}", e),
    }
}