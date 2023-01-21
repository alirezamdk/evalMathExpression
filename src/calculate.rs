use std::{collections::VecDeque, fmt::Error};
use crate::general::{tokens::*, mathfunc::math};


static mut ANGEL_MODE: AngelMode = AngelMode::Radians;

#[derive(Clone, Copy)]
pub enum AngelMode
{
    Degree,     // 0 - 360
    Radians,    // 0 - 2pi - default
    Gradians,   // 0 - 500
}


pub enum Calculatealgorithm
{
    DirectAlgebraic,     // Direct Algebraic Logic
    ShuntingYard,        // Shunting Yard algorithm
    ExpressionTree,      // Expression tree
}



pub fn calculate(expr: Result<VecDeque<Term>, String>, calc_algorithm: Calculatealgorithm) -> Result<NumsType, String>
{
    match expr 
    {
        Ok(expr) => match calc_algorithm
        {
            Calculatealgorithm::DirectAlgebraic => todo!(),
            Calculatealgorithm::ShuntingYard => Ok(calc_postfix(&infix_to_postfix(&expr))),
            Calculatealgorithm::ExpressionTree => todo!(),
        },
        Err(e) => Err(e),
    }
}

// Shunting Yard
fn priority(term: Term) -> i8
{
    match term 
    {
        Term::Bracts(brackets) => 
        {
            match brackets 
            {
                Brackets::Oparantes |
                Brackets::Cparantes => -1,

                Brackets::Obracket  |
                Brackets::Cbracket => 4,

                _ => 0
            }
        },
        Term::Opratr(operator) => 
        {
            match operator 
            {
                Operator::Add |
                Operator::Sub => 1,

                Operator::Div |
                Operator::Mul => 2,

                Operator::Module |
                Operator::Pow => 3,

                Operator::Unknown => unreachable!(),
                Operator::Neg => 4,
            }
        },
        Term::Oprand(_) => unreachable!(),

        Term::Functs(_) => 5,
        Term::Constn(_) => 0,
    }
}

pub fn infix_to_postfix(expr: &VecDeque<Term>) -> VecDeque<Term>
{
    let mut postfix_stack:   VecDeque<Term> = VecDeque::new();
    let mut opr_stack:      VecDeque<Term> = VecDeque::new();
    
    for term in expr
    {
        match term 
        {
            Term::Oprand(num)                   => postfix_stack.push_back(Term::Oprand(*num)),
            Term::Constn(constn)                => postfix_stack.push_back(Term::Constn(*constn)),
            Term::Bracts(Brackets::Oparantes)   => opr_stack.push_back(*term),
            Term::Bracts(Brackets::Cparantes)   => 
            {
                while !(Some(Term::Bracts(Brackets::Oparantes)) == opr_stack.back().copied()
                    || Some(Term::Bracts(Brackets::Obracket)) == opr_stack.back().copied())
                {
                    let operator= opr_stack.pop_back();
                    if operator.is_none()
                        { break }
                    postfix_stack.push_back(operator.unwrap());
                }
                if !opr_stack.is_empty()
                    { opr_stack.pop_back().unwrap(); }
            },
            Term::Opratr(Operator::Neg)          => 
            {
                if opr_stack.is_empty()
                    { opr_stack.push_front(Term::Opratr(Operator::Neg)) }
                else 
                    { opr_stack.push_back(Term::Opratr(Operator::Neg)) }
            }
            _                                    => 
            {
                while !opr_stack.is_empty() 
                    && priority(*term) <= priority(opr_stack.back().copied().unwrap())
                {
                    let operator = opr_stack.pop_back().unwrap();
                    postfix_stack.push_back(operator);
                }
                opr_stack.push_back(*term);
            }
        }
        // println!("Tpos_stack: {:?}", postfix_stack);
        // println!("Zopr_stack: {:?}\n", opr_stack);
    }
    while !opr_stack.is_empty()
    {
        // println!("l_po_stack: {:?}", postfix_stack);
        let operator = opr_stack.pop_back().unwrap();
        postfix_stack.push_back(operator);
    }
    postfix_stack
}

pub fn infix_to_perfix(expr: &VecDeque<Term>) -> VecDeque<Term>
{
    let mut expr = expr.clone();
    let len = expr.len();
    let mut expr_stack_rev : VecDeque<Term> = VecDeque::new();
    for _ in 0..len
    {
        expr_stack_rev.push_front(expr.pop_front().unwrap());
    }
    let mut expr_stack_rev = infix_to_postfix(&expr_stack_rev);

    let mut perfix_stack : VecDeque<Term> = VecDeque::new();
    for _ in 0..len
    {
        if !expr_stack_rev.is_empty()
            { perfix_stack.push_front(expr_stack_rev.pop_front().unwrap()); }
    }
    perfix_stack
}

