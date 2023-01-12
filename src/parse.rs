use std::{collections::VecDeque, fmt::Display};
use crate::general::terms::*;
use Term::*;


pub struct Parser<'a>
{
    expr: &'a str,
    parse: VecDeque<Term>
}

impl<'a> Parser<'a>
{
    pub fn new(expr: &'a str) -> Self
    {
        Self { expr: Box::leak(Box::new(expr.to_ascii_lowercase())), parse: VecDeque::new()}
    }

    pub fn parse(&mut self) -> VecDeque<Term>
    {
        // this stack store parsed math expression
        // let mut self.parse: VecDeque<Term> = VecDeque::new();
        // to store numbers
        let mut str_number = String::new();

        let mut operator: Term;
        let mut count: usize = 0;

        // this flag helps to find negative unraies in expression 
        let mut neg = false;

        // read string char by char 
        while let Some(chars) = self.expr.chars().nth(count)
        {
            println!("sc: {chars}");
            operator = Self::get_operator(&self.expr, &mut count);

            // get number 
            if chars.is_numeric() || chars == '.' 
            {
                neg = true;
                // add numbers to this string
                str_number += &chars.to_string();
                count += 1;
                continue;
            }

            if operator == Bracts(Brackets::Oparantes)
            {
                neg = true;
            }
            if operator == Opratr(Operator::Sub) && !neg
            {
                self.parse.push_back(Opratr(Operator::Neg));
                continue;
            }
            // push oprand to result stack
            if !str_number.is_empty() 
            {
                let oprand = Self::str_to_f64(&str_number);
                self.parse.push_back(Term::Oprand(oprand));
                str_number.clear();
            }
            if operator != Opratr(Operator::Unknown)
            {
                neg = false;
                self.parse.push_back(operator)
            }
            if operator == Bracts(Brackets::Cparantes)
            {
                neg = true;
            }
            if operator == Opratr(Operator::Unknown)
                { count += 1; }
        }
        // input last number if exist
        if !str_number.is_empty()
        {
            self.parse.push_back(Term::Oprand(Self::str_to_f64(&str_number)));
        }
        
        self.parse.clone()
    }

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

    fn get_operator(expr: &str, pos: &mut usize) -> Term
    {
        let mut res: Term;
        let mut out: Term = Opratr(Operator::Unknown);
        let len = expr.chars().count();
        let mut temp: usize = 0;

        for i in 1 ..= len - *pos 
        {
            if i > 9 || (*pos + i) > len
                { break }

            res = Self::operator(&expr[*pos..*pos + i]);

            if res != Opratr(Operator::Unknown)
            {
                out = res;
                temp = i;
            }
        }

        *pos += temp;
        // println!("op: {:?}", out);
        out

    }
}


impl<'a> Display for Parser<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{:?}", self.parse)
    }
}


#[macro_export]
macro_rules! parse 
{
    ($ex: expr) => 
    {
        crate::Parser::new($ex).parse()
    };
}



#[cfg(test)]
mod test_parse
{

    use crate::general::terms::*;
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
        let expr5 = parse!("arcsinhfloorceilphi[]()+0.9+5!!++arccotanh(pi*7)");


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