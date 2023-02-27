# 🔢 &nbsp;&nbsp; Eval Math Expression 

A simple library for evaluating mathematical expressions in Rust.

## 🔧 &nbsp;&nbsp;&nbsp;&nbsp; Installation 

To use this library in your project, you need to add the evalmath source code to your project, and then add extern crate evalmath to the crate root file, and use evalmath::calculate in the code.


## 💻 &nbsp;&nbsp; Usage 
```Rust
extern crate evalmath;
use evalmath::calculate;

fn main() {
    let expr = "-6 + 6 * 6";
    let result = calculate!(expr);
    match result {
        Ok(res) => println!("{} = {}",expr, res),
        Err(e) => println!("Error: {}", e),
    }
}
```

This will output:
`-6 + 6 * 6 = 36` <br>
Note: This example is just a basic example, the library support more advanced mathematical expressions.

Want to see evalmath in action? Simply run `cargo run --example main.rs` in project root

## 📚 &nbsp;&nbsp; Supported Operators and Functions 

- The library supports all common mathematical operators such as `+`, `-`, `*`, `/`, `^`, `!`, etc.
- It also supports a wide range of mathematical functions like `sqrt`, `log`, `ln`, `floor`, `sin`, `cos`, `sinh`, `cosh`, `tan`, `tanh`, `abs`, `max`, `min`, `var`, `round`, etc...
- It also supports conversion functions like `bin`, `hex`, `oct`, etc...

You can find a complete list of supported operators and functions [in the tokens.rs file](https://github.com/alirezamdk/evalMathExpression/blob/main/src/general/tokens.rs), specifically in the TokenType enum.

## 📝 &nbsp;&nbsp; Notation Support 

This library can convert mathematical expressions to a tree, postfix, and prefix notation, and display the results in a user-friendly way.


## 🚫 &nbsp;&nbsp; Limitations 

- Only support f64 numbers
- Only support basic mathematical operations, for example does not support adding functions or variables for now

## 🌳 &nbsp;&nbsp; Algorithms

This library by defualt use the Shunting Yard algorithm for evaluating. Additionally, it also supports expression tree, Direct Algebraic Logic (DAL) algorithm.


## 🤗 &nbsp;&nbsp; Contributions 

Got an idea or a fix? I'm all ears! :) 


## ⚠️ &nbsp;&nbsp; Note 

Please note that this library is in an early stage of development. Use at your own risk.

## 📜 &nbsp;&nbsp; License

This project is licensed under the MIT License