fn pop_2_back(res_stack: &mut VecDeque<f64>) -> (NumsType, NumsType)
{
    (pop_back(res_stack)
    ,pop_back(res_stack))
}

fn pop_back(res_stack: &mut VecDeque<f64>) -> NumsType
{
    res_stack.pop_back().expect("experssion problem")
}

pub fn calc_postfix(expr: &VecDeque<Term>) -> NumsType
{
    let mut res_stack: VecDeque<NumsType> = VecDeque::new();
    let mut term_num: usize = 0;

    while let Some(term) = expr.get(term_num)
    {
        match term
        {
            Term::Oprand(num) => res_stack.push_back(*num),
            Term::Opratr(operator) => 
            {
                let result = match operator 
                {
                    Operator::Add => { let (b, a) = pop_2_back(&mut res_stack); a + b }
                    Operator::Sub => { let (b, a) = pop_2_back(&mut res_stack); a - b }
                    Operator::Div => { let (b, a) = pop_2_back(&mut res_stack); a / b }
                    Operator::Mul => { let (b, a) = pop_2_back(&mut res_stack); a * b }
                    Operator::Pow => { let (b, a) = pop_2_back(&mut res_stack); math::power(a, b) }
                    Operator::Module => { let (b, a) = pop_2_back(&mut res_stack); a % b }
                    Operator::Neg => { let b = pop_back(&mut res_stack); -b},
                    Operator::Unknown => 0.0,
                };
                
                res_stack.push_back(result);
            },
            Term::Functs(func) => 
            {
                let result = match func
                {
                    Function::Fact => 
                    {
                        let b = pop_back(&mut res_stack);
                        math::fact(b)
                    }
                    Function::Sqrt => 
                    {
                        let b = pop_back(&mut res_stack);
                        math::sqrt(b)
                    }
                    Function::Root => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::root(a, b)
                    }
                    Function::Pnr => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::pnr(a, b)
                    }
                    Function::Cnr => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::cnr(a, b)
                    }
                    Function::Log => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::log(a, b)
                    }
                    Function::Ln => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::ln(a)
                    }
                    Function::Floor => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::floor(a)
                    }
                    Function::Ceil => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::ceil(a)
                    }
                    Function::Round => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::round(a)
                    }
                    Function::Abs => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::abs(a)
                    }
                    
                    Function::Sin => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::sin(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cos => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cos(a, unsafe { ANGEL_MODE })
                    },
                    Function::Tan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::tan(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cotan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cotan(a, unsafe { ANGEL_MODE })
                    },

                    Function::Arcsin =>
                    {
                        let a = pop_back(&mut res_stack);
                        math::arcsin(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccos => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccos(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arctan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arctan(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccotan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccotan(a, unsafe { ANGEL_MODE })
                    },

                    Function::Sinh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::sinh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cosh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cosh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Tanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::tanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cotanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cotanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arcsinh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arcsinh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccosh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccosh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arctanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arctanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccotanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccotanh(a, unsafe { ANGEL_MODE })
                    },

                    Function::Max =>
                    {
                        math::max(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Min =>
                    {
                        math::min(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Avg =>
                    {
                        math::avg(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Var => 
                    {
                        math::var(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Dev =>
                    {
                        math::dev(expr, &mut term_num, &mut res_stack)
                    }

                    Function::Bin => todo!(),
                    Function::Oct => todo!(),
                    Function::Dec => todo!(),
                    Function::Hex => todo!(),
                    Function::Tobase => todo!(),
                };
                res_stack.push_back(result);
            },
            Term::Constn(constant) => 
            {   
                let constant = match constant 
                {
                    Constant::Pi => std::f64::consts::PI,
                    Constant::E => std::f64::consts::E,
                    Constant::Phi => 1.61,
                };

                res_stack.push_back(constant);
            },
            Term::Bracts(_) => {},
        }

        term_num += 1;
        // println!("term:   {:?}", term);
        // println!("stack:  {:?}\n", res_stack);
    }
    *res_stack.front().unwrap()
}


// Expression tree
fn infix_to_tree()
{}

fn calc_tree()
{}

// Direct Algebraic Logic
fn infix_add_parantes()
{}

fn calc_infix()
{}

#[macro_export]
macro_rules! calculate
{
    ($expression: expr) => 
    {
        crate::calculate::calculate(
            &crate::evalmath::parse::parser::parse(&$expression.to_ascii_lowercase()), 
            $crate::calculate::Calculatealgorithm::ShuntingYard)
    };
    ($expression: expr, $algorithm: ident) => 
    {
        crate::calculate::calculate(
            crate::evalmath::parse::parser::parse(&$expression.to_ascii_lowercase()), 
            $algorithm)
    };
}




#[cfg(test)]
mod postfix
{

    use std::collections::VecDeque;
    use Term::{Oprand, Opratr, Bracts, Functs, Constn};
    use Operator::*;
    use Brackets::*;
    use Constant::*;
    use Function::*;
    use crate::parse;

    use super::*;
    use crate::calculate::Calculatealgorithm::*;

    #[test]
    fn test_infix_to_postfix()
    {
        // 1+1
        let expr1: VecDeque<Term> 
        = VecDeque::from(   [Term::Oprand(1.0), Opratr(Add), Oprand(1.0)]);

        // 12-(33+44)
        let expr2: VecDeque<Term> 
        = VecDeque::from(   [Oprand(12.0), Opratr(Sub), Bracts(Oparantes), 
                            Oprand(33.0), Opratr(Add), Oprand(44.0), Bracts(Cparantes)]);

        // 1+2-3
        let expr3: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Add), Oprand(2.0),
                            Opratr(Sub), Oprand(3.0)]);
        // 1/2*3*5
        let expr4: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Div), Oprand(2.0),
                            Opratr(Mul), Oprand(3.0), Opratr(Mul), Oprand(5.0)]);

        // 4+7*9/3+6
        let expr5: VecDeque<Term> 
        = VecDeque::from(   [Oprand(4.0), Opratr(Add), Oprand(7.0), 
                            Opratr(Mul), Oprand(9.0), Opratr(Div),
                            Oprand(3.0), Opratr(Add), Oprand(6.0)]);

        // 1+2-2*7^3/4-5
        let expr6: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Add), Oprand(2.0), 
                            Opratr(Sub), Oprand(2.0), Opratr(Mul), 
                            Oprand(7.0), Opratr(Pow), Oprand(3.0), 
                            Opratr(Div), Oprand(4.0), Opratr(Sub), 
                            Oprand(5.0)]);

        // ((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3
        let expr7: VecDeque<Term> 
        = VecDeque::from(   [Bracts(Oparantes), Bracts(Oparantes), Bracts(Oparantes), 
                            Bracts(Oparantes), Oprand(8.0), Opratr(Sub), Oprand(0.0), 
                            Bracts(Cparantes), Opratr(Div), Bracts(Oparantes), 
                            Oprand(5.0), Opratr(Add), Oprand(5.0), Opratr(Mul), 
                            Oprand(6.0), Bracts(Cparantes), Bracts(Cparantes), 
                            Opratr(Pow), Oprand(5.0), Opratr(Add), Oprand(7.0), 
                            Opratr(Add), Bracts(Oparantes), Bracts(Oparantes), 
                            Bracts(Oparantes), Oprand(2.0), Opratr(Sub), Oprand(3.0), 
                            Bracts(Cparantes), Opratr(Mul), Oprand(4.0), 
                            Bracts(Cparantes), Opratr(Div), Oprand(4.0), 
                            Opratr(Pow), Bracts(Oparantes), Oprand(2.0), 
                            Opratr(Sub), Oprand(0.0), Bracts(Cparantes), 
                            Opratr(Pow), Oprand(2.0), Bracts(Cparantes), 
                            Opratr(Sub), Oprand(4.0), Bracts(Cparantes), 
                            Opratr(Add), Oprand(2.0), Opratr(Mul), Oprand(4.0), 
                            Bracts(Cparantes), Opratr(Pow), Oprand(3.0)]);



        assert_eq!(infix_to_postfix(&expr1),  [Oprand(1.0), Oprand(1.0), Opratr(Add)]);

        assert_eq!(infix_to_postfix(&expr2),  [Oprand(12.0), Oprand(33.0), Oprand(44.0), 
                                            Opratr(Add), Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr3),  [Oprand(1.0), Oprand(2.0), Opratr(Add),
                                            Oprand(3.0), Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr4),  [Oprand(1.0), Oprand(2.0), Opratr(Div),
                                            Oprand(3.0), Opratr(Mul), Oprand(5.0), Opratr(Mul)]);

        assert_eq!(infix_to_postfix(&expr5),  [Oprand(4.0), Oprand(7.0), Oprand(9.0),
                                            Opratr(Mul), Oprand(3.0), Opratr(Div),
                                            Opratr(Add), Oprand(6.0), Opratr(Add)]);

        assert_eq!(infix_to_postfix(&expr6),    [Oprand(1.0), Oprand(2.0), Opratr(Add), 
                                                Oprand(2.0), Oprand(7.0), Oprand(3.0), 
                                                Opratr(Pow), Opratr(Mul), Oprand(4.0), 
                                                Opratr(Div), Opratr(Sub), Oprand(5.0), 
                                                Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr7),  [Oprand(8.0), Oprand(0.0), Opratr(Sub), 
                                            Oprand(5.0), Oprand(5.0), Oprand(6.0), 
                                            Opratr(Mul), Opratr(Add), Opratr(Div), 
                                            Oprand(5.0), Opratr(Pow), Oprand(7.0), 
                                            Opratr(Add), Oprand(2.0), Oprand(3.0), 
                                            Opratr(Sub), Oprand(4.0), Opratr(Mul),
                                            Oprand(4.0), Oprand(2.0), Oprand(0.0), 
                                            Opratr(Sub), Opratr(Pow), Oprand(2.0),
                                            Opratr(Pow), Opratr(Div), Opratr(Add), 
                                            Oprand(4.0), Opratr(Sub), Oprand(2.0), 
                                            Oprand(4.0), Opratr(Mul), Opratr(Add), 
                                            Oprand(3.0), Opratr(Pow)]);
    }

    #[test]
    fn test_calculate_postfix()
    {
        // ((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3
        let expr1 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(8.0), Oprand(0.0), Opratr(Sub), Oprand(5.0), 
                            Oprand(5.0), Oprand(6.0), Opratr(Mul), Opratr(Add), 
                            Opratr(Div), Oprand(5.0), Opratr(Pow), Oprand(7.0), 
                            Opratr(Add), Oprand(2.0), Oprand(3.0), Opratr(Sub), 
                            Oprand(4.0), Opratr(Mul), Oprand(4.0), Oprand(2.0), 
                            Oprand(0.0), Opratr(Sub), Opratr(Pow), Oprand(2.0), 
                            Opratr(Pow), Opratr(Div), Opratr(Add), Oprand(4.0), 
                            Opratr(Sub), Oprand(2.0), Oprand(4.0), Opratr(Mul), 
                            Opratr(Add), Oprand(3.0), Opratr(Pow)]);
        
        let expr2 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(5.0), Functs(Fact), Oprand(5.0), 
                            Oprand(2.0), Opratr(Add), Oprand(3.0), 
                            Opratr(Add), Functs(Sqrt), Opratr(Add), 
                            Constn(Pi), Opratr(Add)]);

        let expr3 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(2.0), Constn(E), Opratr(Pow), 
                            Oprand(0.0), Functs(Fact), Opratr(Pow), 
                            Oprand(5.0), Opratr(Add)]);

        // 7-4+2*4/9+3-3
        let expr4 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(7.0), Oprand(4.0), Opratr(Sub), 
                            Oprand(2.0), Oprand(4.0), Opratr(Mul), 
                            Oprand(9.0), Opratr(Div), Opratr(Add), 
                            Oprand(3.0), Opratr(Add), Oprand(3.0), 
                            Opratr(Sub)]);

        assert_eq!(calc_postfix(&expr1), 1325.5620206713477_f64);
        assert_eq!(calc_postfix(&expr2), 126.30387031375818_f64);
        assert_eq!(calc_postfix(&expr3), 11.58088599101792_f64);
        assert!((calc_postfix(&expr4) - 3.88888 < 0.00001));
    }

    #[test]
    fn test_calculate_postfix_static_func()
    {
        let expr1 = parse!("max(-12,2)");
        let expr2 = parse!("max(1+2, 2)");
        let expr3 = parse!("max(12,2)+2");
        let expr4 = parse!("3+max(12,2)");
        let expr5 = parse!("max(max(4,6,14), avg(3,4,5), min(2,99))");
        let expr6 = parse!("(1+(2+(3+(max(1,2,3,4))))");
        let expr7 = parse!("-max(12,2)");
        let expr8 = parse!("1+max(5!,((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3,9^9,0)");

        assert_eq!(calculate(expr1, ShuntingYard).unwrap(), 2.0);
        assert_eq!(calculate(expr2, ShuntingYard).unwrap(), 3.0);
        assert_eq!(calculate(expr3, ShuntingYard).unwrap(), 14.0);
        assert_eq!(calculate(expr4, ShuntingYard).unwrap(), 15.0);
        assert_eq!(calculate(expr5, ShuntingYard).unwrap(), 14.0);
        assert_eq!(calculate(expr6, ShuntingYard).unwrap(), 10.0);
        assert_eq!(calculate(expr7, ShuntingYard).unwrap(), -12.0);
        assert_eq!(calculate(expr8, ShuntingYard).unwrap(), 387420490.0);
    }

    #[test]
    fn test_calculate_postfix_math_func()
    {
        
    }
}
