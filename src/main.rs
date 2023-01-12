use std::{collections::VecDeque};
mod parse;
use crate::parse::*;
mod general;
use crate::general::terms::Term;
mod calculate;
use crate::calculate::*;


fn read_input(input: &mut String)
{
    std::io::stdin().read_line(input)
        .expect("input error");
}

fn main()
{
    // this stack store postfix input
    let mut stack_result: VecDeque<Term> = VecDeque::new();

    // input the exprisson 
    let mut input = String::new(); 
    read_input(&mut input);
    
    // standard input 

    stack_result = parse!(&input);

    let inf_to_post = infix_to_postfix(&stack_result);

    println!("\nparse:\t {:?}", stack_result);

    println!("posfix:\t {:?}\n", inf_to_post);
    // println!("perfix:\t {:?}", to_perfix(&stack_result));

    println!("{}", calc_postfix(&inf_to_post));
}

