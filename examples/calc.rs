use std::{env, io::Write};
use clap::*;

extern crate evalmath;
use evalmath::calculate;
use calculate::{*, Calculatealgorithm::*};


fn main() 
{
    let matches = App::new("eval math expr")
                    .arg(Arg::with_name("expr")
                            .help("expression"))

                    .arg(Arg::with_name("more-precision")
                            .short("p")
                            .long("more-precision")
                            .help("output result with more precision"))

                    .arg(Arg::with_name("set-angle-mode")
                            .long("set-angle-mode")
                            .takes_value(true)
                            .default_value("radians")
                            .possible_values(&["degree", "gradian", "radians"])
                            .help("Set the angle mode"))

                    .arg(Arg::with_name("set-base")
                            .short("b")
                            .long("set-base")
                            .takes_value(true)
                            .default_value("10")
                            .help("Set the base"))

                    .arg(Arg::with_name("set-algorithm")
                            .long("set-algorithm")
                            .takes_value(true)
                            .default_value("ShufligYard")
                            .help("Set the algorithm"))

                    .arg(Arg::with_name("to-postfix")
                            .long("to-postfix")
                            .takes_value(true)
                            .help("conver exprisson to postfix"))

                    .arg(Arg::with_name("to-perfix")
                            .long("to-perfix")
                            .help("conver exprisson to perfix"))

                    .arg(Arg::with_name("to-tree")
                            .long("to-tree")
                            .help("conver exprisson to tree"))
                    .get_matches();
    

    let input = matches.value_of("expr");


    if input.is_none()
    {
        let mut input_stream = String::new();
        // flags:
        let to_tree: bool;
        let to_postfix: bool;
        let to_perfix: bool;
        let round_dec: bool;

        // options
        // let base;
        // let angle;

        while input_stream.trim() != "quit" 
        {
            print!(">>> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input_stream).unwrap();
            let res = calculate!(&input_stream, ShuntingYard);
            match res 
            {
                Ok(res) => println!(">>> {} = {}",input_stream.trim(), res),
                Err(e) => println!(">>> Error: {}", e),
            }
            input_stream.clear();
        }
    }
    else
    {
        let input = input.unwrap();

        let more_precision = matches.is_present("more-precision");
        let angle_mode = matches.value_of("set-angle-mode").unwrap_or("radian");
        let base = matches.value_of("set-base").unwrap_or("10");
        
        let result = calculate!(input);

        if !more_precision
        {
            let rounded_result = round_to_decimal(result.unwrap());
            println!("Rounded result: {}", rounded_result);
        } else 
        {
            println!("result: {:?}", result);
        }
    }
}