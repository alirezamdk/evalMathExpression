



pub mod parser
{
    use std::{collections::VecDeque};
    use crate::general::tokens::*;
    use Term::*;
    /// The function begins by initializing a "str_number" string and an "operator" variable. 
    /// It then reads the expression one character at a time. If a character is numeric, 
    /// it is added to the "str_number" string. If the character is an operator, the function 
    /// uses the "get_operator" function to determine the operator, and then adds it to a stack. 
    /// If the operator is a left bracket, a "neg" flag is set to true. If the operator is a 
    /// negative sign and the "neg" flag is not set, the function pushes the "neg" operator 
    /// onto the stack. If the "str_number" string is not empty, the number is converted to 
    /// a float and added to the stack. If the operator is a right bracket, the "neg" flag 
    /// is set to true. If the operator is unknown, a count is incremented. Finally, if there 
    /// is a number left in the "str_number" string, it is converted to a float and added to 
    /// the stack, and the stack is returned. 
    /// 
    /// # Example
    /// ```
    /// use crate::parse::parser;
    /// use crate::general::tokens::Operator::*;
    /// use Term::{Oprand, Opratr};
    /// 
    /// let parser = parser::parse("-1+3");
    /// assert_eq!(parser, vec![Opratr(Neg), Oprand(1.0), Opratr(Add), Oprand(3.0)]);
    /// 
    /// ```

    pub fn parse(expr: &str) -> VecDeque<Term>
    {
        // this stack store parsed math expression
        // to store numbers
        let mut str_number = String::new();
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
            println!("sc: {chars}");

            // get number 
            if chars.is_numeric() || chars == '.' 
            {
                // after number we dont have a neg sign
                neg = false;
                // add numbers to this string
                str_number += &chars.to_string();
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
        
        parse
    }

    #[inline]
    fn str_to_f64(input: &str) -> NumsType
    {
        input.parse::<NumsType>()
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
            "log10" => { Functs(Function::Log10)}
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
        crate::parse::parser::parse(&$ex.to_ascii_lowercase())
    };
}



#[cfg(test)]
mod test_parse
{

    use crate::general::tokens::*;
    use Term::{Oprand, Opratr, Bracts, Functs, Constn};
    use Operator::*;
    use Brackets::*;
    use Function::*;
    use Constant::*;

    // use super::parse::*;

    #[test]
    fn parse_test_case()
    {
        let expr1 = parse!("1+3.14");
        let expr2 = parse!("(log(5!))+pi/2^%++");
        let expr3 = parse!("54635.4354325-86554654.547547+765766578657656");
        let expr4 = parse!("ceil floor () []^e Ln      8.0001+7");
        let expr5 = parse!("arcsinh floor ceil phi[]()+0.9+5!!++arccotanh(pi*7)");


        assert_eq!(expr1,   [Oprand(1.0), Opratr(Add), Oprand(3.14)]);

        assert_eq!(expr2,   [Bracts(Oparantes), Functs(Log), Bracts(Oparantes),
                            Oprand(5.0), Functs(Fact), Bracts(Cparantes), 
                            Bracts(Cparantes), Opratr(Add), Constn(Pi), 
                            Opratr(Div), Oprand(2.0), Opratr(Pow), 
                            Opratr(Module), Opratr(Add), Opratr(Add)]);

        assert_eq!(expr3,   [Oprand(54635.4354325), Opratr(Sub), Oprand(86554654.547547),
                            Opratr(Add), Oprand(765766578657656.0)]);

        assert_eq!(expr4,   [Functs(Ceil), Functs(Floor), 
                            Bracts(Oparantes), Bracts(Cparantes), 
                            Bracts(Obracket), Bracts(Cbracket), 
                            Opratr(Pow), Constn(E), Functs(Ln),
                            Oprand(8.0001), Opratr(Add), Oprand(7.0)]);

        assert_eq!(expr5,   [Functs(Arcsinh), Functs(Floor), Functs(Ceil),
                            Constn(Phi), Bracts(Obracket), Bracts(Cbracket), 
                            Bracts(Oparantes), Bracts(Cparantes), Opratr(Add), 
                            Oprand(0.9), Opratr(Add), Oprand(5.0), Functs(Fact), 
                            Functs(Fact), Opratr(Add), Opratr(Add), 
                            Functs(Arccotanh), Bracts(Oparantes), Constn(Pi), 
                            Opratr(Mul), Oprand(7.0), Bracts(Cparantes)]);
                            
    }

    #[test]
    fn parse_unary_test()
    {
        let expr1 = parse!("-----9+--3+-2");
        let expr2 = parse!("-(-1+0)--(12-8)-8--21");
        let expr3 = parse!("(7/8*-1)-2*-9");
        let expr4 = parse!("12-(12-1)--(19+20)---(11*2-(11+54+(-9-1))");
        
        
        assert_eq!(expr1,   [Opratr(Neg), Opratr(Neg), Opratr(Neg), 
                            Opratr(Neg), Opratr(Neg), Oprand(9.0), 
                            Opratr(Add), Opratr(Neg), Opratr(Neg), 
                            Oprand(3.0), Opratr(Add), Opratr(Neg), 
                            Oprand(2.0)]);

        assert_eq!(expr2,   [Opratr(Neg), Bracts(Oparantes), Opratr(Neg), 
                            Oprand(1.0), Opratr(Add), Oprand(0.0), 
                            Bracts(Cparantes), Opratr(Sub), Opratr(Neg), 
                            Bracts(Oparantes), Oprand(12.0), Opratr(Sub), 
                            Oprand(8.0), Bracts(Cparantes), Opratr(Sub), 
                            Oprand(8.0), Opratr(Sub), Opratr(Neg), 
                            Oprand(21.0)]);

        assert_eq!(expr3,   [Bracts(Oparantes), Oprand(7.0), Opratr(Div), 
                            Oprand(8.0), Opratr(Mul), Opratr(Neg), 
                            Oprand(1.0), Bracts(Cparantes), Opratr(Sub), 
                            Oprand(2.0), Opratr(Mul), Opratr(Neg), Oprand(9.0)]);

        assert_eq!(expr4,   [Oprand(12.0), Opratr(Sub), Bracts(Oparantes), 
                            Oprand(12.0), Opratr(Sub), Oprand(1.0), 
                            Bracts(Cparantes), Opratr(Sub), Opratr(Neg), 
                            Bracts(Oparantes), Oprand(19.0), Opratr(Add), 
                            Oprand(20.0), Bracts(Cparantes), Opratr(Sub), 
                            Opratr(Neg), Opratr(Neg), Bracts(Oparantes), 
                            Oprand(11.0), Opratr(Mul), Oprand(2.0), 
                            Opratr(Sub), Bracts(Oparantes), Oprand(11.0), 
                            Opratr(Add), Oprand(54.0), Opratr(Add), 
                            Bracts(Oparantes), Opratr(Neg), Oprand(9.0), 
                            Opratr(Sub), Oprand(1.0), Bracts(Cparantes), 
                            Bracts(Cparantes)]);

    }
}