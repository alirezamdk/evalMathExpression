



pub mod parser
{
    use std::{collections::VecDeque};
    use crate::general::tokens::*;
    use Term::*;
    
    /// This function takes a string 'expr' and returns a VecDeque of Terms.The purpose 
    /// of this function is to parse the input mathematical expression, which is passed 
    /// as string, into a series of terms that can be used for further processing or e-
    /// valuation.
    ///
    /// # Example
    /// ```
    /// use crate::evalmath::parse::parser;
    /// use evalmath::general::tokens::{Operator::*, Term::{Oprand, Opratr}};
    /// 
    /// let parser = parser::parse("-1+3");
    /// assert_eq!(parser.unwrap(), vec![Opratr(Neg), Oprand(1.0), Opratr(Add), Oprand(3.0)]);
    /// 
    /// ```

    pub fn parse(expr: &str) -> Result<VecDeque<Term>, String>
    {
        if expr.trim().is_empty()
        {
            return Err("input is empty".to_string());
        }
        // this stack store parsed math expression
        // to store numbers
        let mut str_number: Vec<char> = Vec::new();
        let mut parse: VecDeque<Term> = VecDeque::new();
        let mut operator: Term;
        let mut count: usize = 0;

        // this flag helps to find negative unary in expression 
        // true means we mey have a negative sign and false means 
        // the sub operator absolutely is subtraction sign. 
        let mut neg = true;

        // read string char by char 
        while let Some(chars) = expr.chars().nth(count)
        {
            // println!("sc: {chars}");

            // get number 
            if chars.is_numeric() || chars == '.' 
            {
                // after number we dont have a neg sign
                neg = false;
                // add numbers to this string
                str_number.push(chars);
                count += 1;
                continue;
            }
            
            // push oprand to result stack
            if !str_number.is_empty() 
            {
                let oprand = str_to_f64(&str_number);
                parse.push_back(Term::Oprand(oprand));
                str_number.clear();
            }
            
            operator = get_operator(&expr, &mut count);

            match operator 
            {
                Opratr(Operator::Sub) if neg => 
                {
                    parse.push_back(Opratr(Operator::Neg));
                    continue;
                },
                Opratr(Operator::Unknown) => 
                {
                    count += 1;
                }
                _ => 
                {
                    // after any operators we may have a negative sign, so it sets neg flag to true
                    neg = true;
                    parse.push_back(operator);
                    // but after close bracket we absolutely have a sub sign, set neg flag to false
                    if let Bracts(Brackets::Cparantes) = operator
                    {
                        neg = false;
                    }
                },
            }
        }
        // input last number if exist
        if !str_number.is_empty()
        {
            parse.push_back(Term::Oprand(str_to_f64(&str_number)));
        }
        
        Ok(parse)
    }

    #[inline]
    fn str_to_f64(input: &Vec<char>) -> NumsType
    {
        String::from_iter(input).parse::<NumsType>()
            .expect("str to f64 problem")
    }

    fn operator(op_str: &str) -> Term
    {
        match op_str 
        {
            "+" => { Opratr(Operator::Add) }
            "-" => { Opratr(Operator::Sub) }
            "/" => { Opratr(Operator::Div) }
            "*" => { Opratr(Operator::Mul) }
            "^" => { Opratr(Operator::Pow) }
            "%" => { Opratr(Operator::Module) }
            "!" => { Functs(Function::Fact) }
            
            "," => { Bracts(Brackets::Colon) }
            
            "<" => { Bracts(Brackets::Oangel) }
            "(" => { Bracts(Brackets::Oparantes) }
            ")" => { Bracts(Brackets::Cparantes) }
            ">" => { Bracts(Brackets::Cangel) }
            "[" => { Bracts(Brackets::Obracket) }
            "]" => { Bracts(Brackets::Cbracket) }

            "e" => { Constn(Constant::E) }
            "pi" => { Constn(Constant::Pi) }
            "phi" => { Constn(Constant::Phi)}

            "ln" => { Functs(Function::Ln) }
            "log" => { Functs(Function::Log) }
            "perm" => { Functs(Function::Pnr) }, // Permutation
            "comp" => { Functs(Function::Cnr) }, // Composition
            "round" => { Functs(Function::Round) }
            "abs" => { Functs(Function::Abs) }
            "ceil" => { Functs(Function::Ceil) }
            "sqrt" => { Functs(Function::Sqrt) }
            "root" => { Functs(Function::Root) }
            "floor" => { Functs(Function::Floor) }

            "sin" => { Functs(Function::Sin) },
            "cos" => { Functs(Function::Cos) },
            "tan" => { Functs(Function::Tan) },
            "cotan" => { Functs(Function::Cotan) },

            "arcsin" => { Functs(Function::Arcsin) },
            "arccos" => { Functs(Function::Arccos) },
            "arctan" => { Functs(Function::Arctan) },
            "arccotan" => { Functs(Function::Arccotan) },

            "sinh" => { Functs(Function::Sinh) },
            "cosh" => { Functs(Function::Cosh) },
            "tanh" => { Functs(Function::Tanh) },
            "cotanh" => { Functs(Function::Cotanh) },

            "arcsinh" => { Functs(Function::Arcsinh) },
            "arccosh" => { Functs(Function::Arccosh) },
            "arctanh" => { Functs(Function::Arctanh) },
            "arccotanh" => { Functs(Function::Arccotanh) },

            "max" => { Functs(Function::Max)},
            "min" => { Functs(Function::Min)},
            "avg" => { Functs(Function::Avg)},
            "var" => { Functs(Function::Var)},
            "dev" => { Functs(Function::Dev)}

            "bin" => { Functs(Function::Bin)}, 
            "oct" => { Functs(Function::Oct)},
            "dec" => { Functs(Function::Dec)},
            "hex" => { Functs(Function::Hex)},
            "tobase" => { Functs(Function::Tobase)},

            _ =>    { Opratr(Operator::Unknown) }
        }
    }

    #[inline]
    fn get_operator(expr: &str, pos: &mut usize) -> Term
    {
        let len = expr.chars().count();
        let mut end = *pos;

        while end < len && expr.chars().nth(end).unwrap().is_alphabetic()
        {
            end += 1;
        }
        if end < len && end == *pos
        {
            end += 1;
        }

        // println!("res: {}", &expr[*pos..end]);

        let res = operator(&expr[*pos..end]);

        if end > *pos && res != Opratr(Operator::Unknown)
        {
            *pos = end;
        }

        res
    }
}



#[macro_export]
macro_rules! parse
{
    ($ex: expr) => 
    {
        $crate::parse::parser::parse(&$ex.to_ascii_lowercase())
    };
}


